#![cfg(test)]
use soroban_sdk::{Env, Address, String, i128};
use soroban_sdk::testutils::{Address as _, Env as _};
use crate::shares::SharesContract;

#[test]
fn test_create_pool() {
    let env = Env::default();
    let manager = Address::generate(&env);

    let pool_id = SharesContract::create_pool(
        env.clone(),
        String::from_str(&env, "صندوق التأمين الصحي"),
        String::from_str(&env, "مجمع استثماري لتغطية المخاطر الصحية"),
        i128::from(10000u32),
        i128::from(10u32),
        5u64, // 5 سنوات حجز
        5u64, // توزيع بعد 5 سنوات
    );

    assert!(pool_id > 0);

    // التحقق من التخزين
    let pool_data = SharesContract::get_pool_details(env, pool_id);
    let name: String = pool_data.get(Symbol::new(&env, "name")).unwrap();
    assert_eq!(name, String::from_str(&env, "صندوق التأمين الصحي"));
}

#[test]
fn test_buy_shares() {
    let env = Env::default();
    let manager = Address::generate(&env);
    let buyer = Address::generate(&env);

    // 1. إنشاء مجمع
    let pool_id = SharesContract::create_pool(
        env.clone(),
        String::from_str(&env, "صندوق التأمين الصحي"),
        String::from_str(&env, "مجمع استثماري"),
        i128::from(10000u32),
        i128::from(10u32),
        5u64,
        5u64,
    );

    // 2. شراء أسهم
    SharesContract::buy_shares(
        env.clone(),
        pool_id,
        buyer.clone(),
        i128::from(100u32),
    );

    // 3. التحقق من انخفاض الأسهم المتاحة
    let pool_data = SharesContract::get_pool_details(env, pool_id);
    let available: i128 = pool_data.get(Symbol::new(&env, "available_shares")).unwrap();
    assert_eq!(available, i128::from(9900u32)); // 10000 - 100
}