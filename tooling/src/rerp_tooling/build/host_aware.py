"""Host-aware build: re-export from brrtrouter_tooling.build (cargo/cross/zigbuild, multi-arch)."""

from __future__ import annotations

from brrtrouter_tooling.build.host_aware import (
    ARCH_TARGETS,
    _determine_architectures,
    detect_host_architecture,
    run,
    should_use_cross,
    should_use_zigbuild,
)

__all__ = [
    "ARCH_TARGETS",
    "_determine_architectures",
    "detect_host_architecture",
    "run",
    "should_use_cross",
    "should_use_zigbuild",
]
