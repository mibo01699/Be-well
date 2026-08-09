#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol, panic_with_error};

// 1. Define strict Error codes for the Insurance lifecycle
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum InsuranceError {
    PolicyAlreadyActive = 101,
    UnauthorizedGateway = 102,
    InvalidRiskSignature = 103,
    InsufficientPremium = 104,
}

// 2. Structuring the Policy Metadata in Ledger Storage
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Policy {
    pub pioneer: Address,
    pub risk_score: u32,
    pub premium_paid: i128,
    pub is_active: bool,
    pub expiration_ledger: u32,
}

#[contract]
pub struct BeWellInsuranceContract;

#[contractimpl]
impl BeWellInsuranceContract {
    /// Initializes and purchases a new decentralized insurance policy securely
    pub fn purchase_policy(
        env: Env,
        pioneer: Address,
        gateway: Address,
        risk_score: u32,
        premium: i128,
        duration_ledgers: u32,
    ) -> Result<Policy, InsuranceError> {
        // Enforce Pioneer authentication via native wallet cryptography
        pioneer.require_auth();

        // Generate the unique storage key for this specific user
        let policy_key = Symbol::new(&env, "policy");
        
        if env.storage().persistent().has(&pioneer) {
            return Result::Err(InsuranceError::PolicyAlreadyActive);
        }

        // Validate that the risk score parameters sent from Python are secure (e.g., scale 0-100)
        if risk_score > 100 {
            return Result::Err(InsuranceError::InvalidRiskSignature);
        }

        // Calculate lockup timeframe based on current Stellar/Pi ledger status
        let current_ledger = env.ledger().sequence();
        let expiration = current_ledger + duration_ledgers;

        let new_policy = Policy {
            pioneer: pioneer.clone(),
            risk_score,
            premium_paid: premium,
            is_active: true,
            expiration_ledger: expiration,
        };

        // Save the verified policy metadata persistently to the blockchain ledger
        env.storage().persistent().set(&pioneer, &new_policy);

        // Emit an immutable on-chain event for the Pi Browser UI tracking
        env.events().publish(
            (Symbol::new(&env, "policy_activated"), pioneer),
            risk_score,
        );

        Result::Ok(new_policy)
    }

    /// Fetches live policy status from the blockchain storage for auditing
    pub fn get_policy(env: Env, pioneer: Address) -> Option<Policy> {
        env.storage().persistent().get(&pioneer)
    }
}
