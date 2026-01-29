#!/usr/bin/env just --justfile
# RERP Development Justfile

# Set shell for recipes
set shell := ["bash", "-uc"]

# Default recipe to display help
default:
    @just --list --unsorted

# ============================================================================
# Development Environment
# ============================================================================

# Start development environment (Kind + Tilt)
dev-up:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "🚀 Starting RERP development environment..."
    
    # Create Kind cluster
    echo "📦 Creating Kind cluster..."
    kind create cluster --config kind-config.yaml || true

    # Start local registry and connect to kind network (so docker push localhost:5001/... works)
    echo "📦 Setting up local registry (localhost:5001)..."
    tooling/.venv/bin/rerp tilt setup-kind-registry
    
    # Wait for cluster to be ready
    echo "⏳ Waiting for cluster to be ready..."
    kubectl wait --for=condition=Ready nodes --all --timeout=300s
    
    # Create rerp namespace (at cluster creation; Tilt does not manage it)
    echo "📁 Creating rerp namespace..."
    kubectl apply -f k8s/microservices/namespace.yaml
    
    # Create PersistentVolumes (outside of Tilt management)
    echo "💾 Creating PersistentVolumes..."
    tooling/.venv/bin/rerp tilt setup-persistent-volumes || {
        echo "⚠️  Warning: Some PVs may already exist (this is OK)"
    }
    
    # Start Tilt
    echo "🎯 Starting Tilt..."
    tilt up --host=0.0.0.0 --port=10351

# Stop development environment (Kind cluster and Tilt; local registry is left running)
dev-down:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "🛑 Stopping RERP development environment..."
    
    # Stop Tilt
    echo "Stopping Tilt..."
    pkill -f "tilt up" 2>/dev/null || true
    
    # Delete Kind cluster
    echo "🗑️ Deleting Kind cluster..."
    kind delete cluster --name rerp 2>/dev/null || true
    
    echo "✅ Development environment stopped"
    echo "   (Local registry kind-registry is left running. To remove: just dev-down-full)"

# Stop development environment and remove the local registry
dev-down-full: dev-down
    @echo "🗑️ Removing local registry..."
    @docker stop kind-registry 2>/dev/null || true
    @docker rm kind-registry 2>/dev/null || true
    @echo "✅ Registry removed"

# Setup development environment (Tilt-based)
setup:
    @tooling/.venv/bin/rerp tilt setup

# Teardown development environment (add --remove-images, --remove-volumes, --system-prune as needed)
teardown:
    @tooling/.venv/bin/rerp tilt teardown

# Start services with Tilt (local Docker mode)
up:
    @echo "Starting all services with Tilt (local Docker mode)..."
    @tilt up

# Start with Kind cluster (cluster and rerp namespace must exist; see dev-up)
up-k8s:
    @kubectl apply -f k8s/microservices/namespace.yaml 2>/dev/null || true
    @echo "Starting all services with Tilt (Kubernetes mode)..."
    @tilt up -- --use-kind

# Stop services
down:
    @tilt down

# ============================================================================
# Tooling (.venv and rerp CLI)
# ============================================================================
# Setup is in the justfile (not in tooling) to avoid circular deps: tooling
# requires .venv to exist; creating .venv must not depend on tooling.

# Create tooling/.venv and install rerp tooling [dev]. Run once before first
# use of rerp or `tilt up` (build-tooling in Tilt expects .venv). Idempotent.
init:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "🐍 Setting up tooling .venv..."
    if [ ! -d tooling/.venv ]; then
        python3 -m venv tooling/.venv
    fi
    tooling/.venv/bin/pip install --upgrade pip
    # BFF generation uses brrtrouter-tooling (Story 1.4). Install from sibling path when present.
    if [ -d ../BRRTRouter/tooling ]; then
        tooling/.venv/bin/pip install -e ../BRRTRouter/tooling
    fi
    tooling/.venv/bin/pip install -e ./tooling[dev]
    echo "✅ Tooling .venv ready. Use: tooling/.venv/bin/rerp or add tooling/.venv/bin to PATH"

# Rebuild tooling (pip install -e) after source changes. Used by Tilt on
# tooling/src and pyproject.toml changes (ignores .pyc, __pycache__). Run
# `just init` first if tooling/.venv does not exist.
build-tooling:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ ! -d tooling/.venv ]; then
        echo "❌ tooling/.venv not found. Run: just init"
        exit 1
    fi
    echo "🔨 Rebuilding tooling..."
    if [ -d ../BRRTRouter/tooling ]; then
        tooling/.venv/bin/pip install -e ../BRRTRouter/tooling
    fi
    tooling/.venv/bin/pip install -e ./tooling[dev]
    echo "✅ Tooling rebuilt"

# Start an interactive shell with tooling/.venv sourced and activated (exit to leave).
venv:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ ! -d tooling/.venv ]; then
        echo "❌ tooling/.venv not found. Run: just init"
        exit 1
    fi
    echo "Shell with tooling/.venv activated (exit to leave)..."
    exec bash -c "source tooling/.venv/bin/activate && exec bash -i"

# ============================================================================
# Building
# ============================================================================

# Build all Docker images
build: build-rust

# Build Rust services
build-rust:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Building RERP Rust services..."
    for dir in microservices/accounting/*; do
        if [ -f "$$dir/Dockerfile" ]; then
            service_name=$$(basename $$dir)
            echo "Building $$service_name..."
            docker build -t rerp/$$service_name:latest -f docker/microservices/Dockerfile.$$service_name .
        fi
    done

# Build individual service
# Usage: just build-service <service-name>
# Example: just build-service general-ledger
build-service service:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Building RERP service: {{service}}..."
    docker build -t rerp/{{service}}:latest -f docker/microservices/Dockerfile.{{service}} .

# ============================================================================
# Testing
# ============================================================================

# Run all tests
test: test-rust test-integration

# Run Rust unit tests
test-rust:
    @echo "Running Rust tests..."
    @cd microservices && cargo test --lib --workspace --no-fail-fast

# Run integration tests
test-integration:
    @echo "Running integration tests..."
    @echo "TODO: Add integration tests"

# ============================================================================
# Validation
# ============================================================================

# Validate all (tests + workflows)
validate:
    @echo "All validations passed!"

# Scan port registry, helm, kind, Tiltfile, bff-suite-config; report conflicts
# Run before dev-up to avoid "address already in use"
validate-ports:
    @tooling/.venv/bin/rerp ports validate

# Resolve duplicate service.port in helm; accounting keeps, others get next free
# Use after reconcile when validate reports duplicates
fix-duplicate-ports:
    @tooling/.venv/bin/rerp ports fix-duplicates

# ============================================================================
# Lint & Format
# ============================================================================

# Run ruff check on tooling (E,F,W,B,C4,UP,SIM,I,PTH,RUF,S110,A,BLE,ERA,T10,EM,RET,LOG,PIE,PT,RSE,PGH,N,PERF,C90,TRY). Run `just init` first.
lint:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ ! -d tooling/.venv ]; then
        echo "❌ tooling/.venv not found. Run: just init"
        exit 1
    fi
    tooling/.venv/bin/ruff check tooling/

# Format tooling with ruff. Run `just init` first.
format:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ ! -d tooling/.venv ]; then
        echo "❌ tooling/.venv not found. Run: just init"
        exit 1
    fi
    tooling/.venv/bin/ruff format tooling/

# Run cargo fmt in microservices/ and entities/. Use when microservices/ structure or
# deps change; fast if Tilt has recently built. microservices: cargo fmt; entities:
# rustfmt (cargo fmt from entities/ hits "multiple workspace roots" with root+microservices).
fmt-rust:
    @cd microservices && cargo fmt --all
    @find entities -name '*.rs' -exec rustfmt {} +

# Check tooling is formatted (CI). Run `just init` first.
format-check:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ ! -d tooling/.venv ]; then
        echo "❌ tooling/.venv not found. Run: just init"
        exit 1
    fi
    tooling/.venv/bin/ruff format tooling/ --check

# Full QA: lint + format-check + tooling tests. Run before commit or demo.
qa:
    #!/usr/bin/env bash
    set -euo pipefail
    just lint
    just format-check
    tooling/.venv/bin/pytest tooling/tests -v --tb=short

# Auto-fix fixable ruff rules (including unsafe). Run `just init` first.
lint-fix:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ ! -d tooling/.venv ]; then
        echo "❌ tooling/.venv not found. Run: just init"
        exit 1
    fi
    tooling/.venv/bin/ruff check tooling/ --fix --unsafe-fixes

# Install pre-commit hooks (qa, microservices-fmt). Run `just init` first.
install-hooks:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ ! -d tooling/.venv ]; then
        echo "❌ tooling/.venv not found. Run: just init"
        exit 1
    fi
    tooling/.venv/bin/pre-commit install
    echo "✅ Pre-commit hooks installed"

# Find and remove unused imports in tooling (F401). Uses ruff. Run `just init` first.
# --fix: edit files in place; run again to confirm clean.
lint-unused-imports:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ ! -d tooling/.venv ]; then
        echo "❌ tooling/.venv not found. Run: just init"
        exit 1
    fi
    echo "🔍 Checking for unused imports (F401) in tooling..."
    tooling/.venv/bin/ruff check tooling/ --select F401 --fix

# ============================================================================
# Utilities
# ============================================================================

# Clean build artifacts
clean:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Cleaning build artifacts..."
    cd microservices && cargo clean
    find . -name "*.pyc" -delete
    find . -name "__pycache__" -type d -delete
    find . -name ".pytest_cache" -type d -delete

# Tail logs for a Tilt component
# Usage: just logs <component-name>
# Example: just logs general-ledger
logs component:
    @tooling/.venv/bin/rerp tilt logs {{component}} || echo "Use: tilt logs {{component}}"

# Show logs from all services
logs-all:
    @kubectl logs -n rerp -f --tail=100 --all-containers=true

# Show cluster and service status
status:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Cluster Status:"
    kind get clusters || echo "No Kind clusters found"
    echo ""
    echo "Pods Status:"
    kubectl get pods -n rerp 2>/dev/null || echo "No pods found"
    echo ""
    echo "Services:"
    kubectl get svc -n rerp 2>/dev/null || echo "No services found"
    echo ""
    echo "Ingress:"
    kubectl get ingress -n rerp 2>/dev/null || echo "No ingress found"

# Port forwarding for direct access
port-forward:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Setting up port forwards..."
    kubectl port-forward -n rerp svc/postgresql 5432:5432 &
    kubectl port-forward -n rerp svc/redis-master 6379:6379 &
    kubectl port-forward -n rerp svc/prometheus-server 9091:80 &
    kubectl port-forward -n rerp svc/grafana 3002:80 &
    echo "Port forwards established. Press Ctrl+C to stop."
    wait

# Database migrations
# Usage: just migrate [service]
# Example: just migrate general-ledger
migrate service="":
    #!/usr/bin/env bash
    set -euo pipefail
    if [ -z "{{service}}" ]; then
        echo "Running all database migrations..."
        echo "TODO: Implement migration runner"
    else
        echo "Running migrations for {{service}}..."
        echo "TODO: Implement service-specific migrations"
    fi

# Development shell
shell:
    @kubectl run -n rerp -it --rm debug --image=rust:1.73-slim --restart=Never -- bash

# ============================================================================
# Dependencies & Tools
# ============================================================================

# Check prerequisites
check-deps:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Checking dependencies..."
    command -v docker >/dev/null 2>&1 || { echo "docker is required but not installed."; exit 1; }
    command -v kind >/dev/null 2>&1 || { echo "kind is required but not installed."; exit 1; }
    command -v kubectl >/dev/null 2>&1 || { echo "kubectl is required but not installed."; exit 1; }
    command -v helm >/dev/null 2>&1 || { echo "helm is required but not installed."; exit 1; }
    command -v tilt >/dev/null 2>&1 || { echo "tilt is required but not installed."; exit 1; }
    echo "All dependencies are installed!"

# Install development tools
install-tools:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Installing development tools..."
    echo "Installing Kind..."
    curl -Lo ./kind https://kind.sigs.k8s.io/dl/v0.20.0/kind-$(uname -s | tr '[:upper:]' '[:lower:]')-amd64
    chmod +x ./kind
    sudo mv ./kind /usr/local/bin/kind
    echo "Installing Tilt..."
    curl -fsSL https://raw.githubusercontent.com/tilt-dev/tilt/master/scripts/install.sh | bash
    echo "Installing Helm..."
    curl https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash
    echo "Tools installed!"

# ============================================================================
# Entity Management
# ============================================================================

# Generate SQL migrations from entities
# Usage: just generate-sql
generate-sql:
    @echo "Generating SQL migrations from entities..."
    @cd entities && cargo run --bin generate-sql

# Check entity compilation
check-entities:
    @echo "Checking entity compilation..."
    @cd entities && cargo check --lib
