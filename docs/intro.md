# System Architecture & Technical Deep-Dive

This document outlines the programmatic enforcement mechanisms of Be-well, detailing the identity framework, AI integration, and core execution loops.

## 👥 Quad-Role Identity Architecture

Be-well expands standard Pi KYC into a specialized, multi-layered validation hierarchy:

1. **Individual Accounts (KYC):** Mandatorily bound to the unique Pi Username. Contains encrypted strings representing blood type, chronic/genetic predispositions, and household coordinates.
2. **Case Evaluator Accounts (KYB):** Audited corporate nodes authorized to push damage reports, technical survey scores, or clinical diagnostics directly to the blockchain state.
3. **Restoration & Remedy Nodes (KYB):** Workshops, medical institutions, and repair facilities. The smart contract disburses Pi tokens directly to these nodes upon milestone verification, minimizing individual liquidity diversion fraud.
4. **Professional & Legal Auditors (KYG):** High-privilege entities empowered with programmatic veto and freeze capabilities in the event of AI-flagged collusive fraud patterns.

## 🧠 Real-time AI Risk Mitigation

Premium computations are dynamic and non-linear. The off-chain self-developed AI model executes the following formula constraints:

### Health Risk Formula Matrix

Premium_{Health} = Base_{Rate} × F(Age, ChronicFactor, BioAnomalyScore)

### Transport Depreciation Loop

Every vehicle artifact is registered with a `Technical Condition Index (TCI)` ranging from 1-100. As negative indicators increase, the TCI drops, and the Soroban contract dynamically recalibrates the premium value for the subsequent epoch.

## 📍 Spatial-Temporal Smart Contract Enforcement

Claims verification requires absolute cryptographic alignment of:
- **Timestamping:** Decentralized time-clocks preventing retroactive claim logging.
- **GPS Coordinates:** Geofencing validation. If an incident report falls outside the geofenced perimeter declared inside the policy state, the contract triggers an automatic execution abort.
