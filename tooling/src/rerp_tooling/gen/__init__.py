"""Code generation utilities for RERP services. Re-exports brrtrouter_tooling.gen when available."""

try:
    from brrtrouter_tooling.gen import (
        call_brrtrouter_generate,
        call_brrtrouter_generate_stubs,
        find_brrtrouter,
    )

    from rerp_tooling.gen.regenerate import regenerate_service, regenerate_suite_services

    _BRRT = True
except ImportError:
    _BRRT = False
    call_brrtrouter_generate = None  # type: ignore[assignment]
    call_brrtrouter_generate_stubs = None  # type: ignore[assignment]
    find_brrtrouter = None  # type: ignore[assignment]
    regenerate_service = None  # type: ignore[assignment]
    regenerate_suite_services = None  # type: ignore[assignment]

__all__ = [
    "call_brrtrouter_generate",
    "call_brrtrouter_generate_stubs",
    "find_brrtrouter",
    "regenerate_service",
    "regenerate_suite_services",
]
