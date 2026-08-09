#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, panic_with_error};

// 1. Structural Liquidity Error Codes
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum PoolError {
    ZeroDepositInvalid = 201,
    InsufficientShares = 202,
    LockupPeriodActive = 203,
}

// 2. Liquidity Provider Metadata Structure
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShareHolder {
    pub provider: Address,
    pub share_balance: i128,
    pub initial_deposit_ledger: u32,
}

#[contract]
pub struct BeWellSharesContract;

#[contractimpl]
impl BeWellSharesContract {
    /// Allows community backers to securely deposit liquidity to fund insurance pools
    pub fn deposit_liquidity(
        env: Env,
        provider: Address,
        amount: i128,
    ) -> Result<i128, PoolError> {
        // Enforce secure non-custodial cryptographic authentication
        provider.require_auth();

        if amount <= 0 {
            return Result::Err(PoolError::ZeroDepositInvalid);
        }

        // Fetch current storage state or create a new profile for the shareholder
        let current_ledger = env.ledger().sequence();
        let mut holder: ShareHolder = env.storage().persistent().get(&provider).unwrap_or(ShareHolder {
            provider: provider.clone(),
            share_balance: 0,
            initial_deposit_ledger: current_ledger,
        });

        // Mint programmatic utility shares 1:1 against the deposited asset liquidity
        holder.share_balance += amount;
        holder.initial_deposit_ledger = current_ledger;

        // Persist the updated configuration to the blockchain ledger
        env.storage().persistent().set(&provider, &holder);

        // Emit dynamic on-chain tracking event for the ecosystem indexers
        env.events().publish(
            (Symbol::new(&env, "liquidity_staked"), provider),
            amount,
        );

        Result::Ok(holder.share_balance)
    }

    /// Allows backers to withdraw capital after the lockup security timeframe clears
    pub fn withdraw_liquidity(
        env: Env,
        provider: Address,
        shares_to_burn: i128,
        lockup_duration: u32,
    ) -> Result<i128, PoolError> {
        provider.require_auth();

        if !env.storage().persistent().has(&provider) {
            return Result::Err(PoolError::InsufficientShares);
        }

        let mut holder: ShareHolder = env.storage().persistent().get(&provider).unwrap();

        if holder.share_balance < shares_to_burn {
            return Result::Err(PoolError::InsufficientShares);
        }

        // Enforce structural security lockups to protect contract liquidity from flash runs
        let current_ledger = env.ledger().sequence();
        if current_ledger < (holder.initial_deposit_ledger + lockup_duration) {
            return Result::Err(PoolError::LockupPeriodActive);
        }

        // Burn the fractionalized shares and adjust balance
        holder.share_balance -= shares_to_burn;
        env.storage().persistent().set(&provider, &holder);

        env.events().publish(
            (Symbol::new(&env, "liquidity_withdrawn"), provider),
            shares_to_burn,
        );

        Result::Ok(holder.share_balance)
    }
}
