"""Path-parameter declaration gate for all OpenAPI specs.

Every parameter with ``in: path`` must have its ``{name}`` placeholder in
the path template it is attached to. OpenAPI tooling that only checks
schema validity lets a mis-parented operation through: the route still
generates, but the phantom parameter is never bound and the operation
fails at runtime. This gate caught ``update_ledger_account`` being mounted
as ``PATCH /v1/accounts`` with an undeclared ``id`` parameter.
"""

import re
from pathlib import Path

import pytest
import yaml

ROOT = Path(__file__).resolve().parents[2]
OPENAPI = ROOT / "openapi"

METHODS = {"get", "put", "post", "patch", "delete", "options", "head"}

SPECS = sorted(OPENAPI.rglob("openapi.yaml"))


def _specs():
    return [(str(p.relative_to(ROOT)), p) for p in SPECS]


@pytest.mark.parametrize("rel_path,path", _specs(), ids=[r for r, _ in _specs()])
def test_path_parameters_are_declared_in_their_template(rel_path, path):
    spec = yaml.safe_load(path.read_text())
    if not isinstance(spec, dict) or "paths" not in spec:
        pytest.skip("not an OpenAPI document")
    for template, item in (spec.get("paths") or {}).items():
        if not isinstance(item, dict):
            continue
        declared = set(re.findall(r"\{([^}]+)\}", template))
        for method, operation in item.items():
            if method not in METHODS or not isinstance(operation, dict):
                continue
            for param in operation.get("parameters") or []:
                if isinstance(param, dict) and param.get("in") == "path":
                    assert param.get("name") in declared, (
                        f"{rel_path}: {method.upper()} {template} declares "
                        f"path parameter '{param.get('name')}' that is not in "
                        f"the path template"
                    )
