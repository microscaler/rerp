#!/usr/bin/env bash
# RERP Tilt-Only Setup Script
# Simplified setup that uses Tilt for everything

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_command() {
    if ! command -v "$1" &> /dev/null; then
        log_error "$1 is not installed. Please install it first."
        return 1
    fi
    return 0
}

# Check prerequisites
log_info "Checking prerequisites..."

MISSING_DEPS=""
check_command "docker" || MISSING_DEPS="$MISSING_DEPS docker"
check_command "tilt" || MISSING_DEPS="$MISSING_DEPS tilt"

if [ -n "$MISSING_DEPS" ]; then
    log_error "Missing dependencies:$MISSING_DEPS"
    log_info "Install missing dependencies:"
    
    if [[ "$OSTYPE" == "darwin"* ]]; then
        log_info "  brew install docker tilt"
    else
        log_info "  Docker: https://docs.docker.com/get-docker/"
        log_info "  Tilt: curl -fsSL https://raw.githubusercontent.com/tilt-dev/tilt/master/scripts/install.sh | bash"
    fi
    exit 1
fi

# Optional: Check for Kind if user wants Kubernetes mode
if check_command "kind" 2>/dev/null; then
    log_info "Kind is installed - Kubernetes mode available"
else
    log_warn "Kind not installed - only local Docker mode available"
    log_info "To install Kind: brew install kind (macOS) or see https://kind.sigs.k8s.io/"
fi

# Create necessary directories
log_info "Creating project directories..."
mkdir -p docker/{prometheus,grafana/{dashboards,datasources}}
mkdir -p openapi/accounting
mkdir -p microservices/accounting
mkdir -p k8s/{microservices,data}

# Create Docker volumes
log_info "Creating Docker volumes..."
docker volume create postgres_data 2>/dev/null || true
docker volume create redis_data 2>/dev/null || true
docker volume create prometheus_data 2>/dev/null || true
docker volume create grafana_data 2>/dev/null || true

# Check if running in CI or headless environment
if [ -n "${CI:-}" ] || [ -n "${GITHUB_ACTIONS:-}" ]; then
    TILT_FLAGS="--ci"
    log_info "Running in CI mode"
else
    TILT_FLAGS=""
fi

log_info "Setup complete! 🎉"
echo ""
echo "To start the development environment:"
echo ""
echo "  Using Just (recommended):"
echo "    just up        # Local Docker mode"
echo "    just up-k8s    # Kubernetes mode (requires Kind)"
echo ""
echo "  Or using Tilt directly:"
echo "    tilt up                # Local Docker mode"
echo "    tilt up -- --use-kind  # Kubernetes mode"
echo ""
echo "Services will be available at:"
echo "  - General Ledger API:    http://localhost:8001"
echo "  - General Ledger Docs:   http://localhost:8001/docs"
echo "  - Invoice API:           http://localhost:8002"
echo "  - Invoice Docs:          http://localhost:8002/docs"
echo "  - PostgreSQL:            localhost:5433 (5432 is used by local postgres)"
echo "  - Redis:                 localhost:6379"
echo "  - Prometheus:            http://localhost:9091"
echo "  - Grafana:               http://localhost:3002 (admin/admin)"
echo ""
echo "Useful commands:"
echo "  just help      # Show all available commands"
echo "  just down      # Stop all services"
echo "  just status    # Check service status"
echo "  just logs      # View service logs"
echo "  just teardown  # Clean up everything"
