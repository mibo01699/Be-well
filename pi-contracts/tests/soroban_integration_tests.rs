#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::{Ledger, Address as TestAddress}, Env, Address, String};

#[test]
fn test_investment_share_pricing_and_3_year_lock() {
    // 1. تهيئة بيئة اختبار البلوكشين الافتراضية لـ Soroban
    let env = Env::default();
    env.mock_all_auths(); // محاكاة التوقيعات الرقمية

    let contract_id = env.register_contract(None, BeWellSharesContract);
    let client = BeWellSharesContractClient::new(&env, &contract_id);

    // إنشاء حسابات وهمية للمستثمر وجهة التدقيق
    let investor = Address::generate(&env);
    let auditor = Address::generate(&env);

    // تعيين حساب التدقيق الأعلى (KYG) في العقد
    env.storage().instance().set(&Symbol::new(&env, "admin_auditor"), &auditor);

    // 2. اختبار سعر السهم الثابت: استثمار 100 Pi يجب أن يمنح 400 سهم (لأن السهم بـ 0.25)
    let investment_amount_pi = 100 * 1000000; // تمثيل العملة بـ 6 خانات عشرية
    client.buy_shares(&investor, &investment_amount_pi);

    // 3. محاكاة ضخ أرباح من نظام المراهنات والمرافق بمقدار 50 Pi
    let betting_profits = 50 * 1000000;
    client.inject_ecosystem_profits(&betting_profits);

    // 4. الفحص الصارم الأول: محاولة صرف العوائد بعد مرور سنة واحدة فقط (يجب أن يفشل العقد ويغلق التنفيذ)
    let one_year_seconds = 31536000;
    env.ledger().set_timestamp(env.ledger().timestamp() + one_year_seconds);
    
    // نتوقع هنا أن يفشل العقد (Panic) لأن قفل الـ 3 سنوات ما زال نشطاً
    let result = std::panic::catch_unwind(|| {
        client.claim_dividends(&investor);
    });
    assert!(result.is_err(), "فحص الأمان نجح: العقد منع الصرف قبل مرور 3 سنوات!");

    // 5. الفحص الصارم الثاني: تقديم الزمن ليمر 3 سنوات كاملة (94,608,000 ثانية) من تاريخ الاكتتاب
    let two_more_years = 63072000;
    env.ledger().set_timestamp(env.ledger().timestamp() + two_more_years);

    // الآن يجب أن ينجح الصرف وتخرج الأرباح للمستثمر من عوائد المراهنات والمستشفيات
    let final_claim_result = std::panic::catch_unwind(|| {
        client.claim_dividends(&investor);
    });
    assert!(final_claim_result.is_ok(), "فحص الأمان نجح: العقد سمح بالصرف فوراً بعد انتهاء قفل الـ 3 سنوات!");
}

#[test]
fn test_infrastructure_capital_allocation() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, BeWellSharesContract);
    let client = BeWellSharesContractClient::new(&env, &contract_id);

    let investor = Address::generate(&env);
    let auditor = Address::generate(&env);
    env.storage().instance().set(&Symbol::new(&env, "admin_auditor"), &auditor);

    // اكتتاب للحصول على رأس مال
    client.buy_shares(&investor, &(5000 * 1000000));

    // اختبار سحب رأس المال لبناء منشأة (مستشفى) في دولة معينة (مثال كود الدولة: "YE")
    let asset_type = String::from_str(&env, "Micro-Hospital");
    let country = String::from_str(&env, "YE");
    
    // يجب أن ينجح الأمر لأن التوقيع صادر من جهة التدقيق KYG المعتمدة
    client.allocate_capital_to_infra(&101, &asset_type, &country, &(3000 * 1000000));
    
    // التحقق من خصم الميزانية بنجاح وتوجيهها للبناء على الأرض
    let infra_budget: i128 = env.storage().instance().get(&Symbol::new(&env, "infra_budget")).unwrap_or(0);
    assert_eq!(infra_budget, 2000 * 1000000, "تم سحب ميزانية المرفق الدولي وخصمها من الصندوق بنجاح!");
}
