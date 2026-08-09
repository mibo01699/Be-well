// ============================================================
// الملف: app.js
// المسار: Be-well/frontend/app.js
// الغرض: التفاعل مع واجهات برمجة التطبيقات (APIs) الخلفية
// ============================================================

const API_BASE = 'http://localhost:8080/api';

// ============================================================
// عرض الأقسام
// ============================================================
function showSection(sectionId) {
    document.querySelectorAll('.section').forEach(el => el.classList.remove('active'));
    const section = document.getElementById(sectionId);
    if (section) section.classList.add('active');
}

// ============================================================
// شراء وثيقة تأمين
// ============================================================
document.getElementById('policy-form').addEventListener('submit', async (e) => {
    e.preventDefault();
    const resultDiv = document.getElementById('policy-result');
    resultDiv.textContent = '⏳ جاري المعالجة...';

    const data = {
        holder_address: document.getElementById('holder-address').value,
        policy_type: document.getElementById('policy-type').value,
        coverage_amount: parseInt(document.getElementById('coverage').value),
        premium: parseInt(document.getElementById('premium').value),
        expiry_date: Math.floor(Date.now() / 1000) + 31536000, // سنة واحدة
        terms_hash: 'terms_hash_example'
    };

    try {
        const res = await fetch(`${API_BASE}/purchase-policy`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(data)
        });
        const result = await res.json();
        resultDiv.textContent = JSON.stringify(result, null, 2);
    } catch (err) {
        resultDiv.textContent = `❌ خطأ: ${err.message}`;
    }
});

// ============================================================
// تقديم مطالبة وطلب خدمة
// ============================================================
document.getElementById('claim-form').addEventListener('submit', async (e) => {
    e.preventDefault();
    const resultDiv = document.getElementById('claim-result');
    resultDiv.textContent = '⏳ جاري المعالجة...';

    const data = {
        policy_id: parseInt(document.getElementById('claim-policy-id').value),
        claimant_address: 'GABC123...', // سيتم جلبها من المحفظة
        claim_amount: parseInt(document.getElementById('claim-amount').value),
        proof_hash: 'accident_report_hash',
        gps_lat: 1234567890,
        gps_lng: 9876543210,
        service_description: document.getElementById('service-desc').value,
        service_location: document.getElementById('service-location').value,
        service_deadline: Math.floor(Date.now() / 1000) + 604800, // أسبوع
        estimated_budget: parseInt(document.getElementById('claim-amount').value) * 1.5
    };

    try {
        const res = await fetch(`${API_BASE}/submit-claim`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(data)
        });
        const result = await res.json();
        resultDiv.textContent = JSON.stringify(result, null, 2);
    } catch (err) {
        resultDiv.textContent = `❌ خطأ: ${err.message}`;
    }
});

// ============================================================
// تقييم المخاطر (AI)
// ============================================================
document.getElementById('risk-form').addEventListener('submit', async (e) => {
    e.preventDefault();
    const resultDiv = document.getElementById('risk-result');
    resultDiv.textContent = '⏳ جاري التقييم...';

    const data = {
        age: parseInt(document.getElementById('age').value),
        gender: parseInt(document.getElementById('gender').value),
        pre_existing: parseInt(document.getElementById('pre-existing').value),
        vehicle_type: parseInt(document.getElementById('vehicle-type').value),
        region: parseInt(document.getElementById('region').value)
    };

    try {
        const res = await fetch(`${API_BASE}/assess-risk`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(data)
        });
        const result = await res.json();
        resultDiv.textContent = JSON.stringify(result, null, 2);
    } catch (err) {
        resultDiv.textContent = `❌ خطأ: ${err.message}`;
    }
});

// ============================================================
// تهيئة العرض عند التحميل
// ============================================================
document.addEventListener('DOMContentLoaded', () => {
    showSection('new-policy');
});