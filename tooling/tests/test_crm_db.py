"""CRM-DB-01: runs microservices/crm/scripts/test-crm-db.sh when a disposable Postgres is available.

The suite proves crm-db.sh up/apply/reset, cross-service schema and database isolation, the audit grants
and the fail-closed RLS guard. CI has no Postgres service for the Tooling job, so this skips there; run it
against a throwaway instance:

    docker run -d --name crm-db-test -e POSTGRES_HOST_AUTH_METHOD=trust -p 127.0.0.1:55432:5432 postgres:16
    CRM_PG_URL=postgres://postgres@127.0.0.1:55432/postgres \
        tooling/.venv/bin/pytest tooling/tests/test_crm_db.py -v
"""

from __future__ import annotations

import os
import subprocess
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[2]
SUITE = ROOT / "microservices" / "crm" / "scripts" / "test-crm-db.sh"


def test_crm_db_script_acceptance() -> None:
    url = os.environ.get("CRM_PG_URL", "")
    if not url:
        pytest.skip(
            "CRM_PG_URL not set; start a disposable Postgres to run the CRM-DB-01 acceptance suite"
        )
    result = subprocess.run(
        ["bash", str(SUITE)],
        env={**os.environ, "CRM_PG_URL": url},
        capture_output=True,
        text=True,
        timeout=300,
    )
    assert result.returncode == 0, result.stdout + result.stderr
    assert "checks passed" in result.stdout, result.stdout
