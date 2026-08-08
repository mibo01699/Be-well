#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, String, Vec, Map, Symbol, i128};

// ============================================================
// حالات طلب الخدمة
// ============================================================
const BID_STATUS_OPEN: Symbol = symbol_short!("OPEN");
const BID_STATUS_CLOSED: Symbol = symbol_short!("CLOSED");
const BID_STATUS_AWARDED: Symbol = symbol_short!("AWARD");
const BID_STATUS_COMPLETED: Symbol = symbol_short!("COMP");

#[contract]
pub struct BiddingContract;

#[contractimpl]
impl BiddingContract {
    // ============================================================
    // إنشاء طلب خدمة جديد (من قبل المنصة أو المؤمن له)
    // ============================================================
    pub fn create_service_request(
        env: Env,
        requester: Address,          // معرف المؤمن له (موثق عبر KYC)
        service_type: String,        // نوع الخدمة (فحص طبي، تصليح مركبة، تدقيق، إلخ)
        description: String,         // وصف تفصيلي للخدمة المطلوبة
        location: String,            // موقع تقديم الخدمة
        deadline: u64,               // الموعد النهائي لتقديم العطاءات
        estimated_budget: i128,      // الميزانية التقديرية (للمقارنة)
    ) -> u64 {
        // 1. التحقق من هوية الطالب (KYC)
        // 2. التحقق من صحة البيانات
        // 3. إنشاء معرف فريد للطلب
        let request_id = env.prng().u64();

        // 4. تخزين بيانات الطلب
        let request_data = Map::new(&env);
        request_data.set(Symbol::new(&env, "requester"), requester);
        request_data.set(Symbol::new(&env, "service_type"), service_type);
        request_data.set(Symbol::new(&env, "description"), description);
        request_data.set(Symbol::new(&env, "location"), location);
        request_data.set(Symbol::new(&env, "deadline"), deadline);
        request_data.set(Symbol::new(&env, "estimated_budget"), estimated_budget);
        request_data.set(Symbol::new(&env, "status"), BID_STATUS_OPEN);
        request_data.set(Symbol::new(&env, "created_at"), env.ledger().timestamp());

        // إعداد قائمة فارغة للعطاءات
        let bids: Vec<Map<Symbol, i128>> = Vec::new(&env);
        request_data.set(Symbol::new(&env, "bids"), bids);

        env.storage().persistent().set(&request_id, &request_data);
        
        request_id
    }

    // ============================================================
    // تقديم عطاء من قبل مزود خدمة (مسجل عبر KYB)
    // ============================================================
    pub fn submit_bid(
        env: Env,
        request_id: u64,
        provider: Address,           // مزود الخدمة (موثق عبر KYB)
        bid_amount: i128,            // السعر المقترح بالـ YER
        proposal_hash: String,       // هاش لتفاصيل العرض (للمراجعة)
        estimated_duration: u64,     // المدة التقديرية للخدمة
    ) {
        // 1. التحقق من أن الطلب مفتوح
        let mut request_data: Map<Symbol, i128> = env.storage().persistent().get(&request_id).unwrap();
        let status: Symbol = request_data.get(Symbol::new(&env, "status")).unwrap();
        assert!(status == BID_STATUS_OPEN, "طلب الخدمة مغلق");

        // 2. التحقق من أن مزود الخدمة مسجل في نظام KYB
        // (سيتم التحقق من ذلك عبر استدعاء عقد KYB)

        // 3. إنشاء سجل العطاء
        let bid = Map::new(&env);
        bid.set(Symbol::new(&env, "provider"), provider);
        bid.set(Symbol::new(&env, "bid_amount"), bid_amount);
        bid.set(Symbol::new(&env, "proposal_hash"), proposal_hash);
        bid.set(Symbol::new(&env, "estimated_duration"), estimated_duration);
        bid.set(Symbol::new(&env, "submitted_at"), env.ledger().timestamp());

        // 4. إضافة العطاء إلى قائمة العطاءات
        let mut bids: Vec<Map<Symbol, i128>> = request_data.get(Symbol::new(&env, "bids")).unwrap();
        bids.push_back(bid);
        request_data.set(Symbol::new(&env, "bids"), bids);

        env.storage().persistent().set(&request_id, &request_data);
    }

    // ============================================================
    // إغلاق العطاءات واختيار الفائز (أقل سعر)
    // ============================================================
    pub fn close_bidding_and_award(
        env: Env,
        request_id: u64,
        awarder: Address,           // الطالب أو المنصة
    ) {
        // 1. التحقق من أن الطالب هو من يغلق العطاءات (أو المنصة)
        let mut request_data: Map<Symbol, i128> = env.storage().persistent().get(&request_id).unwrap();
        let requester: Address = request_data.get(Symbol::new(&env, "requester")).unwrap();
        assert!(requester.address() == awarder.address(), "غير مصرح لك بإغلاق العطاءات");

        // 2. التأكد من انتهاء المهلة أو وجود عطاءات
        let deadline: u64 = request_data.get(Symbol::new(&env, "deadline")).unwrap();
        assert!(env.ledger().timestamp() > deadline, "لم تنته مهلة العطاءات");

        let bids: Vec<Map<Symbol, i128>> = request_data.get(Symbol::new(&env, "bids")).unwrap();
        assert!(!bids.is_empty(), "لا توجد عطاءات مقدمة");

        // 3. اختيار العطاء الأقل سعراً
        let mut lowest_bid = bids.get(0).unwrap();
        let mut lowest_amount: i128 = lowest_bid.get(Symbol::new(&env, "bid_amount")).unwrap();

        for i in 1..bids.len() {
            let current_bid = bids.get(i).unwrap();
            let current_amount: i128 = current_bid.get(Symbol::new(&env, "bid_amount")).unwrap();
            if current_amount < lowest_amount {
                lowest_bid = current_bid;
                lowest_amount = current_amount;
            }
        }

        // 4. تحديث حالة الطلب وتعيين الفائز
        request_data.set(Symbol::new(&env, "status"), BID_STATUS_AWARDED);
        request_data.set(Symbol::new(&env, "winner"), lowest_bid.get(Symbol::new(&env, "provider")).unwrap());
        request_data.set(Symbol::new(&env, "awarded_amount"), lowest_amount);
        request_data.set(Symbol::new(&env, "awarded_at"), env.ledger().timestamp());

        env.storage().persistent().set(&request_id, &request_data);
    }

    // ============================================================
    // تأكيد إتمام الخدمة (يستدعيها الفائز بعد الانتهاء)
    // ============================================================
    pub fn complete_service(
        env: Env,
        request_id: u64,
        provider: Address,
        completion_report_hash: String,
    ) {
        let mut request_data: Map<Symbol, i128> = env.storage().persistent().get(&request_id).unwrap();
        let winner: Address = request_data.get(Symbol::new(&env, "winner")).unwrap();
        assert!(winner.address() == provider.address(), "غير مصرح لك بتأكيد الإتمام");

        // تحديث الحالة
        request_data.set(Symbol::new(&env, "status"), BID_STATUS_COMPLETED);
        request_data.set(Symbol::new(&env, "completion_report_hash"), completion_report_hash);
        request_data.set(Symbol::new(&env, "completed_at"), env.ledger().timestamp());

        env.storage().persistent().set(&request_id, &request_data);

        // إطلاق حدث لبدء عملية الدفع (ستكون عبر YER)
        env.events().publish(
            Symbol::new(&env, "service_completed"),
            &(request_id, provider, request_data.get(Symbol::new(&env, "awarded_amount")).unwrap())
        );
    }

    // ============================================================
    // دالة للحصول على تفاصيل الطلب والعطاءات (للاستعلام)
    // ============================================================
    pub fn get_request_details(env: Env, request_id: u64) -> Map<Symbol, i128> {
        env.storage().persistent().get(&request_id).unwrap()
    }
}