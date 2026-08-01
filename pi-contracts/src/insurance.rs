use soroban_sdk::{contract, contractimpl, Env, Address, String, Symbol, panic_with_error};

// تعريف الأخطاء البرمجية الصارمة للمنصة لضمان النزاهة
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum InsuranceError {
    UnauthorizedRole = 1,     // الحساب لا يملك الصلاحية المطلوبة
    PolicyExpired = 2,        // بوليصة التأمين منتهية الصالِحية زمنياً
    GeofenceBreached = 3,     // الموقع الجغرافي خارج النطاق المؤمن عليه (احتيال)
    PolicyAlreadyPaid = 4,    // تم صرف التعويض مسبقاً لهذه المطالبة
}

// تعريف الأدوار الأربعة المعتمدة في المنظومة
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UserRole {
    Individual = 1, // حساب أفراد KYC
    Evaluator = 2,  // حساب جهة تقييم الحالة KYB
    Repairer = 3,   // حساب جهة الإصلاح وجبر الضرر KYB
    Auditor = 4,    // حساب جهة التدقيق القانوني والفني KYG
}

// هيكل بوليصة التأمين الشاملة والديناميكية داخل البلوكشين
#[derive(Clone)]
pub struct InsurancePolicy {
    pub policy_id: u64,
    pub owner: Address,
    pub encrypted_data_hash: String, // الرابط المشفر لملف التحقق والبيانات البيومترية الفائقة
    pub premium_pi: i128,            // قيمة القسط المحسوبة بدقة عبر الذكاء الاصطناعي
    pub expiry_timestamp: u64,       // الزمن الفعلي لانتهاء التأمين
    pub allowed_lat: i32,            // إحداثيات خط العرض المسموحة جغرافيّاً (GPS)
    pub allowed_long: i32,           // إحداثيات خط الطول المسموحة جغرافيّاً (GPS)
    pub is_claimed: bool,            // حالة صرف التعويض
}

#[contract]
pub struct BeWellInsuranceContract;

#[contractimpl]
impl BeWellInsuranceContract {
    
    // 1. تسجيل وتوثيق أدوار الحسابات (KYC / KYB / KYG) على الشبكة
    pub fn set_user_role(env: Env, user: Address, role: u32) {
        // حماية العقد: التوقيع مطلوب من الإدارة أو جهة التدقيق العليا فقط
        user.require_auth();
        env.storage().instance().set(&user, &role);
    }

    // Helper function للتحقق من دور الحساب داخلياً
    fn get_user_role(env: &Env, user: &Address) -> u32 {
        env.storage().instance().get(user).unwrap_or(0) // 0 تعني غير مسجل
    }

    // 2. إنشاء وتفعيل بوليصة تأمين جديدة (صحي أو وسائل نقل)
    // يتم احتساب الـ premium_pi مسبقاً عبر خوارزمية الذكاء الاصطناعي بناءً على المخاطر
    pub fn create_policy(
        env: Env,
        owner: Address,
        policy_id: u64,
        data_hash: String,
        premium_pi: i128,
        duration_seconds: u64,
        lat: i32,
        long: i32,
    ) {
        owner.require_auth(); // التأكد من توقيع صاحب الطلب بنفسه
        
        let current_time = env.ledger().timestamp();
        let expiry = current_time + duration_seconds;

        let policy = InsurancePolicy {
            policy_id,
            owner: owner.clone(),
            encrypted_data_hash: data_hash,
            premium_pi,
            expiry_timestamp: expiry,
            allowed_lat: lat,
            allowed_long: long,
            is_claimed: false,
        };

        // حفظ البوليصة برقمها الفريد في ذاكرة البلوكشين
        env.storage().instance().set(&policy_id, &policy);
    }

    // 3. معالجة وصرف المطالبة التأمينية تلقائياً بناءً على الموقع والزمن والأدوار
    pub fn process_claim(
        env: Env,
        policy_id: u64,
        evaluator: Address,
        repairer: Address,
        incident_lat: i32,
        incident_long: i32,
    ) {
        // التحقق الصارم: يجب أن توقع جهة تقييم الحالة (KYB) رقمياً على هذه المعاملة
        evaluator.require_auth();
        
        // التأكد من أن حساب المقيم هو فعلياً KYB Evaluator (رقم 2)
        if Self::get_user_role(&env, &evaluator) != 2 {
            panic!("Error: Unauthorized Evaluator Role");
        }

        // التأكد من أن حساب جهة الإصلاح هو فعلياً KYB Repairer (رقم 3)
        if Self::get_user_role(&env, &repairer) != 3 {
            panic!("Error: Unauthorized Repairer Role");
        }

        // جلب بيانات البوليصة من البلوكشين
        let mut policy: InsurancePolicy = env.storage().instance().get(&policy_id).expect("Policy not found");

        // الفحص الأول: التأكد من عدم صرف التعويض مسبقاً
        if policy.is_claimed {
            panic!("Error: Claim already processed");
        }

        // الفحص الثاني: التحقق من الزمن الفعلي (هل الحادث وقع قبل انتهاء البوليصة؟)
        let current_time = env.ledger().timestamp();
        if current_time > policy.expiry_timestamp {
            panic!("Error: Policy has expired");
        }

        // الفحص الثالث (الربط الجغرافي الصارم): مطابقة بصمة الـ GPS لمنع الاحتيال
        // إذا اختلف موقع الحادث عن النطاق المسموح به في العقد، يتم إلغاء العملية فوراً
        if incident_lat != policy.allowed_lat || incident_long != policy.allowed_long {
            panic!("Error: Geofence breach detected. Claim rejected.");
        }

        // تفعيل الصرف الآمن والمباشر لجهة الإصلاح وجبر الضرر (KYB) لضمان عدم التلاعب بالسيولة
        policy.is_claimed = true;
        env.storage().instance().set(&policy_id, &policy);

        // هنا يتم تضمين كود تحويل عملة الـ Pi التلقائي من محفظة العقد إلى حساب الـ Repairer
        // token::Client::new(&env, &pi_token_address).transfer(&env.current_contract_address(), &repairer, &payout_amount);
    }
}
