#![cfg(test)]
use soroban_sdk::{Env, Address, String, Vec, Map, Symbol, i128};
use soroban_sdk::testutils::{Address as _, Env as _};
use crate::bidding::BiddingContract;

#[test]
fn test_create_service_request() {
    let env = Env::default();
    let requester = Address::generate(&env);

    let request_id = BiddingContract::create_service_request(
        env.clone(),
        requester.clone(),
        String::from_str(&env, "MECHANICAL"),
        String::from_str(&env, "إصلاح محرك سيارة"),
        String::from_str(&env, "صنعاء"),
        100_000_000u64,
        i128::from(10000u32),
    );

    assert!(request_id > 0);

    // التحقق من التخزين
    let request_details = BiddingContract::get_request_details(env, request_id);
    let requester_from_storage: Address = request_details.get(Symbol::new(&env, "requester")).unwrap();
    assert_eq!(requester_from_storage, requester);
}

#[test]
fn test_submit_and_win_bid() {
    let env = Env::default();
    let requester = Address::generate(&env);
    let provider1 = Address::generate(&env);
    let provider2 = Address::generate(&env);

    // 1. إنشاء طلب خدمة
    let request_id = BiddingContract::create_service_request(
        env.clone(),
        requester.clone(),
        String::from_str(&env, "MEDICAL"),
        String::from_str(&env, "فحص طبي شامل"),
        String::from_str(&env, "عدن"),
        100_000_000u64,
        i128::from(5000u32),
    );

    // 2. تقديم عطاءات (العطاء الثاني أقل سعراً)
    BiddingContract::submit_bid(
        env.clone(),
        request_id,
        provider1.clone(),
        i128::from(4500u32),
        String::from_str(&env, "proposal_hash_1"),
        1000u64,
    );
    BiddingContract::submit_bid(
        env.clone(),
        request_id,
        provider2.clone(),
        i128::from(4000u32), // أقل سعر
        String::from_str(&env, "proposal_hash_2"),
        800u64,
    );

    // 3. إغلاق العطاءات واختيار الفائز
    BiddingContract::close_bidding_and_award(
        env.clone(),
        request_id,
        requester.clone(),
    );

    // 4. التحقق من اختيار الفائز (أقل سعر)
    let request_details = BiddingContract::get_request_details(env, request_id);
    let winner: Address = request_details.get(Symbol::new(&env, "winner")).unwrap();
    assert_eq!(winner, provider2);

    let awarded_amount: i128 = request_details.get(Symbol::new(&env, "awarded_amount")).unwrap();
    assert_eq!(awarded_amount, i128::from(4000u32));
}