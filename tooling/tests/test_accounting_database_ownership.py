"""Contract tests for database bootstrap and migration ownership."""

from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def test_flux_database_job_contains_no_application_migration_path() -> None:
    entrypoint = (ROOT / "microservices/accounting/scripts/db-init-job.sh").read_text(
        encoding="utf-8"
    )
    dockerfile = (ROOT / "docker/jobs/Dockerfile").read_text(encoding="utf-8")

    assert "apply_migrations" not in entrypoint
    assert "apply_seeds" not in entrypoint
    assert "suite/migrations" not in dockerfile
    assert "suite/sql" not in dockerfile


def test_tilt_owns_explicit_manual_application_migration_cycle() -> None:
    tiltfile = (ROOT / "Tiltfile").read_text(encoding="utf-8")

    assert "'accounting-apply-migrations'" in tiltfile
    assert "RERP_APPLY_MIGRATIONS_ONLY=1" in tiltfile
    assert "trigger_mode=TRIGGER_MODE_MANUAL" in tiltfile
    assert "auto_init=False" in tiltfile
