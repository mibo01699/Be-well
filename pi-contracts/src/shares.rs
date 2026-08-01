use soroban_sdk::{contract, contractimpl, Env, Address, Map, Symbol, panic_with_error};

#[contract]
pub struct BeWellSharesContract;

#[contractimpl]
impl BeWellSharesContract {
    
    // إيداع عملات Pi في المجمع للحصول على أسهم استثمارية
    pub fn invest_pool(env: Env, investor: Address, amount_pi: i128) {
        investor.require_auth();
        
        // جلب سجل الأسهم الحالي للمستثمر أو تعيينه لـ 0 إذا كان جديداً
        let mut investor_shares: Map<Address, i128> = env.storage().instance().get(&Symbol::new(&env, "shares")).unwrap_or(Map::new(&env));
        let current_shares = investor_shares.get(investor.clone()).unwrap_or(0);
        
        // تحديث إجمالي الأسهم (تحويل الـ Pi المودع إلى حصص بالتناسب 1:1 كمثال)
        investor_shares.set(investor.clone(), current_shares + amount_pi);
        env.storage().instance().set(&Symbol::new(&env, "shares"), &investor_shares);

        // تحديث إجمالي السيولة في صندوق التأمين الاحتياطي
        let total_pool: i128 = env.storage().instance().get(&Symbol::new(&env, "total_pool")).unwrap_or(0);
        env.storage().instance().set(&Symbol::new(&env, "total_pool"), &(total_pool + amount_pi));
    }

    // توزيع الأرباح التلقائي على حاملي الأسهم (يتم استدعاؤه دورياً بواسطة المنصة)
    pub fn distribute_dividends(env: Env, total_profit_pi: i128) {
        // حماية برمجية: لا يتم استدعاء هذا الأمر إلا بتوقيع جهة التدقيق العليا (KYG)
        let auditor: Address = env.storage().instance().get(&Symbol::new(&env, "admin_auditor")).expect("Auditor not set");
        auditor.require_auth();

        let investor_shares: Map<Address, i128> = env.storage().instance().get(&Symbol::new(&env, "shares")).expect("No investors found");
        let total_pool: i128 = env.storage().instance().get(&Symbol::new(&env, "total_pool")).unwrap_or(1); // منع القسمة على صفر

        // حلقة تكرارية ذكية لتوزيع الأرباح بالتناسب مع نسبة أسهم كل مستثمر
        for (investor, shares) in investor_shares.iter() {
            // المعادلة: الأرباح المستحقة = (أسهم المستثمر / إجمالي السيولة) * إجمالي الأرباح الموزعة
            let dividend_payout = (shares * total_profit_pi) / total_pool;
            
            if dividend_payout > 0 {
                // هنا يتم تفعيل الـ Pi SDK Token client لتحويل الأرباح الفورية لمحفظة المستثمر
                // token_client.transfer(&env.current_contract_address(), &investor, &dividend_payout);
            }
        }
    }
}
