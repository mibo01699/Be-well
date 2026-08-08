#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, String, Vec, Map, Symbol, i128};

// ============================================================
// إدارة حالات وثيقة التأمين
// ============================================================
const POLICY_STATUS_ACTIVE: Symbol = symbol_short!("ACTIVE");
const POLICY_STATUS_EXPIRED: Symbol = symbol_short!("EXP");
const POLICY_STATUS_CLAIMED: Symbol = symbol_short!("CLM");
const POLICY_STATUS_DISPUTED: Symbol = symbol_short!("DISP");

#[contract]
pub struct InsuranceContract;

#[contractimpl]
impl InsuranceContract {
    // ============================================================
    // إنشاء وثيقة تأمين جديدة (للمستخدمين الموثقين عبر KYC)
    // ============================================================
    pub fn create_policy(
        env: Env,
        holder: Address,       // معرف المستخدم (مرتبط بـ KYC Pi)
        policy_type: String,   // نوع التأمين: صحي، مركبات، سفر، إلخ
        coverage_amount: i128, // المبلغ المؤمن عليه
        premium: i128,         // القسط المدفوع
        expiry_date: u64,      // تاريخ انتهاء الوثيقة
        terms_hash: String,    // هاش لشروط الخدمة (لمنع التلاعب)
    ) -> u64 {
        // 1. التحقق من هوية المستخدم (تمت مسبقاً عبر KYC)
        // 2. التحقق من صحة البيانات
        // 3. إنشاء معرف فريد للوثيقة
        let policy_id = env.prng().u64();
        
        // 4. تخزين الوثيقة في دفتر الأستاذ
        let policy_data = Map::new(&env);
        policy_data.set(Symbol::new(&env, "holder"), holder);
        policy_data.set(Symbol::new(&env, "policy_type"), policy_type);
        policy_data.set(Symbol::new(&env, "coverage"), coverage_amount);
        policy_data.set(Symbol::new(&env, "premium"), premium);
        policy_data.set(Symbol::new(&env, "expiry"), expiry_date);
        policy_data.set(Symbol::new(&env, "terms_hash"), terms_hash);
        policy_data.set(Symbol::new(&env, "status"), POLICY_STATUS_ACTIVE);
        policy_data.set(Symbol::new(&env, "created_at"), env.ledger().timestamp());
        
        env.storage().persistent().set(&policy_id, &policy_data);
        
        // 5. إطلاق حدث للتتبع
        env.events().publish(
            Symbol::new(&env, "policy_created"),
            &(policy_id, holder, policy_type)
        );
        
        policy_id
    }

    // ============================================================
    // تقديم مطالبة تأمين
    // ============================================================
    pub fn claim(
        env: Env,
        policy_id: u64,
        claimant: Address,
        claim_amount: i128,
        proof_hash: String,   // هاش للدليل المقدم (مثل تقرير طبي، حادث)
        gps_data: (i128, i128), // إحداثيات GPS لدقة الموقع
    ) {
        // 1. التحقق من وجود الوثيقة وصلاحيتها
        let policy_data: Map<Symbol, i128> = env.storage().persistent().get(&policy_id).unwrap();
        let status: Symbol = policy_data.get(Symbol::new(&env, "status")).unwrap();
        assert!(status == POLICY_STATUS_ACTIVE, "الوثيقة غير نشطة");

        // 2. التحقق من أن المطالب هو صاحب الوثيقة
        let holder: Address = policy_data.get(Symbol::new(&env, "holder")).unwrap();
        assert!(holder.address() == claimant.address(), "غير مصرح لك بتقديم المطالبة");

        // 3. تحديث حالة الوثيقة إلى "مطالب بها"
        policy_data.set(Symbol::new(&env, "status"), POLICY_STATUS_CLAIMED);
        env.storage().persistent().set(&policy_id, &policy_data);
        
        // 4. إطلاق حدث للمطالبة (سيتم معالجته بواسطة الـ AI Backend)
        env.events().publish(
            Symbol::new(&env, "claim_submitted"),
            &(policy_id, claimant, claim_amount, proof_hash, gps_data)
        );
    }

    // ============================================================
    // تسوية المطالبة (يستدعيها نظام الذكاء الاصطناعي أو المدقق)
    // ============================================================
    pub fn settle_claim(
        env: Env,
        policy_id: u64,
        approved: bool,
        settlement_amount: i128,
        auditor: Address,
    ) {
        // 1. التحقق من أن المستدعي مفوض (مدقق مسجل عبر KYB)
        // 2. تحديث حالة الوثيقة
        let mut policy_data: Map<Symbol, i128> = env.storage().persistent().get(&policy_id).unwrap();
        if approved {
            policy_data.set(Symbol::new(&env, "status"), POLICY_STATUS_CLAIMED); // أو حالة جديدة
            // منطق تحويل المبلغ إلى المستفيد
        } else {
            policy_data.set(Symbol::new(&env, "status"), POLICY_STATUS_ACTIVE);
        }
        env.storage().persistent().set(&policy_id, &policy_data);
        
        // 3. إطلاق حدث التسوية
    }

    // ============================================================
    // دالة للحصول على تفاصيل الوثيقة (للاستعلام العام)
    // ============================================================
    pub fn get_policy_details(env: Env, policy_id: u64) -> Map<Symbol, i128> {
        env.storage().persistent().get(&policy_id).unwrap()
    }
}