"""BFF OpenAPI generation: re-export from BRRTRouter tooling (Story 1.4)."""

from brrtrouter_tooling.bff import (
    discover_sub_services,
    generate_system_bff_spec,
    list_systems_with_sub_services,
)

__all__ = [
    "discover_sub_services",
    "generate_system_bff_spec",
    "list_systems_with_sub_services",
]
