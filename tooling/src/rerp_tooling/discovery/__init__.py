"""Discovery: re-export from brrtrouter_tooling.discovery (suites, sources)."""

from brrtrouter_tooling.discovery import (
    bff_service_to_suite,
    bff_suite_config_path,
    discover_bff_suite_config,
    discover_helm,
    discover_kind_host_ports,
    discover_openapi_bff_localhost,
    discover_openapi_suite_microservice_localhost,
    discover_tiltfile,
    get_bff_service_name_from_config,
    iter_bffs,
    load_suite_services,
    openapi_bff_path,
    service_to_suite,
    suite_sub_service_names,
    suites_with_bff,
    tilt_service_names,
)

__all__ = [
    "bff_service_to_suite",
    "bff_suite_config_path",
    "discover_bff_suite_config",
    "discover_helm",
    "discover_kind_host_ports",
    "discover_openapi_bff_localhost",
    "discover_openapi_suite_microservice_localhost",
    "discover_tiltfile",
    "get_bff_service_name_from_config",
    "iter_bffs",
    "load_suite_services",
    "openapi_bff_path",
    "service_to_suite",
    "suite_sub_service_names",
    "suites_with_bff",
    "tilt_service_names",
]
