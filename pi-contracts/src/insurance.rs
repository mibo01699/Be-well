// pi-contracts/src/insurance.rs
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Policy {
    pub holder: Address,
    pub premium: u128,
    pub lock_until: u64, // الطابع الزمني لانتهاء القفل التأميني
    pub is_active: bool,
}

const THREE_YEARS_IN_SECONDS: u64 = 94_608_000; // 3 * 365 * 24 * 60 * 60

#[contract]
pub struct BeWellInsuranceContract;

#[contractimpl]
impl BeWellInsuranceContract {
    // إنشاء البوليصة وتفعيل قفل السيولة لـ 3 سنوات وفق شروط Pi
    pub fn create_policy(env: Env, holder: Address, premium_amount: u128) -> bool {
        holder.require_auth();
        
        let current_time = env.ledger().timestamp();
        let release_time = current_time + THREE_YEARS_IN_SECONDS;

        let policy = Policy {
            holder: holder.clone(),
            premium: premium_amount,
            lock_until: release_time,
            is_active: true,
        };

        // حفظ بيانات البوليصة بشكل آمن في البلوكشين
        env.storage().persistent().set(&holder, &policy);
        true
    }

    // التحقق من حالة قفل الـ 3 سنوات (يمنع السحب قبل انتهاء المدة)
    pub fn verify_insurance_lock(env: Env, holder: Address) -> bool {
        if let Some(policy) = env.storage().persistent().get::<Address, Policy>(&holder) {
            let current_time = env.ledger().timestamp();
            if current_time < policy.lock_until {
                return false; // السيولة لا تزال مقفلة بحكم العقد الذكي
            }
            return true;
        }
        false
    }
}
