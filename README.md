# Be-well: Decentralized Multi-Layered Insurance Platform

Be-well is a global, comprehensive, decentralized insurance ecosystem built on the Pi Network (Protocol 23 / Soroban Smart Contracts). Powered by self-developed artificial intelligence and complex multi-layered cryptographic verification, Be-well revolutionizes health and transport insurance by shifting risk management into real-time deterministic agreements.

## 🚀 Key Features

- **Multi-Layered Governance:** Complete segregation of duties across 4 distinct identity profiles (KYC, KYB, KYG).
- **Predictive AI Risk Engine:** On-the-fly premium adjustments based on historical bio-data anomalies and vehicle technical degradation.
- **Deterministic Claims Settlement:** Smart contracts utilizing real-time GPS coordinates and time-locks to eliminate traditional insurance fraud.
- **Investment Shares (Liquidity Pools):** Tokenized reserves allowing Pioneers to stake Pi and harvest dynamic programmatic yields.

## 📂 Repository Structure

```text
Be-well/
├── README.md               # Project Overview & Quick Start
├── docs/
│   └── intro.md            # Deep-dive Architecture & Core Mechanics
├── pi-contracts/           # Soroban (Rust) Smart Contracts
│   ├── src/
│   │   ├── roles.rs        # KYC/KYB/KYG Verification & Logic
│   │   ├── insurance.rs    # Health & Transport Policy Engine
│   │   └── shares.rs       # Liquidity Pool & Profit Distribution
│   └── Cargo.toml
└── ai-backend/             # Self-developed AI Off-chain Infrastructure
    ├── app.py              # FastAPI Risk & Fraud Assessment Gateway
    └── requirements.txt
```

## 🛠️ Tech Stack

- **Blockchain Layer:** Rust, WebAssembly (WASM), Soroban SDK (Pi Network Protocol 23).
- **AI/Backend Layer:** Python 3.11, FastAPI, Scikit-Learn, PyTorch.
- **Frontend Environment:** React Native, Pi SDK (Pi Browser Isolation).

## 🔒 Security & Privacy

To adhere to Pi Network privacy requirements, raw biometric inputs (Face ID, fingerprints, medical logs) are processed locally on the client-side. The blockchain contract validates state transitions exclusively via cryptographic hashes (`IPFS Hash`) submitted by audited entities.

# Be-Well: ثورة التأمين الشامل اللامركزي على Pi Network

**Be-Well** هي أول منصة تأمين شامل لا مركزية على بلوكشين Pi، تدمج العقود الذكية، الذكاء الاصطناعي، وأنظمة التوثيق متعددة المستويات لتقديم خدمات تأمينية سريعة وشفافة وعادلة.

## الرؤية المتكاملة

1. **التأمين كمنفعة مجتمعية:** خدمة رقمية شفافة وميسرة، متاحة لكل رائد في نظام Pi البيئي، مدعومة بهوية رقمية موثقة عبر KYC الخاص بالشبكة.

2. **الحوكمة اللامركزية والاستثمار المجتمعي:** تعتمد المنصة على نظام أسهم استثمارية مجتمعية، حيث يمكن للرواد المشاركة في تمويل المجمعات الاكتتابية عبر حجز Pi، وجني عوائد برمجية، مع الحفاظ على شفافية العمليات المالية.

3. **الامتثال الذكي والمرن:** صُممت المنصة لتكون متوافقة مع القوانين المحلية والدولية عبر آليات مرنة، تسمح بالتكيف مع مختلف البيئات القانونية مع الحفاظ على جوهر اللامركزية.

## آليات الحماية الصارمة

- **التسوية الحتمية:** تعتمد المطالبات على محفزات موضوعية يمكن التحقق منها (GPS، طوابع زمنية، بيانات أجهزة استشعار موثقة)، مما يزيل الغموض ويجعل الدفع تلقائياً.
- **الشفافية الكاملة:** كل عملية تُسجل بشكل دائم وغير قابل للتغيير على بلوكشين Pi، مما يخلق مسار تدقيق يمكن التحقق منه في أي وقت.
- **عقود الخزائن الذكية:** تمنع آلية القفل والإفراج المشروط وصول أي طرف إلى الأموال قبل استيفاء جميع الشروط المتفق عليها.

## خريطة الطريق

1. **المرحلة 1:** تطوير العقود الذكية الأساسية ونظام التوثيق.
2. **المرحلة 2:** بناء نموذج الذكاء الاصطناعي لتقييم المخاطر والكشف عن الاحتيال.
3. **المرحلة 3:** الإطلاق التجريبي على شبكة Pi Testnet.
4. **المرحلة 4:** الإطلاق الكامل على الشبكة الرئيسية (Mainnet) مع تفعيل نظام الأسهم الاستثمارية.

## كيفية المساهمة

نرحب بمساهمات المجتمع في تطوير الأكواد، كتابة الاختبارات، وتقديم الملاحظات. راجع ملف `CONTRIBUTING.md` للمزيد من التفاصيل.

## 🧠 Orchestrator Contract – The Brain of Be-Well

The **Orchestrator Contract** is the central coordination point for the entire Be-Well platform. It acts as a single, unified interface for all frontend applications and manages the complex workflows that involve multiple other smart contracts (Insurance, Bidding, Payment, Shares, Vaults).

### Key Responsibilities

1.  **Workflow Orchestration:** Manages the end-to-end process from policy purchase to claim settlement.
2.  **State Management:** Tracks the status of each process (e.g., Policy Active, Claim Submitted, Service Requested, Awarded, Settled, Disputed).
3.  **Atomic Operations:** Ensures that all steps in a workflow are completed successfully, preventing inconsistencies.
4.  **Unified API:** Provides a single set of functions for the frontend to interact with, abstracting the complexity of calling individual contracts.

### Main Functions

- `purchase_policy()` – Initiates a new insurance policy.
- `submit_claim_and_request_service()` – Handles a claim and creates a service request.
- `award_service()` – Selects the winning bid and finalizes the contract.
- `confirm_service_and_pay()` – Confirms service completion and triggers the payment.
- `raise_dispute()` – Allows users to open a dispute for manual review.
- `get_process_status()` – Retrieves the current status of any process.

### Benefits of This Architecture

- **Simplified Frontend:** Developers only need to interact with one contract.
- **Enhanced Security:** Prevents race conditions and partial completions.
- **Improved Maintainability:** Changes to business logic are isolated to a single contract.
- **Full Traceability:** Every step of a process is recorded on-chain.
