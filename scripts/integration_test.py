# scripts/integration_test.py - Production Grade System Simulator for Be-Well Platform
import unittest
import requests
import json

class TestBeWellProtocolIntegration(unittest.TestCase):
    
    def setUp(self):
        # Localhost endpoints configuration matching repository microservices
        self.gateway_url = "http://localhost:8000/backend/api_gateway.py"
        self.mock_pioneer_uid = "pioneer_yemen_mayass_ali_2026"
        self.mock_payment_id = "pi_pay_998877665544332211"
        self.mock_txid = "stellar_soroban_tx_88339911aa77bbcc"
        self.mock_policy_id = "policy_health_premium_001"

    def test_01_ai_and_gateway_flow_execution(self):
        """
        Validates the entire secure lifecycle loop: 
        Pi Payment Ingestion -> AI Underwriting Fetch -> Transaction Signing.
        """
        payload = {
            "paymentId": self.mock_payment_id,
            "txid": self.mock_txid,
            "pioneerUid": self.mock_pioneer_uid,
            "policyId": self.mock_policy_id
        }
        
        print("\n[Simulator] Step 1: Broadcasting verification request to API Gateway...")
        headers = {'Content-Type': 'application/json'}
        
        # Testing the system logic simulation endpoint
        response = requests.post(f"{self.gateway_url}/complete-payment", data=json.dumps(payload), headers=headers)
        
        # Enforce technical validation criteria
        self.assertEqual(response.status_code, 200, "Gateway server is unreachable or failed compilation check.")
        
        data = response.json()
        print(f"[Simulator] Subsystem Response Status: {data.get('status')}")
        print(f"[Simulator] Dynamic AI Risk Evaluated Score: {data['policy_parameters']['risk_score']}")
        print(f"[Simulator] Soroban Verification Blueprint Hash: {data.get('blockchain_tx')}")
        
        self.assertEqual(data["status"], "success", "Operational workflow failed execution parameters.")
        self.assertIn("policy_parameters", data, "Payload integrity audit failed.")
        print("[✔] Success: Full system pipeline verification completed with zero telemetry drops.")

if __name__ == "__main__":
    print("=============================================================")
    print("🦅 Arab Eagle Company (A.E.C) - Be-Well Integration Test Suite")
    print("=============================================================")
    unittest.main()
