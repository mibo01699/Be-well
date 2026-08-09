# Be-Well: منصة التأمين الشامل اللامركزي على Pi Network

[![Pi Network](https://img.shields.io/badge/Pi%20Network-Protocol%2025-blue)](https://pinetwork.com)
[![Soroban](https://img.shields.io/badge/Soroban-Rust-orange)](https://soroban.stellar.org)
[![MIT License](https://img.shields.io/badge/License-MIT-green)](LICENSE)

**Be-Well** هي أول منصة تأمين شامل لا مركزية على بلوكشين Pi، تدمج العقود الذكية، الذكاء الاصطناعي، وأنظمة التوثيق متعددة المستويات لتقديم خدمات تأمينية سريعة وشفافة وعادلة.

---

## 📖 نظرة عامة

- **التأمين كمنفعة مجتمعية:** خدمة رقمية شفافة وميسرة، متاحة لكل رائد في نظام Pi البيئي.
- **الحوكمة اللامركزية والاستثمار المجتمعي:** نظام أسهم استثمارية يشارك فيه المجتمع في تمويل وإدارة المنصة.
- **الامتثال الذكي والمرن:** متوافقة مع القوانين المحلية والدولية عبر آليات مرنة.

**للاطلاع على الرؤية المتكاملة والتفاصيل التقنية، راجع:** [الورقة البيضاء (Whitepaper)](docs/WHITEPAPER.md)

---

## 🚀 البدء السريع

### المتطلبات الأساسية

- Rust 1.75+
- Soroban CLI 20.0.0+
- Python 3.11+
- Docker (اختياري)

### التثبيت والتشغيل

1.  **نسخ المستودع:**
    ```bash
    git clone https://github.com/mibo01699/Be-well.git
    cd Be-well
    ```

2.  **بناء العقود الذكية:**
    ```bash
    cd pi-contracts
    cargo build --target wasm32-unknown-unknown --release
    ```

3.  **تشغيل الـ AI Backend:**
    ```bash
    cd ../ai-backend
    pip install -r requirements.txt
    python app.py
    ```

4.  **تشغيل الـ API Gateway:**
    ```bash
    cd ../backend
    pip install -r requirements.txt
    python api_gateway.py
    ```

5.  **فتح الواجهة الأمامية:**
    افتح ملف `frontend/index.html` في متصفحك.

---

## 📂 هيكل المستودع

```

Be-well/
├── pi-contracts/           # العقود الذكية (Rust/Soroban)
│   ├── src/
│   │   ├── orchestrator.rs # عقد التنسيق المركزي
│   │   ├── insurance.rs    # عقد إدارة وثائق التأمين
│   │   ├── shares.rs       # عقد الأسهم الاستثمارية
│   │   └── roles.rs        # منطق التوثيق (KYC/KYB/KYG)
│   └── tests/              # اختبارات العقود
├── ai-backend/             # نظام الذكاء الاصطناعي الخلفي
│   ├── app.py              # واجهة برمجة تطبيقات (API) FastAPI
│   ├── risk_engine.py      # محرك تقييم المخاطر
│   └── requirements.txt    # تبعيات Python
├── backend/                # واجهة برمجة تطبيقات (API) الوسيطة
│   ├── api_gateway.py      # يربط AI بالعقود الذكية
│   └── requirements.txt
├── frontend/               # النموذج الأولي للواجهة الأمامية
│   ├── index.html
│   ├── style.css
│   └── app.js
├── docs/                   # الوثائق
│   ├── WHITEPAPER.md       # الورقة البيضاء
│   ├── BUSINESS_PLAN.md    # خطة العمل
│   └── intro.md            # شرح الهندسة المعمارية
├── scripts/                # نصوص مساعدة
│   └── integration_test.py # اختبارات التكامل
├── CONTRIBUTING.md         # دليل المساهمة
├── LICENSE                 # رخصة المشروع (MIT)
└── README.md               # هذا الملف

```

---

## 🛠️ التقنيات المستخدمة

- **طبقة البلوكشين:** Rust, WebAssembly (WASM), Soroban SDK (Pi Network Protocol 23).
- **طبقة الذكاء الاصطناعي/الخلفية:** Python 3.11, FastAPI, Scikit-Learn, PyTorch.
- **بيئة الواجهة الأمامية:** React Native (مستقبلاً), Pi SDK.

---

## 🧪 الاختبارات

لتشغيل اختبارات العقود الذكية:
```bash
cd pi-contracts
cargo test
```

لتشغيل اختبارات التكامل (تتطلب تشغيل الخدمات محلياً):

```bash
python scripts/integration_test.py
```

---

🤝 المساهمة

نرحب بمساهمات المجتمع! يرجى قراءة دليل المساهمة لمزيد من التفاصيل حول كيفية المشاركة.

---

📄 الترخيص

هذا المشروع مرخص تحت رخصة MIT.

---

📬 التواصل

للاستفسارات، يرجى فتح مشكلة (Issue) على GitHub أو التواصل عبر قنوات التواصل الاجتماعي المخصصة.