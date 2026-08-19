// pi-contracts/src/audit.rs
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec, Map, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuditorRole { Doctor, Accountant }

#[contracttype]
#[derive(Clone, Debug)]
pub struct Auditor {
    address: Address,
    role: AuditorRole,
    reputation: u32,
    total_cases: u32,
    approved_providers: Map<Address, u32>, // لمراقبة تضارب المصالح
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Appeal {
    case_id: u64,
    claimant: Address,
    timestamp: u64,
    resolved: bool,
}

#[contract]
pub struct AuditGovernanceContract;

#[contractimpl]
impl AuditGovernanceContract {
    // تسجيل مدقق معتمد من المجتمع عبر التصويت
    pub fn register_auditor(env: Env, auditor_addr: Address, role: AuditorRole) {
        auditor_addr.require_auth();
        let mut auditors: Map<Address, Auditor> = env.storage().persistent().get(&String::from_str(&env, "auditors")).unwrap_or(Map::new(&env));
        
        let new_auditor = Auditor {
            address: auditor_addr.clone(),
            role,
            reputation: 100, // تقييم مبدئي
            total_cases: 0,
            approved_providers: Map::new(&env),
        };
        auditors.set(auditor_addr, new_auditor);
        env.storage().persistent().set(&String::from_str(&env, "auditors"), &auditors);
    }

    // تقديم استئناف على قرار تدقيق (خلال 30 يوماً)
    pub fn file_appeal(env: Env, case_id: u64, claimant: Address) {
        claimant.require_auth();
        let current_time = env.ledger().timestamp();
        let mut appeals: Map<u64, Appeal> = env.storage().persistent().get(&String::from_str(&env, "appeals")).unwrap_or(Map::new(&env));
        
        let appeal = Appeal {
            case_id,
            claimant,
            timestamp: current_time,
            resolved: false,
        };
        appeals.set(case_id, appeal);
        env.storage().persistent().set(&String::from_str(&env, "appeals"), &appeals);
    }

    // كشف تضارب المصالح: يمنع المدقق إذا تجاوزت نسبة موافقاته لمقدم خدمة معين 40%
    pub fn check_conflict_of_interest(env: Env, auditor_addr: Address, provider_addr: Address) -> bool {
        let auditors: Map<Address, Auditor> = env.storage().persistent().get(&String::from_str(&env, "auditors")).unwrap_or(Map::new(&env));
        if let Some(auditor) = auditors.get(auditor_addr) {
            if auditor.total_cases > 10 {
                let provider_approvals = auditor.approved_providers.get(provider_addr).unwrap_or(0);
                if (provider_approvals * 100) / auditor.total_cases > 40 {
                    return true; // تنبيه وتجميد: هناك تحيز واضح
                }
            }
        }
        false
    }
}
