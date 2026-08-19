// backend/UnifiedIdentityRegistry.js
const PiSDK = require('@pi-apps/sdk'); // متوافق مع تحديثات Pi Mainnet 2026

class UnifiedIdentityRegistry {
    constructor(dbInstance, antiDoubleDippingEngine) {
        this.db = dbInstance;
        this.antiFraud = antiDoubleDippingEngine;
    }

    /**
     * تسجيل وتوثيق الكيانات (أفراد، شركات، مقدمي خدمات)
     */
    async registerEntity(piUserId, entityType, companyDetails = null) {
        // 1. التحقق من توثيق Pi الرسمي عبر الـ SDK للمطورين
        const piAuthStatus = await PiSDK.getKycStatus(piUserId);
        if (!piAuthStatus.verified) {
            throw new Error("ENTITY_AUTH_FAILED: لم يتم اجتياز توثيق Pi KYC الرسمي بعد.");
        }

        // 2. التحقق من الهوية بناءً على نوع الكيان المحدث
        let registrationData = {
            piUserId,
            entityType, // Individual | Company | ServiceProvider
            verifiedAt: Date.now(),
            yerWalletAddress: `YER_${piUserId.substring(0, 10).toUpperCase()}_WALLET`
        };

        if (entityType === 'Company' || entityType === 'ServiceProvider') {
            if (!companyDetails || !companyDetails.commercialRegister) {
                throw new Error("KYB_REQUIRED: متطلبات توثيق السجل التجاري والتراخيص مفقودة.");
            }
            registrationData.kybDetails = companyDetails;
            registrationData.status = 'Pending_Manual_Audit'; // يخضع لتدقيق حوكمة بي ويل
        } else {
            registrationData.status = 'Active';
        }

        // 3. منع تسجيل الحسابات المتكررة أو التلاعب بالهوية
        const exists = await this.db.findEntityByPiId(piUserId);
        if (exists) throw new Error("DUPLICATE_ENTITY: هذا الكيان مسجل بالفعل بالنظام.");

        await this.db.saveEntity(registrationData);
        return registrationData;
    }

    /**
     * معالجة المطالبة والتحقق من منع الصرف المكرر لحظياً
     */
    async processClaimVerification(piUserId, claimAmountYer, serviceInvoiceId) {
        // استدعاء محرك AntiDoubleDippingEngine لمنع صرف مخصصات مكررة لنفس الخدمة
        const isDoubleDipping = await this.antiFraud.checkDuplicateClaim({
            userId: piUserId,
            invoiceId: serviceInvoiceId,
            amount: claimAmountYer,
            timestamp: Date.now()
        });

        if (isDoubleDipping) {
            throw new Error("SECURITY_ALERT: تم رصد محاولة صرف مكررة! تعليق المعاملة فوراً.");
        }

        return { approved: true, trackingId: `TX_WELL_${Date.now()}` };
    }
}

module.exports = UnifiedIdentityRegistry;
