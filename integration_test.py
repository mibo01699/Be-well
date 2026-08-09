# ============================================================
# الملف: integration_test.py
# المسار: Be-well/tests/integration_test.py
# الغرض: اختبار التكامل بين AI و Gateway ومحاكاة العقد
# ============================================================

import pytest
import httpx
import asyncio

# ============================================================
# تكوين العناوين (يجب تشغيل الخدمات محلياً أولاً)
# ============================================================
AI_URL = "http://localhost:8000"
GATEWAY_URL = "http://localhost:8080"

@pytest.mark.asyncio
async def test_ai_backend_health():
    """اختبار صحة الـ AI Backend"""
    async with httpx.AsyncClient() as client:
        response = await client.get(f"{AI_URL}/health")
        assert response.status_code == 200
        assert response.json()["status"] == "healthy"

@pytest.mark.asyncio
async def test_gateway_health():
    """اختبار صحة الـ API Gateway"""
    async with httpx.AsyncClient() as client:
        response = await client.get(f"{GATEWAY_URL}/api/health")
        assert response.status_code == 200
        assert response.json()["status"] == "healthy"

@pytest.mark.asyncio
async def test_risk_assessment():
    """اختبار تقييم المخاطر عبر الـ API Gateway"""
    risk_data = {
        "age": 35,
        "gender": 1,
        "pre_existing": 0,
        "vehicle_type": 2,
        "region": 1
    }
    async with httpx.AsyncClient() as client:
        response = await client.post(
            f"{GATEWAY_URL}/api/assess-risk",
            json=risk_data
        )
        assert response.status_code == 200
        data = response.json()
        assert "risk_score" in data
        assert "risk_level" in data

@pytest.mark.asyncio
async def test_policy_purchase_flow():
    """اختبار تدفق شراء وثيقة التأمين"""
    policy_data = {
        "holder_address": "GABC123...",
        "policy_type": "HEALTH",
        "coverage_amount": 10000,
        "premium": 500,
        "expiry_date": 100000000,
        "terms_hash": "terms_hash_example"
    }
    async with httpx.AsyncClient() as client:
        # 1. شراء الوثيقة
        response = await client.post(
            f"{GATEWAY_URL}/api/purchase-policy",
            json=policy_data
        )
        assert response.status_code == 200
        data = response.json()
        assert data["success"] is True
        assert "policy_id" in data

        # 2. تقديم مطالبة (استخدام policy_id من الخطوة السابقة)
        # (سيتم إضافة هذا الجزء عند اكتمال ربط العقد الفعلي)