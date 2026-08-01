# 1. استخدام نسخة خفيفة ومستقرة من بايثون 3.11
FROM python:3.11-slim

# 2. تعيين مجلد العمل الافتراضي داخل الحاوية
WORKDIR /app

# 3. تثبيت الأدوات الأساسية للنظام لبناء مكتبات الذكاء الاصطناعي
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# 4. نسخ ملف المتطلبات البرمجية أولاً للاستفادة من الكاش
COPY requirements.txt .

# 5. تثبيت مكتبات الذكاء الاصطناعي والـ FastAPI بدون حفظ كاش لتقليل الحجم
RUN pip install --no-cache-dir -r requirements.txt

# 6. نسخ كود المحرك الذكي بالكامل إلى داخل الحاوية
COPY . .

# 7. فتح المنفذ (Port 8000) الخاص باستقبال طلبات البلوكشين
EXPOSE 8000

# 8. أمر تشغيل خادم Uvicorn لربط الذكاء الاصطناعي بمتصفح وعقود Pi Network
CMD ["uvicorn", "app:app", "--host", "0.0.0.0", "--port", "8000"]
