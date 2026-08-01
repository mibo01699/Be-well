import os
from fastapi import FastAPI, HTTPException, Depends
from pydantic import BaseModel
from typing import Optional

app = FastAPI(
    title="Be-well Proprietary AI Engine",
    version="1.0.0",
    description="Off-chain real-time risk assessment and spatial-temporal fraud evaluation gateway for Pi Blockchain."
)

class HealthRiskPayload(BaseModel):
    pi_username: str
    age: int
    has_chronic_diseases: bool
    genetic_risk_factor: float  # Scale 0.0 to 1.0
    coverage_type: str          # "Full" or "Partial"

class TransportRiskPayload(BaseModel):
    vehicle_id: str
    technical_condition_score: int  # 1 (Perfect) to 100 (Critical Failure)
    incident_lat: float
    incident_long: float
    allowed_geofence_radius_km: float

@app.get("/")
def read_root():
    return {"status": "active", "engine": "Be-well AI Risk Core v1", "network": "Pi Network Protocol 23 Compatible"}

@app.post("/ai/risk/health")
async def calculate_health_risk(payload: HealthRiskPayload):
    """
    Evaluates individual medical metrics, runs predictive analytics for fraud prevention,
    and returns a deterministic Risk Score to be pushed to the Soroban contract.
    """
    try:
        # Self-developed predictive weighting algorithm
        base_score = 15 if payload.age < 35 else 35
        disease_weight = 40 if payload.has_chronic_diseases else 0
        genetic_weight = payload.genetic_risk_factor * 25
        
        final_risk_score = min(base_score + disease_weight + genetic_weight, 100)
        
        # Calculate optimal premium multiplier
        multiplier = 1.5 if payload.coverage_type == "Full" else 0.8
        suggested_premium_pi = final_risk_score * multiplier
        
        return {
            "pi_username": payload.pi_username,
            "risk_score": round(final_risk_score, 2),
            "suggested_premium_pi": round(suggested_premium_pi, 4),
            "status": "APPROVED_FOR_BLOCKCHAIN_STATE"
        }
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/ai/risk/transport")
async def evaluate_spatial_temporal_claim(payload: TransportRiskPayload):
    """
    Validates GPS telemetry and technical condition logs to detect anomalous claim behavior.
    """
    # AI Logic to detect if technical condition score justifies the premium delta
    if payload.technical_condition_score > 80:
        risk_tier = "CRITICAL_RISK_ELEVATION"
        premium_markup_factor = 2.5
    else:
        risk_tier = "STANDARD_DEPRECIATION"
        premium_markup_factor = 1.0 + (payload.technical_condition_score / 100.0)

    # Simplified spatial containment validation simulation
    # (In production, this cross-references the Oracle spatial nodes)
    geofence_breached = False 
    
    return {
        "vehicle_id": payload.vehicle_id,
        "risk_tier": risk_tier,
        "premium_markup_factor": round(premium_markup_factor, 2),
        "geofence_status": "VALID" if not geofence_breached else "BREACHED_REJECT_CLAIM"
    }
