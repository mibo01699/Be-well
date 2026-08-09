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
"""
Be-well AI Backend - FastAPI Gateway
Handles risk assessment and fraud detection
"""

from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from typing import List, Dict, Optional
import joblib
import numpy as np
from datetime import datetime
import logging

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

app = FastAPI(
    title="Be-well AI Engine",
    description="Risk assessment and fraud detection for decentralized insurance",
    version="1.0.0"
)

# ============== Data Models ==============
class RiskAssessmentRequest(BaseModel):
    user_id: str
    age: int
    health_history: List[str]
    vehicle_condition: Optional[Dict] = None

class FraudDetectionRequest(BaseModel):
    transaction_id: str
    amount: float
    gps_coordinates: Dict[str, float]
    timestamp: float

class RiskResponse(BaseModel):
    risk_score: float
    risk_level: str
    recommended_premium: float
    timestamp: str

class FraudResponse(BaseModel):
    fraud_score: float
    is_fraudulent: bool
    reason: Optional[str] = None
    timestamp: str

# ============== Mock AI Models ==============
# In production, replace with actual trained models
class RiskModel:
    def predict(self, data: Dict) -> float:
        # Mock risk scoring logic
        age = data.get('age', 30)
        health_issues = len(data.get('health_history', []))
        
        base_score = 0.3
        age_factor = 0.01 * (age - 25) if age > 25 else 0
        health_factor = 0.05 * health_issues
        
        return min(base_score + age_factor + health_factor, 1.0)

class FraudModel:
    def predict(self, data: Dict) -> Dict:
        # Mock fraud detection logic
        amount = data.get('amount', 0)
        # Simple rule: amounts > 10000 flagged
        is_fraudulent = amount > 10000
        
        return {
            'fraud_score': 0.8 if is_fraudulent else 0.1,
            'is_fraudulent': is_fraudulent,
            'reason': 'Amount exceeds threshold' if is_fraudulent else None
        }

# Initialize models
risk_model = RiskModel()
fraud_model = FraudModel()

# ============== Endpoints ==============
@app.get("/")
async def root():
    return {
        "service": "Be-well AI Backend",
        "status": "operational",
        "version": "1.0.0"
    }

@app.get("/health")
async def health_check():
    return {"status": "healthy", "timestamp": datetime.utcnow().isoformat()}

@app.post("/assess_risk", response_model=RiskResponse)
async def assess_risk(request: RiskAssessmentRequest):
    try:
        logger.info(f"Processing risk assessment for user: {request.user_id}")
        
        data = request.dict()
        risk_score = risk_model.predict(data)
        
        # Determine risk level
        if risk_score < 0.3:
            risk_level = "LOW"
            premium_multiplier = 1.0
        elif risk_score < 0.6:
            risk_level = "MEDIUM"
            premium_multiplier = 1.5
        else:
            risk_level = "HIGH"
            premium_multiplier = 2.0
        
        base_premium = 100.0  # Base premium in Pi
        recommended_premium = base_premium * premium_multiplier
        
        return RiskResponse(
            risk_score=risk_score,
            risk_level=risk_level,
            recommended_premium=recommended_premium,
            timestamp=datetime.utcnow().isoformat()
        )
    
    except Exception as e:
        logger.error(f"Error in risk assessment: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/detect_fraud", response_model=FraudResponse)
async def detect_fraud(request: FraudDetectionRequest):
    try:
        logger.info(f"Processing fraud detection for transaction: {request.transaction_id}")
        
        data = request.dict()
        result = fraud_model.predict(data)
        
        return FraudResponse(
            fraud_score=result['fraud_score'],
            is_fraudulent=result['is_fraudulent'],
            reason=result.get('reason'),
            timestamp=datetime.utcnow().isoformat()
        )
    
    except Exception as e:
        logger.error(f"Error in fraud detection: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/models/status")
async def model_status():
    return {
        "risk_model": "loaded",
        "fraud_model": "loaded",
        "last_updated": datetime.utcnow().isoformat()
    }

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)

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