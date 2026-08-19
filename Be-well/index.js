/**
 * Be-well Healthcare Ecosystem - Main Core Server
 * منظومة Arabian Eagle Ecosystem (A.E.C.) - ربط محرك التحذير الطبي المعزول
 */

const express = require('express');
const cors = require('cors');
const healthAlerts = require('./notifications/health_alerts'); // استدعاء وحدة التنبيهات الطبية

const app = express();
const PORT = process.env.PORT || 3001;

app.use(cors());
app.use(express.json());

// بث إشعار نظام عند إقلاع خادم الرعاية الصحية
app.listen(PORT, () => {
    healthAlerts.triggerAlert('SYSTEM', 'INFO', `خادم Be-well الطبي مستقر ويعمل الآن على المنفذ ${PORT}`);
});

// مسار استقبال المؤشرات الحيوية ومحاكاة التنبيهات الحرجة للمرضى
app.post('/api/vitals/update', (req, res) => {
    const { patientId, heartRate } = req.body;
    
    if (heartRate > 120) {
        const alert = healthAlerts.triggerAlert(
            patientId, 
            'VITAL_SIGNS', 
            `ارتفاع حرج في معدل ضربات القلب للمريض: ${heartRate} نبضة/دقيقة`
        );
        return res.json({ status: "CRITICAL_ALERT_TRIGGERED", alert });
    }
    
    res.json({ status: "NORMAL", message: "المؤشرات الحيوية مستقرة تماماً وضمن النطاق الآمن." });
});
