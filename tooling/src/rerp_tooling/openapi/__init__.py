"""OpenAPI: validate, fix operationId casing, check decimal formats, fix impl controllers. Re-exported from brrtrouter_tooling."""

from brrtrouter_tooling.openapi import (
    check_openapi_dir,
    find_openapi_files,
    fix_impl_controller,
    fix_impl_controllers_dir,
    fix_operation_id_run,
    is_snake_case,
    process_file,
    to_snake_case,
    validate_specs,
)

try:
    from brrtrouter_tooling.openapi import check_number_fields
except ImportError:
    check_number_fields = None  # type: ignore[misc, assignment]

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
