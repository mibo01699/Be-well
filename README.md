Be-well: Decentralized Multi-Layered Insurance Platform

Be-well is a global, comprehensive, decentralized insurance ecosystem built on the Pi Network (Protocol 23 / Soroban Smart Contracts). Powered by self-developed artificial intelligence and complex multi-layered cryptographic verification, Be-well revolutionizes health and transport insurance by shifting risk management into real-time deterministic agreements.

🚀 Key Features

· Multi-Layered Governance: Complete segregation of duties across 4 distinct identity profiles (KYC, KYB, KYG).
· Predictive AI Risk Engine: On-the-fly premium adjustments based on historical bio-data anomalies and vehicle technical degradation.
· Deterministic Claims Settlement: Smart contracts utilizing real-time GPS coordinates and time-locks to eliminate traditional insurance fraud.
· Investment Shares (Liquidity Pools): Tokenized reserves allowing Pioneers to stake Pi and harvest dynamic programmatic yields.

📂 Repository Structure

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

🛠️ Tech Stack

· Blockchain Layer: Rust, WebAssembly (WASM), Soroban SDK (Pi Network Protocol 23).
· AI/Backend Layer: Python 3.11, FastAPI, Scikit-Learn, PyTorch.
· Frontend Environment: React Native, Pi SDK (Pi Browser Isolation).

🔒 Security & Privacy

To adhere to Pi Network privacy requirements, raw biometric inputs (Face ID, fingerprints, medical logs) are processed locally on the client-side. The blockchain contract validates state transitions exclusively via cryptographic hashes (IPFS Hash) submitted by audited entities.

---

Be-Well: The Decentralized Comprehensive Insurance Revolution on Pi Network

Be-Well is the first comprehensive decentralized insurance platform on the Pi blockchain, integrating smart contracts, artificial intelligence, and multi-level authentication systems to deliver fast, transparent, and equitable insurance services.

Integrated Vision

1. Insurance as a Community Public Good: A transparent and accessible digital service available to every Pioneer in the Pi ecosystem, backed by a digitally verified identity through the network's KYC.
2. Decentralized Governance and Community Investment: The platform relies on a community investment share system, allowing Pioneers to participate in underwriting pools by staking Pi, harvesting programmatic yields, while maintaining full transparency of financial operations.
3. Smart and Flexible Compliance: The platform is designed to be compliant with local and international laws through flexible mechanisms, allowing adaptation to various legal environments while preserving the essence of decentralization.

Rigorous Protection Mechanisms

· Deterministic Settlement: Claims rely on objectively verifiable triggers (GPS, timestamps, documented sensor data), eliminating ambiguity and making payouts automatic.
· Complete Transparency: Every transaction is permanently and immutably recorded on the Pi blockchain, creating an auditable trail verifiable at any time.
· Smart Vault Contracts: The conditional lock-and-release mechanism prevents any party from accessing funds until all agreed-upon conditions are met.

Roadmap

1. Phase 1: Development of core smart contracts and authentication system.
2. Phase 2: Building the AI risk assessment and fraud detection model.
3. Phase 3: Pilot launch on the Pi Testnet.
4. Phase 4: Full Mainnet launch with activation of the investment share system.

How to Contribute

We welcome community contributions in code development, test writing, and feedback. Please refer to the CONTRIBUTING.md file for more details.

---

🧠 Orchestrator Contract – The Brain of Be-Well

The Orchestrator Contract is the central coordination point for the entire Be-Well platform. It acts as a single, unified interface for all frontend applications and manages the complex workflows that involve multiple other smart contracts (Insurance, Bidding, Payment, Shares, Vaults).

Key Responsibilities

1. Workflow Orchestration: Manages the end-to-end process from policy purchase to claim settlement.
2. State Management: Tracks the status of each process (e.g., Policy Active, Claim Submitted, Service Requested, Awarded, Settled, Disputed).
3. Atomic Operations: Ensures that all steps in a workflow are completed successfully, preventing inconsistencies.
4. Unified API: Provides a single set of functions for the frontend to interact with, abstracting the complexity of calling individual contracts.

Main Functions

· purchase_policy() – Initiates a new insurance policy.
· submit_claim_and_request_service() – Handles a claim and creates a service request.
· award_service() – Selects the winning bid and finalizes the contract.
· confirm_service_and_pay() – Confirms service completion and triggers the payment.
· raise_dispute() – Allows users to open a dispute for manual review.
· get_process_status() – Retrieves the current status of any process.

Benefits of This Architecture

· Simplified Frontend: Developers only need to interact with one contract.
· Enhanced Security: Prevents race conditions and partial completions.
· Improved Maintainability: Changes to business logic are isolated to a single contract.
· Full Traceability: Every step of a process is recorded on-chain.

---
