"""BFF system generation: implemented in BRRTRouter tooling (Story 1.4).

This module is deprecated; use brrtrouter_tooling.bff or rerp_tooling.bff (re-export) instead.
"""

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
