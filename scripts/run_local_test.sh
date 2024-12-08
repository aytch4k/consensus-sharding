#!/bin/bash

# Run local test environment for shard prototype

# Deploy Docker containers
echo "Deploying Docker containers..."
bash scripts/deploy_docker.sh

# Wait for containers to initialize
echo "Waiting for services to start..."
sleep 10

# Run test transactions
echo "Running test transactions..."
python3 scripts/load_test_transactions.py

echo "Local test complete. Check logs and metrics for results."
