"""Code generation utilities for RERP services. Re-exports brrtrouter_tooling.gen + regenerate."""

from brrtrouter_tooling.gen import (
    call_brrtrouter_generate,
    call_brrtrouter_generate_stubs,
    find_brrtrouter,
)

from rerp_tooling.gen.regenerate import (
    regenerate_service,
    regenerate_suite_services,
)

__all__ = [
    "call_brrtrouter_generate",
    "call_brrtrouter_generate_stubs",
    "find_brrtrouter",
    "regenerate_service",
    "regenerate_suite_services",
]
