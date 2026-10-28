"""Contract-honesty gate for scaffold-only Accounting services (NFR-DOG-013).

Scaffold services must never serve generated example payloads: their impl
binaries register only impl controllers (honest 501s until delivered), and
hand-written controllers are marked user-owned to opt out of the 501 rule.
Delivered services (invoice, general-ledger) and the BFF proxy are exempt by
name, not by pattern.
"""

from pathlib import Path

import pytest

ACCOUNTING = Path(__file__).resolve().parents[2] / "microservices" / "accounting"

# Services with delivered product behavior (registered in the readiness plan).
DELIVERED = {"invoice", "general-ledger"}
# Honest proxy suite entry; routes to upstreams, registers no mock handlers.
PROXY = {"bff"}
# Everything else with an impl binary must satisfy the honesty rules below.


def scaffold_services() -> list[Path]:
    return sorted(
        impl.parent
        for impl in ACCOUNTING.glob("*/impl")
        if (impl / "src" / "main.rs").exists() and impl.parent.name not in DELIVERED | PROXY
    )


@pytest.mark.parametrize("svc", scaffold_services(), ids=lambda p: p.name)
def test_scaffold_main_registers_no_generated_mocks(svc: Path) -> None:
    main = (svc / "impl" / "src" / "main.rs").read_text(encoding="utf-8")
    assert "register_from_spec" not in main, (
        f"{svc.name}: main.rs dispatches generated mock handlers; scaffold "
        "runtimes must call impl_registry::register_impl (ADR 0001 Layer 2)"
    )
    assert "register_impl(" in main, f"{svc.name}: main.rs does not register impl controllers"


@pytest.mark.parametrize("svc", scaffold_services(), ids=lambda p: p.name)
def test_scaffold_controllers_are_honest(svc: Path) -> None:
    controllers = sorted((svc / "impl" / "src" / "controllers").glob("*.rs"))
    assert controllers, f"{svc.name}: no controllers found"
    for ctrl in controllers:
        if ctrl.name == "mod.rs":
            continue
        text = ctrl.read_text(encoding="utf-8")
        head = "\n".join(text.splitlines()[:5])
        marked = "BRRTRouter: user-owned" in head
        if marked:
            # Deliberately implemented by a work package; opt-in and visible.
            continue
        assert "not_implemented(" in text, (
            f"{ctrl}: scaffold controller neither returns an honest 501 nor is "
            "marked '// BRRTRouter: user-owned' as implemented"
        )
        assert "HttpJson::ok(Response {" not in text, (
            f"{ctrl}: returns a fabricated generated example payload"
        )
