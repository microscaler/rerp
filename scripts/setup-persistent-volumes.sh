#!/bin/bash
# Setup PersistentVolumes for RERP
# This script should be run after the Kind cluster is created but before Tilt starts
# PersistentVolumes are excluded from Tilt management to prevent deletion/recreation issues

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "🔧 Setting up PersistentVolumes for RERP..."
echo ""

# Check if kubectl can connect to the cluster
if ! kubectl cluster-info &>/dev/null; then
    echo "❌ Error: Cannot connect to Kubernetes cluster"
    echo "   Please ensure your Kind cluster is running: kind get clusters"
    exit 1
fi

# Apply data PersistentVolumes (if they exist)
if [ -f "$PROJECT_ROOT/k8s/data/persistent-volumes.yaml" ]; then
    echo "📦 Creating data PersistentVolumes..."
    kubectl apply -f "$PROJECT_ROOT/k8s/data/persistent-volumes.yaml" || {
        echo "⚠️  Warning: Some data PVs may already exist (this is OK)"
    }
else
    echo "ℹ️  No data PersistentVolumes file found (this is OK for initial setup)"
fi

# Apply monitoring PersistentVolumes (if they exist)
if [ -f "$PROJECT_ROOT/k8s/monitoring/persistent-volumes.yaml" ]; then
    echo "📦 Creating monitoring PersistentVolumes..."
    kubectl apply -f "$PROJECT_ROOT/k8s/monitoring/persistent-volumes.yaml" || {
        echo "⚠️  Warning: Some monitoring PVs may already exist (this is OK)"
    }
else
    echo "ℹ️  No monitoring PersistentVolumes file found (this is OK for initial setup)"
fi

echo ""
echo "✅ PersistentVolumes setup complete!"
echo ""
echo "📊 Current PersistentVolumes:"
kubectl get pv 2>/dev/null || echo "No PersistentVolumes found"

echo ""
echo "💡 Note: These PVs are not managed by Tilt to prevent deletion/recreation issues."
echo "   If you need to recreate them, delete the cluster and run this script again."
