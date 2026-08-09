# ============================================================
# الملف: app.py
# المسار: Be-well/ai-backend/app.py
# الغرض: خادم FastAPI لتقديم خدمة تقييم المخاطر
# ============================================================

from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from risk_engine import RiskEngine
import uvicorn

# تهيئة التطبيق
app = FastAPI(title="Be-Well Risk Engine API", version="1.0")

# تحميل النموذج
engine = RiskEngine()
if not engine.load_model():
    print("⚠️ النموذج غير موجود. سيتم تدريبه افتراضياً.")
    engine.train_model()

# ============================================================
# نماذج البيانات (Pydantic)
# ============================================================

class RiskFeatures(BaseModel):
    age: int
    gender: int
    pre_existing: int
    vehicle_type: int
    region: int

class RiskResponse(BaseModel):
    risk_score: float
    risk_level: str

# ============================================================
# واجهات برمجة التطبيقات (APIs)
# ============================================================

@app.get("/")
async def root():
    return {"message": "Be-Well AI Risk Engine is running"}

@app.post("/predict", response_model=RiskResponse)
async def predict_risk(features: RiskFeatures):
    try:
        risk_score = engine.predict_risk(features.dict())
        risk_level = "HIGH" if risk_score > 0.5 else "LOW"
        return RiskResponse(risk_score=risk_score, risk_level=risk_level)
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/health")
async def health_check():
    return {"status": "healthy"}

# ============================================================
# تشغيل الخادم
# ============================================================
if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)