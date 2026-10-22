#!/usr/bin/env bash
# pre-commit hook `qa`: tooling QA through just. On a runner without just (the octopilot lint leg) it says so and passes:
# checks.yml's Tooling job runs the same QA there.
command -v just >/dev/null 2>&1 || { echo "just not installed: tooling QA left to checks.yml"; exit 0; }
exec just qa
