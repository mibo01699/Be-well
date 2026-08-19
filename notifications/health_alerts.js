/**
 * Be-well Healthcare Ecosystem - Internal Alerts & Notifications
 * منظومة Arabian Eagle Ecosystem (A.E.C.) - طبقة الاتصال والتحذير الطبي المعزول
 */

class HealthAlertEngine {
    constructor() {
        this.alertQueue = [];
    }

    // توليد تنبيه طبي حيوي داخلي
    triggerAlert(patientId, alertType, description) {
        const alert = {
            alertId: `ALERT-${Date.now()}-${Math.floor(Math.random() * 1000)}`,
            patientId,
            alertType, // e.g., VITAL_SIGNS, APPOINTMENT, SYSTEM
            description,
            status: "UNREAD",
            createdAt: new Date().toISOString()
        };

        this.alertQueue.push(alert);
        console.log(`[🏥 BE-WELL ALERT] [${alertType}] Patient: ${patientId} -> ${description}`);
        return alert;
    }

    markAsRead(alertId) {
        const alert = this.alertQueue.find(a => a.alertId === alertId);
        if (alert) {
            alert.status = "READ";
            return true;
        }
        return false;
    }
}

module.exports = new HealthAlertEngine();
