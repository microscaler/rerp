"""OpenAPI: validate, fix operationId casing. Re-exports from brrtrouter_tooling when available."""

from rerp_tooling.openapi.validate import validate_specs

try:
    from brrtrouter_tooling.openapi import (
        check_number_fields,
        check_openapi_dir,
        find_openapi_files,
        fix_impl_controller,
        fix_impl_controllers_dir,
        fix_operation_id_run,
        is_snake_case,
        process_file,
        to_snake_case,
    )

    _BRRT = True
except ImportError:
    _BRRT = False
    check_number_fields = None  # type: ignore[assignment]
    check_openapi_dir = None  # type: ignore[assignment]
    find_openapi_files = None  # type: ignore[assignment]
    fix_impl_controller = None  # type: ignore[assignment]
    fix_impl_controllers_dir = None  # type: ignore[assignment]
    fix_operation_id_run = None  # type: ignore[assignment]
    is_snake_case = None  # type: ignore[assignment]
    process_file = None  # type: ignore[assignment]
    to_snake_case = None  # type: ignore[assignment]

# RERP's fix_operation_id is used by cli/openapi (works with or without brrtrouter)
from rerp_tooling.openapi.fix_operation_id import run as fix_operation_id_run

__all__ = [
    "check_number_fields",
    "check_openapi_dir",
    "find_openapi_files",
    "fix_impl_controller",
    "fix_impl_controllers_dir",
    "fix_operation_id_run",
    "is_snake_case",
    "process_file",
    "to_snake_case",
    "validate_specs",
]
