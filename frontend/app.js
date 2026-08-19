// frontend/app.js
const BeWellApp = {
    init: async function() {
        console.log("Initializing Hybrid Insurance Platform...");
        await this.connectPiWallet();
        this.renderUnifiedDashboard();
    },

    connectPiWallet: async function() {
        try {
            // ربط محفظة Pi الرسمية من خلال Pi Browser SDK
            const response = await window.Pi.authenticate(['payments', 'username']);
            this.userWallet = response.user.uid;
            document.getElementById("wallet-status").innerText = `Connected: ${this.userWallet}`;
        } catch (error) {
            console.error("Pi Wallet connection failed", error);
        }
    },

    renderUnifiedDashboard: function() {
        // عرض البيانات المالية الموحدة والغطاء التأميني المشترك مع BIGISH-YER
        const dashboardHtml = `
            <div class="card">
                <h3>لوحة التحكم الموحدة (Be-Well & YER)</h3>
                <p>رصيد المحفظة التأميني: <span id="yer-balance">0.00 YER</span></p>
                <p>حالة التوثيق (Identity): <span id="kyc-status">Verified (Individual)</span></p>
                <button onclick="BeWellApp.triggerDigitalDoctor()">استشارة الطبيب الرقمي AI</button>
            </div>
        `;
        document.getElementById("app-container").innerHTML = dashboardHtml;
    },

    triggerDigitalDoctor: async function() {
        // واجهة تفاعلية مع الطبيب الرقمي والمساعد الذكي GavAiSupportSystem.js
        const userSymptom = prompt("مرحباً بك في نظام الدعم الذكي، كيف تشعر اليوم؟");
        if (userSymptom) {
            document.getElementById("ai-support-logs").innerText = "جاري تحليل المؤشرات السلوكية والصحية...";
            const aiResponse = await fetch('/api/v1/ai/digital-doctor', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ symptom: userSymptom, wallet: this.userWallet })
            });
            const result = await aiResponse.json();
            alert(`توصية الطبيب الرقمي: ${result.advice}\nتم جدولة تذكير وقائي لموعدك القادم.`);
        }
    }
};

window.onload = () => BeWellApp.init();
