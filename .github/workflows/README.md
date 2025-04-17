# GitHub Actions Workflows

## Available Workflows

### `generate-bff-specs.yml`

**Purpose**: Automatically generate system-level BFF OpenAPI specifications from sub-service specs.

**Triggers**:
- Push to `main` or `develop` branches when OpenAPI specs change
- Pull requests that modify OpenAPI specs
- Manual workflow dispatch

**What It Does**:
1. Detects changes to sub-service OpenAPI specs (`openapi/{system}/{service}/openapi.yaml`)
2. Runs `scripts/generate_system_bff.py` to regenerate all system BFF specs
3. Commits updated BFF specs (on push) or creates PR (on pull request)

**Output**:
- Updated system-level BFF specs in `openapi/{system}/openapi.yaml`
- Automatic commit or PR with changes

**Why This Exists**:
- Ensures BFF specs are always in sync with sub-service specs
- Prevents manual editing of auto-generated files
- Provides audit trail of when specs were regenerated

---

## Future Workflows

Additional workflows will be added for:
- Code generation from OpenAPI specs
- Testing generated services
- Deployment automation
- Documentation generation
