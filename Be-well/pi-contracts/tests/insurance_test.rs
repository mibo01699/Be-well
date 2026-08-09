#![cfg(test)]
use soroban_sdk::{Env, Address, String, i128};
use soroban_sdk::testutils::{Address as _, Env as _};
use crate::insurance::InsuranceContract;

#[test]
fn test_create_policy() {
    // 1. تهيئة البيئة
    let env = Env::default();
    let holder = Address::generate(&env);

    // 2. استدعاء دالة إنشاء الوثيقة
    let policy_id = InsuranceContract::create_policy(
        env.clone(),
        holder.clone(),
        String::from_str(&env, "HEALTH"),
        i128::from(10000u32),
        i128::from(500u32),
        100_000_000u64, // تاريخ انتهاء افتراضي
        String::from_str(&env, "terms_hash_example"),
    );

    // 3. التحقق من صحة النتيجة
    assert!(policy_id > 0);
    
    // 4. استدعاء دالة الحصول على التفاصيل للتحقق من التخزين
    let policy_details = InsuranceContract::get_policy_details(env, policy_id);
    let holder_from_storage: Address = policy_details.get(Symbol::new(&env, "holder")).unwrap();
    assert_eq!(holder_from_storage, holder);
}

#[test]
fn test_claim() {
    // 1. تهيئة البيئة وإنشاء وثيقة أولاً
    let env = Env::default();
    let holder = Address::generate(&env);
    let policy_id = InsuranceContract::create_policy(
        env.clone(),
        holder.clone(),
        String::from_str(&env, "HEALTH"),
        i128::from(10000u32),
        i128::from(500u32),
        100_000_000u64,
        String::from_str(&env, "terms_hash_example"),
    );

    // 2. تقديم مطالبة
    InsuranceContract::claim(
        env.clone(),
        policy_id,
        holder.clone(),
        i128::from(2000u32),
        String::from_str(&env, "proof_hash_example"),
        (i128::from(1234567890), i128::from(9876543210)),
    );

    // 3. التحقق من تغيير حالة الوثيقة
    let policy_details = InsuranceContract::get_policy_details(env, policy_id);
    let status: Symbol = policy_details.get(Symbol::new(&env, "status")).unwrap();
    assert_eq!(status, Symbol::new(&env, "CLM")); // POLICY_STATUS_CLAIMED
}