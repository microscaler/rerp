#!/usr/bin/env bash
# Tail logs for a Tilt component
# Usage: ./scripts/tail-tilt-logs.sh <component-name>
# Example: ./scripts/tail-tilt-logs.sh general-ledger

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
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

# Check if component name is provided
if [ $# -eq 0 ]; then
    log_error "Component name is required"
    echo ""
    echo "Usage: $0 <component-name>"
    echo ""
    echo "Examples:"
    echo "  $0 general-ledger"
    echo "  $0 invoice"
    echo "  $0 accounts-receivable"
    echo "  $0 accounts-payable"
    echo "  $0 bank-sync"
    echo "  $0 asset"
    echo "  $0 budget"
    echo ""
    echo "To list all available Tilt resources:"
    echo "  tilt get uiresources"
    exit 1
fi

COMPONENT_NAME="$1"

# Check if tilt is installed
if ! command -v tilt &> /dev/null; then
    log_error "Tilt is not installed. Please install it first."
    echo "  Install: curl -fsSL https://raw.githubusercontent.com/tilt-dev/tilt/master/scripts/install.sh | bash"
    exit 1
fi

# Check if Tilt is running
if ! tilt get uiresources &> /dev/null; then
    log_error "Tilt is not running or not connected to a Tilt instance"
    echo ""
    echo "Please start Tilt first:"
    echo "  tilt up"
    exit 1
fi

# Verify the component exists
if ! tilt get uiresources --format json 2>/dev/null | grep -q "\"name\":\"${COMPONENT_NAME}\""; then
    log_warn "Component '${COMPONENT_NAME}' not found in Tilt resources"
    echo ""
    echo "Available components:"
    tilt get uiresources --format json 2>/dev/null | grep -o '"name":"[^"]*"' | sed 's/"name":"//g' | sed 's/"//g' | sort | while read -r name; do
        echo "  - $name"
    done
    exit 1
fi

log_info "Tailing logs for component: ${BLUE}${COMPONENT_NAME}${NC}"
echo "Press Ctrl+C to stop"
echo ""

# Tail the logs using Tilt CLI
tilt logs "${COMPONENT_NAME}" --follow
