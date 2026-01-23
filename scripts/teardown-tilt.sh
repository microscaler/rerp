#!/usr/bin/env bash
# RERP Tilt Teardown Script
# Cleans up all Tilt and Docker resources

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

# Stop Tilt if running
if pgrep -f "tilt up" > /dev/null; then
    log_info "Stopping Tilt..."
    tilt down || true
    sleep 2
fi

# Stop and remove Docker containers
log_info "Stopping Docker containers..."
docker stop postgres-dev redis-dev prometheus-dev grafana-dev 2>/dev/null || true
docker rm postgres-dev redis-dev prometheus-dev grafana-dev 2>/dev/null || true

# Remove RERP microservice containers (if any)
for service in general-ledger invoice accounts-receivable accounts-payable bank-sync asset budget; do
    docker stop "rerp-${service}-dev" 2>/dev/null || true
    docker rm "rerp-${service}-dev" 2>/dev/null || true
done

# Remove Docker images (optional)
read -p "Do you want to remove Docker images? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    log_info "Removing Docker images..."
    # Remove RERP microservice images
    for service in general-ledger invoice accounts-receivable accounts-payable bank-sync asset budget; do
        docker rmi "rerp-${service}:latest" 2>/dev/null || true
        docker rmi "localhost:5001/rerp-${service}:tilt" 2>/dev/null || true
    done
fi

# Clean up Docker volumes (optional)
read -p "Do you want to remove Docker volumes (this will delete all data)? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    log_info "Removing Docker volumes..."
    docker volume rm postgres_data redis_data prometheus_data grafana_data 2>/dev/null || true
fi

# Clean up Docker networks
log_info "Cleaning up Docker networks..."
docker network prune -f 2>/dev/null || true

# Clean up any dangling resources
read -p "Do you want to run Docker system prune? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    log_info "Running Docker system prune..."
    docker system prune -f
fi

log_info "Teardown complete!"
