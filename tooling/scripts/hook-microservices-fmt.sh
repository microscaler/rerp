#!/usr/bin/env bash
# pre-commit hook `microservices-fmt`. Without the tooling venv (the octopilot lint leg, which runs cargo fmt --check
# itself) it says so and passes.
[ -x tooling/.venv/bin/rerp ] || { echo "no tooling venv: cargo fmt is checked by the lint leg"; exit 0; }
exec tooling/.venv/bin/rerp pre-commit microservices-fmt
