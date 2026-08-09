# ============================================================
# الملف: api_gateway.py
# المسار: Be-well/backend/api_gateway.py
# الغرض: واجهة برمجة تطبيقات (API) موحدة للتكامل بين AI والبلوكشين
# ============================================================

from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import httpx
import os
import json

# ============================================================
# تكوين العناوين
# ============================================================
AI_BACKEND_URL = os.getenv("AI_BACKEND_URL", "http://localhost:8000")
# عنوان عقد التنسيق (سيتم تحديده بعد النشر على Pi Network)
ORCHESTRATOR_CONTRACT_ADDRESS = os.getenv("ORCHESTRATOR_ADDRESS", "0x...")

app = FastAPI(title="Be-Well API Gateway", version="1.0")

# ============================================================
# نماذج البيانات للطلبات والاستجابات
# ============================================================

class PolicyPurchaseRequest(BaseModel):
    holder_address: str
    policy_type: str
    coverage_amount: int
    premium: int
    expiry_date: int
    terms_hash: str

class ClaimRequest(BaseModel):
    policy_id: int
    claimant_address: str
    claim_amount: int
    proof_hash: str
    gps_lat: int
    gps_lng: int
    service_description: str
    service_location: str
    service_deadline: int
    estimated_budget: int

class RiskAssessmentRequest(BaseModel):
    age: int
    gender: int
    pre_existing: int
    vehicle_type: int
    region: int

# ============================================================
# واجهات برمجة التطبيقات (APIs) - التكامل مع AI والبلوكشين
# ============================================================

@app.post("/api/purchase-policy")
async def purchase_policy(request: PolicyPurchaseRequest):
    """
    شراء وثيقة تأمين جديدة (تقييم المخاطر + استدعاء العقد)
    """
    try:
        # 1. استدعاء AI Backend لتقييم المخاطر (محاكاة)
        async with httpx.AsyncClient() as client:
            # في الواقع، سيتم جمع بيانات المستخدم من ملفه الشخصي
            risk_features = {
                "age": 35,  # سيتم جلبها من ملف المستخدم
                "gender": 1,
                "pre_existing": 0,
                "vehicle_type": 2,
                "region": 1
            }
            risk_response = await client.post(
                f"{AI_BACKEND_URL}/predict",
                json=risk_features
            )
            risk_data = risk_response.json()
            risk_score = risk_data.get("risk_score", 0.5)

            # تعديل القسط بناءً على درجة المخاطرة
            adjusted_premium = int(request.premium * (1 + risk_score))

        # 2. استدعاء عقد التنسيق (Orchestrator) على البلوكشين
        #    هذا الجزء سيتم تنفيذه باستخدام Pi SDK أو Soroban SDK
        #    سنقوم بمحاكاة الاستدعاء حالياً

        # 3. إرجاع النتيجة
        return {
            "success": True,
            "policy_id": 12345,
            "risk_score": risk_score,
            "adjusted_premium": adjusted_premium,
            "message": "تم شراء الوثيقة بنجاح"
        }

    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.post("/api/submit-claim")
async def submit_claim(request: ClaimRequest):
    """
    تقديم مطالبة وطلب خدمة (التدفق الكامل)
    """
    try:
        # 1. التحقق من صحة المطالبة عبر AI (كشف الاحتيال)
        #    سيتم إرسال بيانات المطالبة إلى AI Backend للتحليل
        #    async with httpx.AsyncClient() as client:
        #       verification = await client.post(...)

        # 2. استدعاء عقد التنسيق (Orchestrator) لتنفيذ العملية
        #    هذا الجزء سيتم تنفيذه باستخدام Pi SDK أو Soroban SDK

        # 3. إرجاع النتيجة
        return {
            "success": True,
            "request_id": 67890,
            "message": "تم تقديم المطالبة وطلب الخدمة بنجاح"
        }

    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.post("/api/assess-risk")
async def assess_risk(features: RiskAssessmentRequest):
    """
    تقييم المخاطر باستخدام AI (دالة مساعدة للواجهة الأمامية)
    """
    try:
        async with httpx.AsyncClient() as client:
            response = await client.post(
                f"{AI_BACKEND_URL}/predict",
                json=features.dict()
            )
            data = response.json()
            return {
                "success": True,
                "risk_score": data.get("risk_score"),
                "risk_level": data.get("risk_level")
            }
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.get("/api/health")
async def health_check():
    return {"status": "healthy", "ai_backend": AI_BACKEND_URL}

# ============================================================
# تشغيل الخادم
# ============================================================
if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8080)