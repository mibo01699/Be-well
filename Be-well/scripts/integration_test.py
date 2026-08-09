# ============================================================
# الملف: integration_test.py
# المسار: Be-well/scripts/integration_test.py
# الغرض: اختبار التكامل الشامل للمنصة
# ============================================================

import pytest
import httpx
import asyncio

# تكوين العناوين (يجب تشغيل الخدمات محلياً أولاً)
AI_URL = "http://localhost:8000"
GATEWAY_URL = "http://localhost:8080"

@pytest.mark.asyncio
async def test_full_workflow():
    """
    اختبار التدفق الكامل للمنصة:
    1. شراء وثيقة تأمين
    2. تقديم مطالبة وطلب خدمة
    """
    async with httpx.AsyncClient() as client:
        # 1. شراء وثيقة
        policy_data = {
            "holder_address": "GABC123...",
            "policy_type": "HEALTH",
            "coverage_amount": 10000,
            "premium": 500,
            "expiry_date": 100000000,
            "terms_hash": "terms_hash_example"
        }
        policy_response = await client.post(
            f"{GATEWAY_URL}/api/purchase-policy",
            json=policy_data
        )
        assert policy_response.status_code == 200
        policy_result = policy_response.json()
        assert policy_result["success"] is True
        policy_id = policy_result["policy_id"]

        # 2. تقديم مطالبة
        claim_data = {
            "policy_id": policy_id,
            "claimant_address": "GABC123...",
            "claim_amount": 2000,
            "proof_hash": "accident_report_hash",
            "gps_lat": 1234567890,
            "gps_lng": 9876543210,
            "service_description": "تصليح مركبة",
            "service_location": "صنعاء",
            "service_deadline": 100000000,
            "estimated_budget": 3000
        }
        claim_response = await client.post(
            f"{GATEWAY_URL}/api/submit-claim",
            json=claim_data
        )
        assert claim_response.status_code == 200
        claim_result = claim_response.json()
        assert claim_result["success"] is True
        assert "request_id" in claim_result

        print("✅ تم اجتياز اختبار التدفق الكامل بنجاح!")