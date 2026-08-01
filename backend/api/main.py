import os
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from typing import List, Dict

app = FastAPI(
    title="Be-well AI Sovereign Risk & Infrastructure Core",
    version="1.1.0",
    description="محرك الذكاء الاصطناعي الخاص بالمنصة لتحليل كثافة الجمهور وتوجيه رأس مال الأسهم لبناء المرافق دولياً."
)

# نموذج بيانات لحساب كثافة الجمهور في الدول
class CountryDensityPayload(BaseModel):
    country_code: str
    active_pioneers_count: int
    local_claims_rate: float      # معدل الحوادث والمطالبات محلياً (0.0 إلى 1.0)
    nearest_infrastructure_km: float # المسافة لأقرب منشأة تابعة للمنصة حالياً

class InfrastructurePriorityResponse(BaseModel):
    country_code: str
    priority_score: float         # مؤشر الأولوية من 100
    recommended_facility: str     # المنشأة الموصى ببنائها فوراً لتحقيق الاكتفاء الذاتي
    allocated_budget_tier: str    # حجم الميزانية المقترحة من رأس مال الأسهم

@app.get("/")
def read_root():
    return {
        "status": "active", 
        "engine": "Be-well AI Infrastructure Optimization Core", 
        "rules": "3-Year Capital Lock Active"
    }

@app.post("/ai/geo-expansion/analyze", response_model=List[InfrastructurePriorityResponse])
async def analyze_infrastructure_priority(countries_data: List[CountryDensityPayload]):
    """
    خوارزمية مطورة ذاتياً لتحليل كثافة جمهور المنصة وتحديد الدول ذات الأولوية
    لشراء وتجهيز مستشفيات أو ورش إصلاح مركزية من رأس مال الأسهم الاستثمارية.
    """
    analysis_results = []
    
    for data in countries_data:
        # معادلة الذكاء الاصطناعي التنبؤية للأولوية:
        # تعتمد طردياً على عدد المستخدمين ونسبة المطالبات وعكسياً مع قرب المرافق الحالية
        density_weight = data.active_pioneers_count / 10000.0 # لكل 10 آلاف مستخدم نشط
        claims_weight = data.local_claims_rate * 50.0
        distance_urgency = min(data.nearest_infrastructure_km / 10.0, 30.0) # حد أقصى لتأثير المسافة
        
        # حساب المؤشر النهائي للأولوية الدولية
        priority_score = min((density_weight + claims_weight + distance_urgency), 100.0)
        
        # اتخاذ قرار ذكي بشأن نوع المنشأة المطلوبة للاكتفاء الذاتي
        if claims_weight > 30.0:
            recommended_facility = "مستشفى مركزي ومجمع عيادات تشخيصية متكاملة (Micro-Hospital)"
            budget_tier = "HIGH_CAPITAL_TIER"
        else:
            recommended_facility = "ورشة إصلاح مركزية ومرفق دعم لوجستي فني (Central Workshop)"
            budget_tier = "MEDIUM_CAPITAL_TIER"
            
        analysis_results.append(
            InfrastructurePriorityResponse(
                country_code=data.country_code,
                priority_score=round(priority_score, 2),
                recommended_facility=recommended_facility,
                allocated_budget_tier=budget_tier
            )
        )
        
    # ترتيب الدول تنازلياً حسب أولوية احتياج الجمهور لتوجيه العقد الذكي وصندوق السيادة
    analysis_results.sort(key=lambda x: x.priority_score, reverse=True)
    return analysis_results
