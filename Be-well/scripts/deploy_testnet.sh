#!/bin/bash

# ============================================================
# توجيهات التنفيذ:
# 1. تأكد من تثبيت Soroban CLI: cargo install soroban-cli
# 2. عرّف المفتاح السري: export SECRET_KEY="S..."
# 3. اجعل الملف قابلاً للتنفيذ: chmod +x deploy_testnet.sh
# 4. شغّل: ./deploy_testnet.sh
# ============================================================

set -e

echo "🚀 Starting deployment to Pi Network Testnet..."

# Set network
NETWORK="testnet"
SECRET_KEY="${SECRET_KEY:?Error: SECRET_KEY not set}"

# Build contracts
echo "📦 Building smart contracts..."
cd pi-contracts
cargo build --target wasm32-unknown-unknown --release

# Deploy Orchestrator Contract
echo "📤 Deploying Orchestrator Contract..."
ORCHESTRATOR_ID=$(soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/be_well.wasm \
    --source "$SECRET_KEY" \
    --network "$NETWORK")

echo "✅ Orchestrator Contract deployed: $ORCHESTRATOR_ID"

# Deploy Insurance Contract
echo "📤 Deploying Insurance Contract..."
INSURANCE_ID=$(soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/insurance.wasm \
    --source "$SECRET_KEY" \
    --network "$NETWORK")

echo "✅ Insurance Contract deployed: $INSURANCE_ID"

# Deploy Shares Contract
echo "📤 Deploying Shares Contract..."
SHARES_ID=$(soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/shares.wasm \
    --source "$SECRET_KEY" \
    --network "$NETWORK")

echo "✅ Shares Contract deployed: $SHARES_ID"

# Save contract addresses
echo "📝 Saving contract addresses..."
cat > deploy_addresses.txt << EOF
ORCHESTRATOR_CONTRACT=$ORCHESTRATOR_ID
INSURANCE_CONTRACT=$INSURANCE_ID
SHARES_CONTRACT=$SHARES_ID
NETWORK=$NETWORK
DEPLOYED_AT=$(date)
EOF

echo "🎉 Deployment complete! Contract addresses saved to deploy_addresses.txt"