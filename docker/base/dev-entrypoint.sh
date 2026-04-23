#!/bin/sh
# dev-entrypoint.sh
# 
# A lightweight wrapper script executing the target application.
# For local Tilt development, the binary might not be present immediately when the
# container starts (as it is copied asynchronously via live_update sync).
# This script stalls loop execution safely instead of allowing Kubernetes 
# to trigger CrashLoopBackOff instantly.

if [ "$#" -lt 1 ]; then
    echo "❌ Usage: $0 <binary_path> [args...]"
    exit 1
fi

BINARY="$1"

# Trap SIGHUP from Tilt (kill -HUP 1) to restart the child process gracefully without restarting the container
PID=""

trap 'echo "🔄 Received SIGHUP from Tilt. Hot-reloading..."; [ -n "$PID" ] && kill -TERM "$PID"' HUP

while true; do
    if [ ! -x "$BINARY" ]; then
        echo "⏳ Waiting for binary $BINARY to be synced by Tilt..."
        while [ ! -x "$BINARY" ]; do
            sleep 0.5
        done
        echo "✅ Binary $BINARY found!"
    fi

    echo "🚀 Booting $BINARY..."
    "$@" &
    PID=$!
    
    # Wait for child process; wait is interrupted when trap executes
    wait $PID
    EXIT_CODE=$?
    
    echo "⚠️ Process exited (code $EXIT_CODE). Restarting in 1s..."
    sleep 1
done
