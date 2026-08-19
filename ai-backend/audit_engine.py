# ai-backend/audit_engine.py
import time
import requests

class AuditEngine:
    def __init__(self, pi_dex_url):
        self.pi_dex_url = pi_dex_url

    def calculate_monthly_payout(self, cases_count, complexity_score, accuracy_rate, market_rate_yer):
        """ احتساب مكافأة المدقق بناءً على الأداء ومتوسط أسعار السوق """
        if accuracy_rate < 0.85:
            # خصم أو تجميد في حال تدني الدقة عن 85%
            return 0
        
        base_pay = cases_count * market_rate_yer
        performance_bonus = complexity_score * 1.5
        total_yer = base_pay + performance_bonus
        return total_yer

    def settle_auditor_payout_instant(self, auditor_wallet, total_yer):
        """ جلب سعر الصرف ولحظية تحويل السعر من Pi إلى YER للصرف """
        try:
            # استعلام سعر الصرف اللحظي من مجمع سيولة AMM
            response = requests.get(f"{self.pi_dex_url}/api/v1/price?pair=Pi_YER")
            pi_per_yer = response.json().get("price")
            
            pi_amount_needed = total_yer * pi_per_yer
            
            # تنفيذ عملية الـ Swap والصرف الفوري للمحفظة الرقمية للمدقق
            payload = {
                "to": auditor_wallet,
                "amount_yer": total_yer,
                "pi_equivalent": pi_amount_needed,
                "timestamp": int(time.time())
            }
            settle_res = requests.post(f"{self.pi_dex_url}/api/v1/clearing/payout", json=payload)
            return settle_res.json().get("status") == "success"
        except Exception as e:
            print(f"Error during clearing and settlement for auditor: {e}")
            return False
