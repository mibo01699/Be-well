// ============================================================
// إضافة إلى عقد التأمين (insurance.rs)
// ============================================================

// دالة جديدة لربط المطالبة بطلب خدمة
pub fn request_service_for_claim(
    env: Env,
    claim_id: u64,
    service_type: String,
    description: String,
    location: String,
    deadline: u64,
    estimated_budget: i128,
) -> u64 {
    // استدعاء عقد العطاءات لإنشاء طلب خدمة
    // سيتم تمرير معرف المطالبة لربطها بالخدمة
    // سيتم استدعاء هذا من قبل المؤمن له أو المنصة
    unimplemented!()
}

#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String, Map, Symbol};

#[derive(Clone, PartialEq)]
pub enum PolicyStatus {
    Active,
    Expired,
    Claimed,
    Rejected,
}

#[derive(Clone)]
pub struct Policy {
    pub id: u64,
    pub holder: Address,
    pub policy_type: String,
    pub coverage_amount: i128,
    pub premium_paid: i128,
    pub status: PolicyStatus,
    pub issued_at: u64,
    pub expires_at: u64,
}

#[contract]
pub struct InsuranceContract;

#[contractimpl]
impl InsuranceContract {
    pub fn create_policy(
        env: Env,
        holder: Address,
        policy_type: String,
        coverage_amount: i128,
        premium_paid: i128,
        duration_days: u64,
    ) -> u64 {
        holder.require_auth();
        
        let now = env.ledger().timestamp();
        let policy_id = env.prng().generate::<u64>();
        
        let policy = Policy {
            id: policy_id,
            holder,
            policy_type,
            coverage_amount,
            premium_paid,
            status: PolicyStatus::Active,
            issued_at: now,
            expires_at: now + (duration_days * 86400),
        };
        
        env.storage().persistent().set(
            &Symbol::new(&env, "policy"),
            &policy_id,
            &policy,
        );
        
        policy_id
    }
    
    pub fn submit_claim(
        env: Env,
        holder: Address,
        policy_id: u64,
        claim_details: Map<String, String>,
    ) {
        holder.require_auth();
        
        let policy: Policy = env.storage()
            .persistent()
            .get(&Symbol::new(&env, "policy"), &policy_id)
            .unwrap();
        
        let now = env.ledger().timestamp();
        if now > policy.expires_at {
            panic!("Policy expired");
        }
        
        let updated_policy = Policy {
            status: PolicyStatus::Claimed,
            ..policy
        };
        
        env.storage().persistent().set(
            &Symbol::new(&env, "policy"),
            &policy_id,
            &updated_policy,
        );
    }
    
    pub fn get_policy(
        env: Env,
        policy_id: u64,
    ) -> Policy {
        env.storage()
            .persistent()
            .get(&Symbol::new(&env, "policy"), &policy_id)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_create_policy() {
        let env = Env::default();
        let holder = Address::generate(&env);
        let contract_id = env.register_contract(None, InsuranceContract);
        let client = InsuranceContractClient::new(&env, &contract_id);

        let policy_id = client.create_policy(
            &holder,
            &String::from_str(&env, "HEALTH"),
            &10000,
            &500,
            &365,
        );
        
        assert!(policy_id > 0);
    }
}



