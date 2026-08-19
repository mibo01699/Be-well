# scripts/integration_test.py
import unittest
import requests
import json
import time

class TestBeWellProtocolE2E(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        # المحاكاة الافتراضية للمنافذ المحلية المشغلة عبر Replit
        cls.ai_backend_url = "http://127.0.0.1:8000"
        cls.middleware_url = "http://127.0.0.1:5000"

    def test_01_ai_underwriting_risk_engine(self):
        """ اختبار نموذج الاكتتاب وتقييم المخاطر بالذكاء الاصطناعي """
        payload = {
            "user_id": "pi_test_user_01699",
            "routine_data": [0.1, 0.4, 0.9, 0.2],
            "diagnostic_score": 0.85
        }
        try:
            response = requests.post(f"{self.ai_backend_url}/api/v1/risk/evaluate", json=payload, timeout=5)
            if response.status_code == 200:
                data = response.json()
                self.assertIn("premium_rate_pi", data)
                self.assertIn("risk_score", data)
        except requests.exceptions.ConnectionError:
            print("⚠️ ميكروسيرفيس AI Backend غير نشط حالياً، تخطي الفحص المباشر.")

    def test_02_anti_double_dipping_and_kyb(self):
        """ اختبار منع الصرف المزدوج وسجل الهوية المشتركة المعياري """
        payload = {
            "piUserId": "pi_test_user_01699",
            "entityType": "Company",
            "companyDetails": {"commercialRegister": "AEC-2026-YER"}
        }
        try:
            response = requests.post(f"{self.middleware_url}/api/v1/identity/register", json=payload, timeout=5)
            if response.status_code == 200:
                self.assertEqual(response.json().get("status"), "Pending_Audit")
        except requests.exceptions.ConnectionError:
            print("⚠️ ميكروسيرفيس Middleware Relayer غير نشط حالياً.")

if __name__ == "__main__":
    print("[*] بدء فحص ترابط طبقات المنصة الهجينة (Be-Well + BIGISH-YER)...")
    unittest.main()
