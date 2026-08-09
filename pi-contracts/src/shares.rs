#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, i128, Map, Symbol, Vec};

const SHARE_STATUS_ACTIVE: Symbol = symbol_short!("ACTIVE");
const SHARE_STATUS_LOCKED: Symbol = symbol_short!("LOCKED");
const SHARE_STATUS_DISTRIBUTED: Symbol = symbol_short!("DIST");

#[contract]
pub struct SharesContract;

#[contractimpl]
impl SharesContract {
    /// إنشاء مجمع استثماري جديد
    pub fn create_pool(
        env: Env,
        name: String,
        description: String,
        total_shares: i128,       // إجمالي الأسهم المطروحة
        price_per_share: i128,    // السعر بـ YER
        lockup_period: u64,       // فترة الحجز (سنوات)
        distribution_after: u64,  // التوزيع بعد (سنوات)
    ) -> u64 {
        let pool_id = env.prng().u64();
        let pool_data = Map::new(&env);
        pool_data.set(Symbol::new(&env, "name"), name);
        pool_data.set(Symbol::new(&env, "description"), description);
        pool_data.set(Symbol::new(&env, "total_shares"), total_shares);
        pool_data.set(Symbol::new(&env, "price_per_share"), price_per_share);
        pool_data.set(Symbol::new(&env, "lockup_period"), lockup_period);
        pool_data.set(Symbol::new(&env, "distribution_after"), distribution_after);
        pool_data.set(Symbol::new(&env, "available_shares"), total_shares);
        pool_data.set(Symbol::new(&env, "status"), SHARE_STATUS_ACTIVE);
        pool_data.set(Symbol::new(&env, "created_at"), env.ledger().timestamp());
        
        env.storage().persistent().set(&pool_id, &pool_data);
        pool_id
    }

    /// شراء أسهم (اكتتاب)
    pub fn buy_shares(
        env: Env,
        pool_id: u64,
        buyer: Address,
        amount: i128,
    ) {
        let mut pool_data: Map<Symbol, i128> = env.storage().persistent().get(&pool_id).unwrap();
        let available: i128 = pool_data.get(Symbol::new(&env, "available_shares")).unwrap();
        assert!(amount <= available, "لا يوجد عدد كافٍ من الأسهم");
        
        // حساب القيمة الإجمالية
        let price: i128 = pool_data.get(Symbol::new(&env, "price_per_share")).unwrap();
        let total_cost = amount * price;
        
        // منطق تحويل المبلغ (سيتم ربطه بـ BIGISH-YER)
        // تسجيل عملية الشراء في دفتر الأستاذ
        pool_data.set(Symbol::new(&env, "available_shares"), available - amount);
        env.storage().persistent().set(&pool_id, &pool_data);
    }

    /// توزيع الأرباح المؤجلة (يستدعى بعد فترة الحجز)
    pub fn distribute_dividends(
        env: Env,
        pool_id: u64,
        total_profit: i128,
    ) {
        // المنطق المعقد لتوزيع الأرباح بناءً على نسبة المساهمة
        // مع الحفاظ على توجيه جزء لتطوير البنية التحتية بعد السنة الثالثة
    }
}
#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Vec, Symbol};

#[derive(Clone)]
pub struct ShareHolder {
    pub address: Address,
    pub shares: i128,
    pub staked_amount: i128,
    pub rewards_earned: i128,
}

#[contract]
pub struct SharesContract;

#[contractimpl]
impl SharesContract {
    pub fn stake(
        env: Env,
        investor: Address,
        amount: i128,
    ) {
        investor.require_auth();
        
        let shareholder: Option<ShareHolder> = env.storage()
            .persistent()
            .get(&Symbol::new(&env, "shareholder"), &investor);
        
        match shareholder {
            Some(mut holder) => {
                holder.staked_amount += amount;
                holder.shares += amount / 100; // 1 share = 100 Pi
                env.storage().persistent().set(
                    &Symbol::new(&env, "shareholder"),
                    &investor,
                    &holder,
                );
            }
            None => {
                let new_holder = ShareHolder {
                    address: investor.clone(),
                    shares: amount / 100,
                    staked_amount: amount,
                    rewards_earned: 0,
                };
                env.storage().persistent().set(
                    &Symbol::new(&env, "shareholder"),
                    &investor,
                    &new_holder,
                );
            }
        }
    }
    
    pub fn withdraw(
        env: Env,
        investor: Address,
    ) -> i128 {
        investor.require_auth();
        
        let shareholder: ShareHolder = env.storage()
            .persistent()
            .get(&Symbol::new(&env, "shareholder"), &investor)
            .unwrap();
        
        let total = shareholder.staked_amount + shareholder.rewards_earned;
        
        // Clear storage
        env.storage().persistent().remove(
            &Symbol::new(&env, "shareholder"),
            &investor,
        );
        
        total
    }
    
    pub fn get_shareholder_info(
        env: Env,
        investor: Address,
    ) -> ShareHolder {
        env.storage()
            .persistent()
            .get(&Symbol::new(&env, "shareholder"), &investor)
            .unwrap_or(ShareHolder {
                address: investor,
                shares: 0,
                staked_amount: 0,
                rewards_earned: 0,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_stake_and_withdraw() {
        let env = Env::default();
        let investor = Address::generate(&env);
        let contract_id = env.register_contract(None, SharesContract);
        let client = SharesContractClient::new(&env, &contract_id);

        client.stake(&investor, &1000);
        
        let info = client.get_shareholder_info(&investor);
        assert_eq!(info.shares, 10);
        assert_eq!(info.staked_amount, 1000);
    }
}