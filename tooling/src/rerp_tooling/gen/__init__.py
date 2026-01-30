"""Code generation utilities for RERP services. Re-exports brrtrouter_tooling.gen + regenerate."""

from brrtrouter_tooling.gen import (
    call_brrtrouter_generate,
    call_brrtrouter_generate_stubs,
    find_brrtrouter,
)

try:
    from rerp_tooling.gen.regenerate import (
        regenerate_service,
        regenerate_suite_services,
    )
except (ImportError, ModuleNotFoundError):
    regenerate_service = None  # type: ignore[misc, assignment]
    regenerate_suite_services = None  # type: ignore[misc, assignment]

__all__ = [
    "call_brrtrouter_generate",
    "call_brrtrouter_generate_stubs",
    "find_brrtrouter",
    "regenerate_service",
    "regenerate_suite_services",
]
