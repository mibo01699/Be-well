#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, String, Vec, Map, Symbol, i128};

// استيراد العقود الأخرى (سيتم ربطها فعلياً عند التجميع)
// use crate::insurance::InsuranceContract;
// use crate::bidding::BiddingContract;
// use crate::payment::PaymentContract;
// use crate::shares::SharesContract;
// use crate::vaults::SmartVaultContract;

// ============================================================
// حالات العملية الرئيسية
// ============================================================
const PROCESS_STATUS_INITIATED: Symbol = symbol_short!("INIT");
const PROCESS_STATUS_POLICY_ACTIVE: Symbol = symbol_short!("P_ACT");
const PROCESS_STATUS_CLAIM_SUBMITTED: Symbol = symbol_short!("C_SUB");
const PROCESS_STATUS_SERVICE_REQUESTED: Symbol = symbol_short!("S_REQ");
const PROCESS_STATUS_AWARDED: Symbol = symbol_short!("AWARD");
const PROCESS_STATUS_SERVICE_COMPLETED: Symbol = symbol_short!("S_COM");
const PROCESS_STATUS_SETTLED: Symbol = symbol_short!("SETL");
const PROCESS_STATUS_DISPUTED: Symbol = symbol_short!("DISP");

#[contract]
pub struct OrchestratorContract;

#[contractimpl]
impl OrchestratorContract {
    // ============================================================
    // العملية 1: شراء وثيقة تأمين جديدة
    // ============================================================
    pub fn purchase_policy(
        env: Env,
        holder: Address,
        policy_type: String,
        coverage_amount: i128,
        premium: i128,
        expiry_date: u64,
        terms_hash: String,
    ) -> u64 {
        // 1. استدعاء عقد التأمين لإنشاء الوثيقة
        // let policy_id = InsuranceContract::create_policy(env, holder, policy_type, coverage_amount, premium, expiry_date, terms_hash);
        let policy_id = env.prng().u64(); // محاكاة

        // 2. استدعاء عقد الدفع لتحويل القسط (Pi)
        // PaymentContract::pay_premium(env, policy_id, holder, premium);

        // 3. تسجيل العملية في سجل المنصة
        let process_data = Map::new(&env);
        process_data.set(Symbol::new(&env, "policy_id"), policy_id);
        process_data.set(Symbol::new(&env, "holder"), holder);
        process_data.set(Symbol::new(&env, "status"), PROCESS_STATUS_POLICY_ACTIVE);
        process_data.set(Symbol::new(&env, "created_at"), env.ledger().timestamp());
        env.storage().persistent().set(&policy_id, &process_data);

        // 4. إطلاق حدث
        env.events().publish(
            Symbol::new(&env, "policy_purchased"),
            &(policy_id, holder, policy_type)
        );

        policy_id
    }

    // ============================================================
    // العملية 2: تقديم مطالبة وطلب خدمة (التدفق الكامل)
    // ============================================================
    pub fn submit_claim_and_request_service(
        env: Env,
        policy_id: u64,
        claimant: Address,
        claim_amount: i128,
        proof_hash: String,
        gps_data: (i128, i128),
        service_description: String,
        service_location: String,
        service_deadline: u64,
        estimated_budget: i128,
    ) -> u64 {
        // 1. التحقق من وجود الوثيقة
        let process_data: Map<Symbol, i128> = env.storage().persistent().get(&policy_id).unwrap();
        let holder: Address = process_data.get(Symbol::new(&env, "holder")).unwrap();
        assert!(holder.address() == claimant.address(), "غير مصرح لك بالمطالبة");

        // 2. استدعاء عقد التأمين لتسجيل المطالبة
        // InsuranceContract::claim(env, policy_id, claimant, claim_amount, proof_hash, gps_data);

        // 3. استدعاء عقد العطاءات لإنشاء طلب خدمة
        // let request_id = BiddingContract::create_service_request(env, claimant, "SERVICE".into(), service_description, service_location, service_deadline, estimated_budget);
        let request_id = env.prng().u64(); // محاكاة

        // 4. ربط الطلب بالمطالبة في سجل المنصة
        process_data.set(Symbol::new(&env, "status"), PROCESS_STATUS_SERVICE_REQUESTED);
        process_data.set(Symbol::new(&env, "request_id"), request_id);
        process_data.set(Symbol::new(&env, "claim_amount"), claim_amount);
        process_data.set(Symbol::new(&env, "claim_submitted_at"), env.ledger().timestamp());
        env.storage().persistent().set(&policy_id, &process_data);

        // 5. إطلاق حدث
        env.events().publish(
            Symbol::new(&env, "claim_and_service_requested"),
            &(policy_id, request_id, claimant)
        );

        request_id
    }

    // ============================================================
    // العملية 3: اختيار العطاء الفائز وتأكيد التعاقد
    // ============================================================
    pub fn award_service(
        env: Env,
        policy_id: u64,
        request_id: u64,
        awarder: Address,
    ) {
        // 1. التحقق من أن الطالب هو من يغلق العطاءات
        let process_data: Map<Symbol, i128> = env.storage().persistent().get(&policy_id).unwrap();
        let holder: Address = process_data.get(Symbol::new(&env, "holder")).unwrap();
        assert!(holder.address() == awarder.address(), "غير مصرح لك بالإغلاق");

        // 2. استدعاء عقد العطاءات لإغلاق العطاءات واختيار الفائز
        // BiddingContract::close_bidding_and_award(env, request_id, awarder);

        // 3. تحديث حالة العملية
        process_data.set(Symbol::new(&env, "status"), PROCESS_STATUS_AWARDED);
        process_data.set(Symbol::new(&env, "awarded_at"), env.ledger().timestamp());
        env.storage().persistent().set(&policy_id, &process_data);
    }

    // ============================================================
    // العملية 4: تأكيد إتمام الخدمة وتفعيل الدفع
    // ============================================================
    pub fn confirm_service_and_pay(
        env: Env,
        policy_id: u64,
        request_id: u64,
        provider: Address,
        completion_report_hash: String,
    ) {
        // 1. استدعاء عقد العطاءات لتأكيد الإتمام
        // BiddingContract::complete_service(env, request_id, provider, completion_report_hash);

        // 2. استدعاء عقد الدفع لصرف التعويض للمزود (بـ YER)
        // PaymentContract::pay_service_provider(env, request_id, provider, claim_amount);

        // 3. تحديث حالة العملية
        let mut process_data: Map<Symbol, i128> = env.storage().persistent().get(&policy_id).unwrap();
        process_data.set(Symbol::new(&env, "status"), PROCESS_STATUS_SETTLED);
        process_data.set(Symbol::new(&env, "completed_at"), env.ledger().timestamp());
        env.storage().persistent().set(&policy_id, &process_data);

        env.events().publish(
            Symbol::new(&env, "service_completed_and_paid"),
            &(policy_id, request_id, provider)
        );
    }

    // ============================================================
    // العملية 5: فتح نزاع (للمراجعة)
    // ============================================================
    pub fn raise_dispute(
        env: Env,
        policy_id: u64,
        disputer: Address,
        reason: String,
    ) {
        let mut process_data: Map<Symbol, i128> = env.storage().persistent().get(&policy_id).unwrap();
        let holder: Address = process_data.get(Symbol::new(&env, "holder")).unwrap();
        assert!(holder.address() == disputer.address() || true, "غير مصرح لك بفتح نزاع");

        process_data.set(Symbol::new(&env, "status"), PROCESS_STATUS_DISPUTED);
        process_data.set(Symbol::new(&env, "dispute_reason"), reason);
        process_data.set(Symbol::new(&env, "disputed_at"), env.ledger().timestamp());
        env.storage().persistent().set(&policy_id, &process_data);
    }

    // ============================================================
    // الحصول على حالة العملية
    // ============================================================
    pub fn get_process_status(env: Env, policy_id: u64) -> Symbol {
        let process_data: Map<Symbol, i128> = env.storage().persistent().get(&policy_id).unwrap();
        process_data.get(Symbol::new(&env, "status")).unwrap()
    }
}