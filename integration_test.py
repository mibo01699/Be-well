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
#!/usr/bin/env python3
"""
Integration Test for Be-well Platform
Testing workflow: Health Check → Risk Assessment → Fraud Detection
"""

import requests
import json
import sys
import time
from typing import Dict, Any

class BeWellTester:
    def __init__(self, base_url: str = "http://localhost:8000"):
        self.base_url = base_url
        self.test_results = []
    
    def _log_result(self, test_name: str, passed: bool, message: str = ""):
        """Log test result"""
        status = "✅ PASS" if passed else "❌ FAIL"
        self.test_results.append({
            "test": test_name,
            "passed": passed,
            "message": message
        })
        print(f"{status}: {test_name} - {message}")
    
    def test_health_check(self) -> bool:
        """Test AI backend is running"""
        try:
            response = requests.get(f"{self.base_url}/health", timeout=5)
            passed = response.status_code == 200
            self._log_result("Health Check", passed, 
                f"Status: {response.status_code}" if not passed else "Service is healthy")
            return passed
        except Exception as e:
            self._log_result("Health Check", False, str(e))
            return False
    
    def test_risk_assessment(self) -> bool:
        """Test AI risk assessment"""
        data = {
            "user_id": "test_user_001",
            "age": 35,
            "health_history": ["hypertension", "diabetes"],
            "vehicle_condition": {"mileage": 50000, "year": 2020}
        }
        
        try:
            response = requests.post(
                f"{self.base_url}/assess_risk",
                json=data,
                timeout=10
            )
            
            if response.status_code == 200:
                result = response.json()
                passed = "risk_score" in result and "risk_level" in result
                self._log_result("Risk Assessment", passed,
                    f"Score: {result.get('risk_score', 'N/A')}" if passed else "Invalid response")
                return passed
            else:
                self._log_result("Risk Assessment", False, f"Status: {response.status_code}")
                return False
        except Exception as e:
            self._log_result("Risk Assessment", False, str(e))
            return False
    
    def test_fraud_detection(self) -> bool:
        """Test fraud detection"""
        data = {
            "transaction_id": "txn_test_001",
            "amount": 5000.0,
            "gps_coordinates": {"lat": 15.3694, "lng": 44.1910},
            "timestamp": time.time()
        }
        
        try:
            response = requests.post(
                f"{self.base_url}/detect_fraud",
                json=data,
                timeout=10
            )
            
            if response.status_code == 200:
                result = response.json()
                passed = "fraud_score" in result and "is_fraudulent" in result
                self._log_result("Fraud Detection", passed,
                    f"Score: {result.get('fraud_score', 'N/A')}" if passed else "Invalid response")
                return passed
            else:
                self._log_result("Fraud Detection", False, f"Status: {response.status_code}")
                return False
        except Exception as e:
            self._log_result("Fraud Detection", False, str(e))
            return False
    
    def test_model_status(self) -> bool:
        """Test model status endpoint"""
        try:
            response = requests.get(f"{self.base_url}/models/status", timeout=5)
            passed = response.status_code == 200
            self._log_result("Model Status", passed,
                "Models loaded" if passed else f"Status: {response.status_code}")
            return passed
        except Exception as e:
            self._log_result("Model Status", False, str(e))
            return False
    
    def run_all_tests(self) -> bool:
        """Run all integration tests"""
        print("🧪 Be-well Integration Tests")
        print("=" * 60)
        
        tests = [
            self.test_health_check,
            self.test_risk_assessment,
            self.test_fraud_detection,
            self.test_model_status
        ]
        
        passed = 0
        for test in tests:
            if test():
                passed += 1
        
        print("=" * 60)
        print(f"✅ Tests passed: {passed}/{len(tests)}")
        
        if passed == len(tests):
            print("🎉 All integration tests passed!")
            return True
        else:
            print("❌ Some tests failed!")
            return False

def main():
    tester = BeWellTester()
    success = tester.run_all_tests()
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()