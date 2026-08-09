# Contributing to Be-Well Protocol 🦅
**Developer Guidelines and Code Governance Framework for the Arab Eagle Company (A.E.C) Ecosystem**

Thank you for your interest in contributing to the **Be-Well Protocol**, the first decentralized parametric insurance utility platform built on Soroban-WASM and AI runtime modules. We welcome open-source developers, blockchain architects, and AI researchers to collaborate with us.

By contributing to this repository, you help build a non-speculative financial safety net for millions of Web3 Pioneers globally.

---

## 1. Code of Conduct and Ecosystem Alignment
The Be-Well Protocol strictly follows a **zero-speculation utility policy**. All contributions must focus entirely on optimizing transaction throughput, lowering gas instruction footprints, enhancing predictive accuracy, and securing the on-chain infrastructure. We do not accept contributions that introduce secondary speculative tokenomics, inflationary mechanisms, or external third-party asset dependencies.

---

## 2. Core Development Workflow

To maintain production-grade architecture and clear continuous integration (CI) pipelines, please follow this strict workflow:

### Step 1: Fork and Clone
1. Fork the official repository under `https://github.com`.
2. Clone your fork locally:
   ```bash
   git clone https://github.com
   cd Be-well
   ```

### Step 2: Establish an Isolated Feature Branch
Never commit directly to the `main` branch. Create a descriptive feature branch:
```bash
git checkout -b feature/optimize-soroban-insurance-gas
# OR
git checkout -b fix/ai-backend-latency
```

### Step 3: Enforce Coding Standards
Your modifications must pass our rigorous testing suites before submission:

*   **On-Chain Smart Contracts (`pi-contracts/`):** Code must be written in standard, high-efficiency Rust conforming to the latest Soroban SDK compilation rules. Run the built-in formatter and linter:
    ```bash
    cargo fmt --all
    cargo clippy --all-targets -- -D warnings
    ```
*   **AI Backend Subsystems (`ai-backend/`):** Python scripts must follow strict PEP 8 formatting rules. Do not include static keys, biometric logging arrays, or unverified mathematical variables inside the PyTorch routing structures.

---

## 3. Strict Testing Obligations
Every single pull request (PR) that alters functional system states must pass all automated verification tests without telemetry degradation.

1.  **Execute Local Unit Tests for Soroban Smart Contracts:**
    ```bash
    cd pi-contracts
    cargo test
    ```
2.  **Execute End-to-End System Integration Tests:**
    Ensure both the FastAPI backend and middleware servers are active, then run:
    ```bash
    python scripts/integration_test.py
    ```

---

## 4. Submission & Architectural Peer Review
1. Commit your changes using clean, clear structural terminology:
   ```bash
   git commit -m "feat(contracts): optimize persistence storage bounds in insurance.rs"
   ```
2. Push your changes to your fork and open a **Pull Request (PR)** against our main development branch.
3. Every PR requires formal review from the **Arab Eagle Company (A.E.C)** technical committee led by **Mayass Ali**. Be prepared to provide optimization metrics regarding WASM byte sizes or execution instruction allocations during the engineering review cycles.

---

## 5. Software Licensing & PiOS Framework Compliance
The Be-Well Protocol is explicitly committed to open-source public good innovation. 
* By submitting code to this repository, you agree that your contributions will be bound by dual-licensing parameters including the standard **MIT License** and the specialized **Pi Open Source (PiOS)** framework registry rules.
* This ensures that all code enhancements remain legally protected as a collaborative ecosystem utility asset, prohibiting centralized monetization or proprietary code concealment.
