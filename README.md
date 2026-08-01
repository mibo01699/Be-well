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
