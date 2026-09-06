// ============================================================
// الملف: app.js - Be-well Platform (Sandbox/Testnet)
// الدور: منصة التأمين اللامركزية
// ============================================================

const express = require('express');
const cors = require('cors');
const app = express();

// التفعيلات الأساسية
app.use(cors());
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// ============================================================
// نقاط النهاية الأساسية (APIs)
// ============================================================

// نقطة الصحة (Health Check)
app.get('/api/health', (req, res) => {
    res.json({
        status: 'online',
        service: 'Be-well',
        version: '1.0.0',
        sandbox: true,
        timestamp: new Date().toISOString()
    });
});

// نقطة التوطين (Localization)
try {
    const languageManager = require('./locales/languageManager');
    app.get('/api/localization', (req, res) => {
        const userLang = req.headers['accept-language'];
        const data = languageManager.detectAndGetTranslation(userLang);
        res.json(data);
    });
} catch (error) {
    app.get('/api/localization', (req, res) => {
        res.json({ message: 'Localization service unavailable', fallback: 'en' });
    });
}

// المسار الرئيسي
app.get('/', (req, res) => {
    res.json({
        message: '🦅 Be-well Platform API is running',
        version: '1.0.0',
        environment: 'sandbox',
        endpoints: ['/api/health', '/api/localization']
    });
});

// ============================================================
// ✅ نقطة الدخول لـ Vercel (تصدير التطبيق)
// ============================================================
module.exports = app;

// ============================================================
// تشغيل الخادم محلياً (فقط عند التشغيل المباشر)
// ============================================================
if (require.main === module) {
    const PORT = process.env.PORT || 3000;
    app.listen(PORT, () => {
        console.log(`🦅 Be-well server running on port ${PORT}`);
    });
}