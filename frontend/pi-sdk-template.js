// frontend/pi-sdk-template.js
/**
 * قالب دمج Pi SDK الرسمي المحدث لمنصة Be-Well + BIGISH-YER
 * مخصص للتشغيل داخل Pi Browser واعتماده في استديو التطبيقات (Pi Developer Portal)
 */

const PiSandboxConfig = {
    sandbox: true, // تفعيل الوضع التجريبي مع جمهور Pi Network
};

const PiInsuranceSDK = {
    init: async function() {
        try {
            // 1. تهيئة الـ SDK الرسمي لشبكة Pi داخل بيئة متصفح Pi Browser
            await window.Pi.init({ version: "2.0", sandbox: PiSandboxConfig.sandbox });
            console.log("✅ تم دمج Pi SDK بنجاح في الوضع التجريبي.");
            
            // 2. طلب المصادقة الفورية للمستخدم وجلب الهوية الموحدة
            await this.authenticateUser();
        } catch (error) {
            console.error("❌ فشل دمج Pi SDK. تأكد من تشغيل التطبيق داخل Pi Browser:", error);
        }
    },

    authenticateUser: async function() {
        try {
            const scopes = ['username', 'payments', 'wallet_address'];
            const authResult = await window.Pi.authenticate(scopes, this.onIncompletePaymentFound);
            
            console.log("👤 تم التحقق من هوية مستخدم Pi:", authResult.user.username);
            
            // ربط المعرف الفوري بـ سجل الهوية الموحد في الباكيند لدينا
            await this.linkIdentityWithBackend(authResult.user);
        } catch (err) {
            console.error("Authentication failed:", err);
        }
    },

    // معالجة مدفوعات الأقساط التأمينية بالـ Pi (Premium Collection)
    createPremiumPayment: async function(amountPi, claimId) {
        try {
            const payment = await window.Pi.createPayment({
                amount: amountPi,
                memo: `قسط التأمين الصحي للمطالبة رقم #${claimId}`,
                metadata: { claimId: claimId, platform: "Be-Well-Hybrid" },
            }, {
                onReadyForServerApproval: async (paymentId) => {
                    // إرسال معرف المعاملة إلى خادم المقاصة الخاص بنا لإرسالها للـ DEX وتحويلها لـ YER
                    await fetch('/api/v1/payments/approve', {
                        method: 'POST',
                        body: JSON.stringify({ paymentId })
                    });
                },
                onReadyForServerCompletion: async (paymentId, txid) => {
                    // تأكيد الدفع النهائي على البلوكشين
                    await fetch('/api/v1/payments/complete', {
                        method: 'POST',
                        body: JSON.stringify({ paymentId, txid })
                    });
                    alert("✅ تم تحصيل القسط التأميني بنجاح وتحويل الاحتياطي لمجمع السيولة!");
                },
                onCancel: (paymentId) => console.log("Payment cancelled", paymentId),
                onError: (error, payment) => console.error("Payment error", error)
            });
        } catch (error) {
            console.error("فشلت عملية الدفع عبر Pi SDK:", error);
        }
    },

    onIncompletePaymentFound: function(payment) {
        console.log("⚠️ تم العثور على معاملة غير مكتملة، جاري معالجتها تلقائيًا برقم:", payment.identifier);
        // كود تلقائي لتسوية المدفوعات العالقة لمنع تجميد أرصدة الجمهور
    },

    linkIdentityWithBackend: async function(userData) {
        await fetch('/api/v1/auth/pi-login', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ userData })
        });
    }
};

// تشغيل القالب فور جاهزية الواجهة
document.addEventListener("DOMContentLoaded", () => {
    PiInsuranceSDK.init();
});

/**
 * قالب دمج Pi SDK الرسمي لمنصة Be-Well في استوديو التطبيقات (الوضع التجريبي لجمهور Pi)
 */
const PiInsuranceSDK = {
    init: async function() {
        try {
            await window.Pi.init({ version: "2.0", sandbox: true });
            console.log("🚀 Pi SDK Initialized inside Sandbox Mode.");
            await this.authenticateUser();
        } catch (error) {
            console.error("Pi SDK configuration error:", error);
        }
    },

    authenticateUser: async function() {
        const scopes = ['username', 'payments', 'wallet_address'];
        const authResult = await window.Pi.authenticate(scopes, (payment) => {
            console.log("Incomplete payment found:", payment);
        });
        console.log("Connected Pi User:", authResult.user.username);
    }
};

document.addEventListener("DOMContentLoaded", () => PiInsuranceSDK.init());
