#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, i128, Map, Symbol};

const VAULT_STATUS_LOCKED: Symbol = symbol_short!("LOCKED");
const VAULT_STATUS_RELEASED: Symbol = symbol_short!("RELEASED");
const VAULT_STATUS_DISPUTED: Symbol = symbol_short!("DISP");

#[contract]
pub struct SmartVaultContract;

#[contractimpl]
impl SmartVaultContract {
    /// إنشاء خزنة ذكية جديدة
    pub fn create_vault(
        env: Env,
        owner: Address,
        amount: i128,
        conditions_hash: String, // هاش لشروط الإفراج
        release_time: u64,       // وقت الإفراج التلقائي (إن وجد)
    ) -> u64 {
        let vault_id = env.prng().u64();
        let vault_data = Map::new(&env);
        vault_data.set(Symbol::new(&env, "owner"), owner);
        vault_data.set(Symbol::new(&env, "amount"), amount);
        vault_data.set(Symbol::new(&env, "conditions_hash"), conditions_hash);
        vault_data.set(Symbol::new(&env, "release_time"), release_time);
        vault_data.set(Symbol::new(&env, "status"), VAULT_STATUS_LOCKED);
        vault_data.set(Symbol::new(&env, "created_at"), env.ledger().timestamp());
        
        env.storage().persistent().set(&vault_id, &vault_data);
        vault_id
    }

    /// الإفراج عن الأموال (عند استيفاء الشروط)
    pub fn release_vault(
        env: Env,
        vault_id: u64,
        approver: Address,
    ) {
        let mut vault_data: Map<Symbol, i128> = env.storage().persistent().get(&vault_id).unwrap();
        let status: Symbol = vault_data.get(Symbol::new(&env, "status")).unwrap();
        assert!(status == VAULT_STATUS_LOCKED, "الخزنة غير مقفلة");
        
        // منطق التحقق من الشروط وتحديث الحالة
        vault_data.set(Symbol::new(&env, "status"), VAULT_STATUS_RELEASED);
        env.storage().persistent().set(&vault_id, &vault_data);
    }
}