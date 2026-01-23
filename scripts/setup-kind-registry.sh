#!/bin/bash
# Setup local Docker registry for Kind (localhost:5001).
#
# Run after "kind create cluster". Ensures the kind-registry container exists,
# is running, and is connected to the kind network so:
#   - the host can push images to localhost:5001/<image>:<tag>
#   - cluster nodes can pull via the containerd mirror (kind-config.yaml maps
#     localhost:5000/localhost:5001 -> http://kind-registry:5000)
#
# See: https://kind.sigs.k8s.io/docs/user/local-registry/
#
# To remove the registry: docker stop kind-registry && docker rm kind-registry

set -euo pipefail

REG_NAME='kind-registry'
REG_PORT='5001'

# 1. Create registry container if it doesn't exist; start it if it's stopped
if ! docker inspect "$REG_NAME" &>/dev/null; then
  echo "📦 Creating local registry: $REG_NAME (host port $REG_PORT)"
  docker run -d --restart=always \
    -p "127.0.0.1:${REG_PORT}:5000" \
    --network bridge \
    --name "$REG_NAME" \
    registry:2
  echo "   Created and started $REG_NAME"
elif [ "$(docker inspect -f '{{.State.Running}}' "$REG_NAME" 2>/dev/null)" != "true" ]; then
  echo "📦 Starting existing registry: $REG_NAME"
  docker start "$REG_NAME"
  echo "   Started $REG_NAME"
else
  echo "📦 Registry already running: $REG_NAME"
fi

# 2. Connect the registry to the kind network (cluster nodes reach it as kind-registry:5000)
if ! docker network inspect kind &>/dev/null; then
  echo "⚠️  Docker network 'kind' not found. Create a Kind cluster first:"
  echo "   kind create cluster --config kind-config.yaml"
  exit 1
fi

if [ "$(docker inspect -f '{{json .NetworkSettings.Networks.kind}}' "$REG_NAME" 2>/dev/null)" = "null" ]; then
  echo "🔗 Connecting $REG_NAME to Docker network 'kind'..."
  docker network connect kind "$REG_NAME"
  echo "   Connected $REG_NAME to kind"
else
  echo "🔗 Registry already on kind network"
fi

# 3. Document the local registry (optional; for tools that read localRegistryHosting)
if kubectl cluster-info &>/dev/null; then
  cat <<EOF | kubectl apply -f - 2>/dev/null || true
apiVersion: v1
kind: ConfigMap
metadata:
  name: local-registry-hosting
  namespace: kube-public
data:
  localRegistryHosting.v1: |
    host: "localhost:${REG_PORT}"
    help: "https://kind.sigs.k8s.io/docs/user/local-registry/"
EOF
fi

echo "✅ Local registry ready: push images to localhost:${REG_PORT}/<image>:<tag>"
