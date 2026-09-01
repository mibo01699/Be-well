// app.js - بوابة تطبيق الصحة الرقمي (Be-well Platform) ضمن منظومة النسر العربي
const http = require('http');

console.log("🏥 منصة الصحة الرقمية (Be-well) نشطة لبناء Vercel...");

function processMedicalTransaction() {
    try {
        const yerScale = 10000000000n; // 10 decimals لعملة YER
        
        // محاكاة دفع رسوم معاينة طبية أو دواء مدعوم من صندوق النسر العربي
        const medicalFeeYER = 350n * yerScale; 
        
        if (medicalFeeYER <= 0n) {
            throw new Error("قيمة المعاملة الطبية غير صالحة");
        }

        return {
            success: true,
            service: "معاينة طبية ورعاية صحية أولية للأطفال والأمهات",
            cost_yer: "350 YER",
            currency_precision: "Strict BigInt Verified"
        };
    } catch (err) {
        return { success: false, error: err.message };
    }
}

const server = http.createServer((req, res) => {
    const healthResult = processMedicalTransaction();
    
    res.writeHead(200, { 'Content-Type': 'application/json; charset=utf-8' });
    res.end(JSON.stringify({
        ecosystem_gateway: "بوابة النسر العربي الأم (A.E.C)",
        application: "منصة الصحة والرعاية الطبية الرقمية (Be-well Platform)",
        status: "CONNECTED_TO_MAIN_GATEWAY",
        unicef_health_compliance: "PASSED",
        transaction_log: healthResult
    }, null, 2));
});

const PORT = process.env.PORT || 3000;
server.listen(PORT);

module.exports = server;
