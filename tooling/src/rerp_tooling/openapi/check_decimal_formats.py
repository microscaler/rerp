"""Check OpenAPI specs for number fields without format: decimal or format: money."""

import sys
from pathlib import Path
from typing import Any

import yaml


def check_number_fields(spec_path: Path) -> list[str]:  # noqa: C901
    """Check for number fields without format: decimal or format: money.

    Returns list of issue descriptions.
    """
    try:
        with spec_path.open() as f:
            spec = yaml.safe_load(f)
    except (OSError, yaml.YAMLError) as e:
        return [f"Failed to parse: {e}"]

    if not spec:
        return []

    issues = []

    def check_schema(schema: dict[str, Any], path: str = "") -> None:
        if not isinstance(schema, dict):
            return

        if schema.get("type") == "number":
            format_val = schema.get("format")
            if format_val not in ("decimal", "money"):
                issues.append(f"{path}: type: number without format: decimal or format: money")

        # Check properties
        if "properties" in schema:
            for prop_name, prop_schema in schema["properties"].items():
                new_path = f"{path}.{prop_name}" if path else prop_name
                check_schema(prop_schema, new_path)

        # Check items (for arrays)
        if "items" in schema:
            check_schema(schema["items"], f"{path}[]")

        # Check allOf, anyOf, oneOf
        for key in ("allOf", "anyOf", "oneOf"):
            if key in schema:
                for sub_schema in schema[key]:
                    check_schema(sub_schema, path)

    # Check all schemas
    if "components" in spec and "schemas" in spec["components"]:
        for schema_name, schema in spec["components"]["schemas"].items():
            check_schema(schema, f"components.schemas.{schema_name}")

    # Check request/response schemas in paths
    if "paths" in spec:
        for path, methods in spec["paths"].items():
            if not isinstance(methods, dict):
                continue
            for method, operation in methods.items():
                if not isinstance(operation, dict):
                    continue
                # Request body
                if "requestBody" in operation:
                    request_body = operation["requestBody"]
                    if isinstance(request_body, dict) and "content" in request_body:
                        content = request_body["content"]
                        for media_obj in content.values():
                            if isinstance(media_obj, dict) and "schema" in media_obj:
                                check_schema(
                                    media_obj["schema"],
                                    f"paths.{path}.{method}.requestBody",
                                )

                # Responses
                if "responses" in operation:
                    responses = operation["responses"]
                    if isinstance(responses, dict):
                        for status, response in responses.items():
                            if isinstance(response, dict) and "content" in response:
                                content = response["content"]
                                for media_obj in content.values():
                                    if isinstance(media_obj, dict) and "schema" in media_obj:
                                        check_schema(
                                            media_obj["schema"],
                                            f"paths.{path}.{method}.responses.{status}",
                                        )

    return issues


def main() -> int:
    """Check all OpenAPI specs in openapi/ directory."""
    # Script is in tooling/src/rerp_tooling/openapi/, so go up to project root
    script_path = Path(__file__)
    project_root = script_path.parent.parent.parent.parent.parent
    openapi_dir = project_root / "openapi"

    if not openapi_dir.exists():
        print(f"❌ OpenAPI directory not found: {openapi_dir}")
        return 1

    total_issues = 0
    files_with_issues = []

    # Find all openapi.yaml files
    for spec_path in sorted(openapi_dir.rglob("openapi.yaml")):
        issues = check_number_fields(spec_path)
        if issues:
            files_with_issues.append((spec_path, issues))
            total_issues += len(issues)

    if files_with_issues:
        print(
            f"❌ Found {total_issues} number fields without format in {len(files_with_issues)} file(s):\n"
        )
        for spec_path, issues in files_with_issues:
            rel_path = spec_path.relative_to(project_root)
            print(f"  {rel_path}: {len(issues)} issue(s)")
            for issue in issues[:5]:  # Show first 5 issues per file
                print(f"    - {issue}")
            if len(issues) > 5:
                print(f"    ... and {len(issues) - 5} more")
            print()
        return 1
    print("✅ All OpenAPI specs have format: decimal or format: money for number fields")
    return 0


if __name__ == "__main__":
    sys.exit(main())
