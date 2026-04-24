# rerp CLI Tooling

> The `rerp` CLI in `tooling/` — the central automation tool for RERP development.

**Status:** partially-verified

## Setup

```bash
just init          # Creates tooling/.venv
tooling/.venv/bin/rerp  # Use this for all rerp commands
```

## Main Subcommands

- `rerp bff generate-system` — Generate system-level BFF specs
- `rerp bff generate-suite` — Generate suite-level BFF (e.g. accounting)
- `rerp ports` — Show/manage service ports from port-registry.json

## BFF Generation

Suite BFF:
```bash
bff-generator generate-spec --config openapi/{suite}/bff-suite-config.yaml --output openapi/{suite}/openapi_bff.yaml
```

System BFF:
```bash
rerp bff generate-system
```

## Important

- All scripting and automation **must be in `tooling/`**
- Do not create scripts in other directories
- CI workflows use `rerp` commands via the venv
- See `tooling/README.md` for full surface

## Code Anchors
- CLI source: `tooling/src/`
- Tests: `tooling/tests/`
- Config: `tooling/.venv/` (virtualenv)
- README: `tooling/README.md`
