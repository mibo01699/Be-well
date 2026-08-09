# ============================================================
# الملف: risk_engine.py
# المسار: Be-well/ai-backend/risk_engine.py
# الغرض: محرك تقييم المخاطر التنبئي (نموذج أولي)
# ============================================================

import pandas as pd
import numpy as np
from sklearn.ensemble import RandomForestClassifier
from sklearn.model_selection import train_test_split
from sklearn.metrics import accuracy_score
import joblib
import os

class RiskEngine:
    def __init__(self):
        self.model = None
        self.model_path = "risk_model.pkl"
        # إنشاء بيانات تدريب وهمية (سيتم استبدالها ببيانات حقيقية)
        self.training_data = self._generate_sample_data()

    def _generate_sample_data(self):
        """توليد بيانات تدريب وهمية للاختبار الأولي"""
        np.random.seed(42)
        n_samples = 1000
        # الميزات: العمر، الجنس، تاريخ مرضي، نوع المركبة، المنطقة، إلخ.
        age = np.random.randint(18, 70, n_samples)
        gender = np.random.randint(0, 2, n_samples)
        pre_existing_conditions = np.random.randint(0, 2, n_samples)
        vehicle_type = np.random.randint(0, 5, n_samples)  # 0: سيارة, 1: دراجة, إلخ.
        region = np.random.randint(0, 5, n_samples)  # 0: آمن, 4: عالي المخاطر

        # حساب درجة المخاطرة (نتيجة وهمية)
        risk_score = (age / 100) + (pre_existing_conditions * 0.3) + (vehicle_type * 0.1) + (region * 0.2)
        risk_score = np.clip(risk_score, 0, 1)  # تطبيع إلى 0-1
        risk_label = (risk_score > 0.5).astype(int)  # 1: عالي المخاطرة, 0: منخفض

        data = pd.DataFrame({
            'age': age,
            'gender': gender,
            'pre_existing': pre_existing_conditions,
            'vehicle_type': vehicle_type,
            'region': region,
            'risk_label': risk_label
        })
        return data

    def train_model(self):
        """تدريب نموذج التصنيف"""
        X = self.training_data.drop('risk_label', axis=1)
        y = self.training_data['risk_label']
        X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)

        self.model = RandomForestClassifier(n_estimators=100, random_state=42)
        self.model.fit(X_train, y_train)

        # تقييم النموذج
        y_pred = self.model.predict(X_test)
        accuracy = accuracy_score(y_test, y_pred)
        print(f"✅ دقة النموذج الأولي: {accuracy:.2f}")

        # حفظ النموذج
        joblib.dump(self.model, self.model_path)
        return accuracy

    def predict_risk(self, features):
        """تقدير المخاطرة بناءً على ميزات المدخل"""
        if self.model is None:
            if os.path.exists(self.model_path):
                self.model = joblib.load(self.model_path)
            else:
                raise Exception("النموذج غير موجود. قم بتدريبه أولاً.")
        
        # تحويل المدخلات إلى DataFrame
        input_df = pd.DataFrame([features])
        risk_score = self.model.predict_proba(input_df)[0][1]  # احتمالية المخاطرة العالية
        return risk_score

    def load_model(self):
        """تحميل نموذج مدرب من القرص"""
        if os.path.exists(self.model_path):
            self.model = joblib.load(self.model_path)
            return True
        return False

# وحدة مستقلة لتشغيل النموذج (للاختبار)
if __name__ == "__main__":
    engine = RiskEngine()
    engine.train_model()
    
    # اختبار نموذج على بيانات جديدة
    sample_features = {
        'age': 45,
        'gender': 1,
        'pre_existing': 1,
        'vehicle_type': 2,
        'region': 3
    }
    risk = engine.predict_risk(sample_features)
    print(f"⚡ درجة المخاطرة المقدرة: {risk:.2f}")