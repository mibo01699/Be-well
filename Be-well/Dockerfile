# ============================================================
# توجيهات التنفيذ:
# 1. بناء الصورة: docker build -t be-well .
# 2. تشغيل الحاوية: docker run -p 8000:8000 be-well
# ============================================================

FROM python:3.11-slim

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    gcc \
    && rm -rf /var/lib/apt/lists/*

# Copy and install Python dependencies
COPY ai-backend/requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy application code
COPY ai-backend/ .

# Expose port
EXPOSE 8000

# Run the application
CMD ["uvicorn", "app:app", "--host", "0.0.0.0", "--port", "8000"]