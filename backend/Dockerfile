# ============================================================
# الملف: Dockerfile (AI Backend)
# المسار: Be-well/ai-backend/Dockerfile
# الغرض: بناء حاوية لتشغيل الـ AI Backend
# ============================================================

FROM python:3.11-slim

WORKDIR /app

# نسخ ملفات المتطلبات
COPY requirements.txt .

# تثبيت التبعيات
RUN pip install --no-cache-dir -r requirements.txt

# نسخ الكود المصدري
COPY . .

# فتح المنفذ
EXPOSE 8000

# تشغيل التطبيق
CMD ["uvicorn", "app:app", "--host", "0.0.0.0", "--port", "8000"]

# ============================================================
# الملف: Dockerfile (API Gateway)
# المسار: Be-well/backend/Dockerfile
# الغرض: بناء حاوية لتشغيل الـ API Gateway
# ============================================================

FROM python:3.11-slim

WORKDIR /app

# نسخ ملفات المتطلبات
COPY requirements.txt .

# تثبيت التبعيات
RUN pip install --no-cache-dir -r requirements.txt

# نسخ الكود المصدري
COPY . .

# فتح المنفذ
EXPOSE 8080

# تشغيل التطبيق
CMD ["uvicorn", "api_gateway:app", "--host", "0.0.0.0", "--port", "8080"]