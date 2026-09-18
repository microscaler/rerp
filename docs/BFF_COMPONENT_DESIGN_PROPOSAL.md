# BFF Component Design Proposal

## Executive Summary

This document proposes the design for externalizing the BFF (Backend for Frontend) generation component as a separate, reusable tool that supports **multi-suite architectures** used by both PriceWhisperer (trader + platform suites) and RERP (accounting + sales + hr + ... suites).

## Current State Analysis

### PriceWhisperer Architecture (Multiple Suites)

**Structure**:
```
pricewhisperer/
├── microservices/
│   ├── openapi/
│   │   ├── trader/              # Suite 1: Trader Portal
│   │   │   ├── idam/
│   │   │   ├── marketing/
│   │   │   ├── portfolio/
│   │   │   └── ... (18 services)
│   │   ├── platform/            # Suite 2: Platform Portal (prepared, not yet implemented)
│   │   │   └── ... (future services)
│   │   ├── openapi_trader_bff.yaml    # BFF for trader suite
│   │   └── openapi_platform_bff.yaml  # BFF for platform suite (future)
│   └── bff/
│       ├── traderBFF/          # BFF service for trader suite
│       └── platformBFF/         # BFF service for platform suite (prepared)
```

**Characteristics**:
- **Two suites**: `trader` (implemented) and `platform` (prepared, not yet implemented)
- **Per-suite BFFs**: Each suite has its own BFF spec and service
- **Independent deployment**: Suites can be deployed separately
- **Discovery**: `microservices/openapi/{suite}/*/openapi.yaml` per suite
- **Current state**: Trader suite has 18 services, platform suite is empty (prepared for future)

### RERP Architecture (Multiple Suites)

**Structure**:
```
rerp/
├── openapi/
│   ├── accounting/              # Suite 1: Accounting
│   │   ├── general-ledger/
│   │   ├── invoice/
│   │   ├── accounts-receivable/
│   │   ├── ... (9 services)
│   │   └── openapi_bff.yaml     # BFF for accounting suite
│   ├── sales/                    # Suite 2: Sales & CRM (future)
│   │   ├── crm/
│   │   ├── quotations/
│   │   ├── orders/
│   │   └── openapi_bff.yaml     # BFF for sales suite
│   ├── hr/                       # Suite 3: Human Resources (future)
│   │   ├── hr-core/
│   │   ├── payroll/
│   │   └── openapi_bff.yaml     # BFF for hr suite
│   └── ... (more suites)
```

**Characteristics**:
- **Multiple independent suites**: accounting (implemented), sales, hr, manufacturing, etc. (planned)
- **Per-suite BFFs**: Each suite has its own BFF spec and service
- **Independent deployment**: Suites can be deployed separately (separate K8s namespaces)
- **Discovery**: `openapi/{suite}/*/openapi.yaml` per suite
- **Current state**: Accounting suite has 9 services, other suites planned

## Requirements

### Functional Requirements

1. **OpenAPI Spec Discovery**
   - Detect BRRTRouter OpenAPI YAML files
   - Support directory-based discovery
   - Handle both single-suite and multi-suite structures

2. **BFF Spec Generation**
   - Aggregate paths, schemas, tags from microservice specs
   - Generate idempotent, deterministic output
   - Support schema prefixing to avoid conflicts
   - Handle nested $ref references

3. **BFF Service Generation**
   - Generate BFF service code from BFF OpenAPI spec
   - Use BRRTRouter to generate handlers/routes
   - Support suite-specific configuration

4. **Configuration Management**
   - Suite-based configuration (for RERP)
   - Service-to-port mapping
   - Base path configuration
   - Output path configuration

### Non-Functional Requirements

1. **Idempotency**: Same inputs always produce same output
2. **Deterministic**: Sorted output for consistent diffs
3. **Extensible**: Easy to add new suites or services
4. **Reusable**: Works for both PriceWhisperer and RERP architectures

## Design Options

### Option 1: Python Package (PyPI) ⭐ **RECOMMENDED**

**Approach**: Package as a Python library/CLI tool, installable via pip

**Structure**:
```
bff-generator/
├── pyproject.toml              # Python package configuration
├── setup.py                    # Package setup (if needed)
├── README.md
├── LICENSE
├── src/
│   └── bff_generator/
│       ├── __init__.py
│       ├── cli.py              # CLI entry point
│       ├── discovery.py        # OpenAPI spec discovery
│       ├── generator.py        # BFF spec generation
│       ├── config.py           # Configuration management
│       └── utils.py            # Utility functions
├── tests/
│   └── test_*.py
└── examples/
    ├── pricewhisperer/
    │   └── bff-config.yaml     # Example config
    └── rerp/
        └── bff-config.yaml     # Example config
```

**Installation**:
```bash
pip install bff-generator
# or
pip install git+https://github.com/microscaler/bff-generator.git
```

**Usage**:
```bash
# PriceWhisperer - Trader suite
bff-generator generate \
  --suite trader \
  --openapi-dir microservices/openapi/trader \
  --output microservices/openapi/openapi_trader_bff.yaml \
  --config microservices/openapi/trader/bff-config.yaml

# PriceWhisperer - Platform suite (when implemented)
bff-generator generate \
  --suite platform \
  --openapi-dir microservices/openapi/platform \
  --output microservices/openapi/openapi_platform_bff.yaml \
  --config microservices/openapi/platform/bff-config.yaml

# RERP - Accounting suite
bff-generator generate \
  --suite accounting \
  --openapi-dir openapi/accounting \
  --output openapi/accounting/openapi_bff.yaml \
  --config openapi/accounting/bff-config.yaml
```

**Configuration File Examples**:

**PriceWhisperer - Trader Suite** (`microservices/openapi/trader/bff-config.yaml`):
```yaml
suite: trader
architecture: multi-suite
services:
  idam:
    base_path: /api/identity
    port: 8002
  marketing:
    base_path: /api/marketing
    port: 8004
  portfolio:
    base_path: /api/portfolio
    port: 8016
  # ... 19 services total
```

**RERP - Accounting Suite** (`openapi/accounting/bff-config.yaml`):
```yaml
suite: accounting
architecture: multi-suite
services:
  general-ledger:
    base_path: /api/general-ledger
    port: 8001
  invoice:
    base_path: /api/invoice
    port: 8002
  # ... 9 services total
```

**Pros**:
- ✅ Standard Python packaging (PyPI)
- ✅ Easy to install and use
- ✅ Version management via pip
- ✅ Can be used as library or CLI
- ✅ Well-understood by developers
- ✅ Easy to extend with plugins

**Cons**:
- ⚠️ Requires Python runtime
- ⚠️ Python dependency management

---

### Option 2: Rust CLI Tool (Cargo)

**Approach**: Package as a Rust binary, installable via cargo

**Structure**:
```
bff-generator/
├── Cargo.toml
├── README.md
├── LICENSE
├── src/
│   ├── main.rs                 # CLI entry point
│   ├── discovery.rs            # OpenAPI spec discovery
│   ├── generator.rs            # BFF spec generation
│   ├── config.rs               # Configuration management
│   └── utils.rs                # Utility functions
├── tests/
│   └── test_*.rs
└── examples/
    └── bff-config.toml
```

**Installation**:
```bash
cargo install --git https://github.com/microscaler/bff-generator.git
```

**Usage**:
```bash
bff-generator generate \
  --suite accounting \
  --openapi-dir openapi/accounting \
  --output openapi/accounting/openapi_bff.yaml \
  --config bff-config.toml
```

**Pros**:
- ✅ Single binary (no runtime dependencies)
- ✅ Fast execution
- ✅ Consistent with Rust ecosystem
- ✅ Can be distributed as binary

**Cons**:
- ⚠️ Requires Rust toolchain to build
- ⚠️ Less familiar to non-Rust developers
- ⚠️ YAML parsing in Rust (more dependencies)

---

### Option 3: Standalone Script Repository

**Approach**: Keep as Python scripts, but in separate repository

**Structure**:
```
bff-generator/
├── README.md
├── LICENSE
├── scripts/
│   ├── generate_bff_spec.py    # Main generation script
│   ├── discover_services.py     # Service discovery
│   └── utils.py                # Utilities
├── config/
│   ├── pricewhisperer.yaml     # PriceWhisperer config
│   └── rerp.yaml               # RERP config template
└── examples/
    └── ...
```

**Usage**:
```bash
# Clone or reference as git submodule
git clone https://github.com/microscaler/bff-generator.git
cd bff-generator
python3 scripts/generate_bff_spec.py --suite accounting --config ../rerp/bff-config.yaml
```

**Pros**:
- ✅ Simple, no packaging overhead
- ✅ Easy to modify and extend
- ✅ Can be referenced as git dependency

**Cons**:
- ❌ No version management
- ❌ Manual installation/updates
- ❌ Less discoverable

---

## Recommended Approach: Python Package (Option 1) ⭐

### Rationale

1. **Python is already used**: Both PriceWhisperer and RERP use Python for BFF generation
2. **YAML handling**: Python has excellent YAML libraries (PyYAML)
3. **Easy to extend**: Python is accessible and extensible
4. **Version management**: PyPI provides versioning and distribution
5. **CLI + Library**: Can be used as both CLI tool and Python library
6. **Cross-platform**: Works on all platforms

### Implementation Details

#### Package Structure

```
bff-generator/
├── pyproject.toml
├── README.md
├── LICENSE
├── .gitignore
├── src/
│   └── bff_generator/
│       ├── __init__.py
│       ├── cli.py              # CLI entry point (click/argparse)
│       ├── discovery.py         # OpenAPI spec discovery
│       ├── generator.py        # BFF spec generation logic
│       ├── config.py            # Configuration loading/validation
│       ├── schemas.py           # Schema merging and prefixing
│       └── utils.py             # Utility functions
├── tests/
│   ├── test_discovery.py
│   ├── test_generator.py
│   ├── test_config.py
│   └── fixtures/
│       └── openapi_specs/
├── examples/
│   ├── pricewhisperer/
│   │   └── bff-config.yaml
│   └── rerp/
│       └── bff-config.yaml
└── docs/
    └── USAGE.md
```

#### Configuration Schema

**PriceWhisperer - Trader Suite**:
```yaml
# microservices/openapi/trader/bff-config.yaml
suite: trader
architecture: multi-suite
openapi_base_dir: microservices/openapi
output_path: microservices/openapi/openapi_trader_bff.yaml
bff_service_name: traderBFF
bff_output_dir: microservices/bff/traderBFF

services:
  idam:
    base_path: /api/identity
    port: 8002
    spec_path: trader/idam/openapi.yaml
  marketing:
    base_path: /api/marketing
    port: 8004
    spec_path: trader/marketing/openapi.yaml
  portfolio:
    base_path: /api/portfolio
    port: 8016
    spec_path: trader/portfolio/openapi.yaml
  # ... 18 services total

metadata:
  title: "PriceWhisperer Trader Backend for Frontend API"
  description: "BFF API for the Trader Portal"
  version: "1.0.0"
  contact:
    name: "PriceWhisperer API Support"
    email: "api-support@pricewhisperer.ai"
  servers:
    - url: "https://api.pricewhisperer.ai"
      description: "Production server"
    - url: "http://localhost:8000"
      description: "Local development server"
```

**PriceWhisperer - Platform Suite** (prepared, not yet implemented):
```yaml
# microservices/openapi/platform/bff-config.yaml
suite: platform
architecture: multi-suite
openapi_base_dir: microservices/openapi
output_path: microservices/openapi/openapi_platform_bff.yaml
bff_service_name: platformBFF
bff_output_dir: microservices/bff/platformBFF

services:
  # Future platform services will be added here

metadata:
  title: "PriceWhisperer Platform Backend for Frontend API"
  description: "BFF API for the Platform Portal"
  version: "1.0.0"
  contact:
    name: "PriceWhisperer API Support"
    email: "api-support@pricewhisperer.ai"
  servers:
    - url: "https://platform.pricewhisperer.ai"
      description: "Production server"
    - url: "http://localhost:8001"
      description: "Local development server"
```

**RERP - Accounting Suite**:
```yaml
# openapi/accounting/bff-config.yaml
suite: accounting
architecture: multi-suite
openapi_base_dir: openapi/accounting
output_path: openapi/accounting/openapi_bff.yaml
bff_service_name: accountingBFF
bff_output_dir: microservices/bff/accountingBFF

services:
  general-ledger:
    base_path: /api/general-ledger
    port: 8001
    spec_path: general-ledger/openapi.yaml
  invoice:
    base_path: /api/invoice
    port: 8002
    spec_path: invoice/openapi.yaml
  # ... more services

metadata:
  title: "RERP Accounting Backend for Frontend API"
  description: "BFF API for the Accounting Suite"
  version: "1.0.0"
  contact:
    name: "RERP API Support"
    email: "api-support@rerp.ai"
  servers:
    - url: "https://api.rerp.ai/accounting"
      description: "Production server"
    - url: "http://localhost:8000"
      description: "Local development server"
```

#### CLI Interface

```bash
# Generate BFF spec only
bff-generator generate-spec \
  --config bff-config.yaml \
  --suite accounting

# Generate BFF spec and service code
bff-generator generate \
  --config bff-config.yaml \
  --suite accounting \
  --brrtrouter-path ../BRRTRouter/target/debug/brrtrouter-gen

# Discover services automatically (no config needed)
bff-generator discover \
  --openapi-dir openapi/accounting \
  --suite accounting \
  --output openapi/accounting/openapi_bff.yaml

# Validate configuration
bff-generator validate \
  --config bff-config.yaml

# List discovered services
bff-generator list-services \
  --openapi-dir openapi/accounting \
  --suite accounting
```

#### Python API (Library Usage)

```python
from bff_generator import BFFGenerator, SuiteConfig

# Load configuration
config = SuiteConfig.from_file('bff-config.yaml')

# Generate BFF spec
generator = BFFGenerator(config)
bff_spec = generator.generate_spec()

# Write to file
generator.write_spec('openapi/accounting/openapi_bff.yaml')

# Generate BFF service code (if BRRTRouter path provided)
if brrtrouter_path:
    generator.generate_service(brrtrouter_path, 'microservices/bff/accountingBFF')
```

#### Integration with Existing Systems

**PriceWhisperer Tiltfile**:
```python
# Generate BFF for trader suite
local_resource(
    'bff-trader-spec-gen',
    cmd='''
        bff-generator generate-spec \
          --config ./microservices/openapi/trader/bff-config.yaml \
          --suite trader
    ''',
    deps=[
        './microservices/openapi/trader/bff-config.yaml',
        './microservices/openapi/trader/**/*.yaml',
    ],
)

# Generate BFF for platform suite (when implemented)
local_resource(
    'bff-platform-spec-gen',
    cmd='''
        bff-generator generate-spec \
          --config ./microservices/openapi/platform/bff-config.yaml \
          --suite platform
    ''',
    deps=[
        './microservices/openapi/platform/bff-config.yaml',
        './microservices/openapi/platform/**/*.yaml',
    ],
)
```

**RERP Tiltfile**:
```python
# Generate BFF for each suite
for suite in ['accounting', 'sales', 'hr']:
    local_resource(
        f'bff-{suite}-spec-gen',
        cmd=f'''
            bff-generator generate-spec \
              --config ./openapi/{suite}/bff-config.yaml \
              --suite {suite}
        ''',
        deps=[
            f'./openapi/{suite}/bff-config.yaml',
            f'./openapi/{suite}/**/*.yaml',
        ],
    )
```

## Architecture Support

### Multi-Suite Architecture (Both PriceWhisperer and RERP)

**Both PriceWhisperer and RERP use multi-suite architecture**:

- **PriceWhisperer**: `trader` suite (implemented) + `platform` suite (prepared)
- **RERP**: `accounting` suite (implemented) + `sales`, `hr`, etc. (planned)

**Configuration**:
- `architecture: multi-suite`
- Per-suite `bff-config.yaml` files
- Per-suite BFF spec output
- Per-suite BFF service

**Discovery**:
- **PriceWhisperer**: Scans `microservices/openapi/{suite}/*/openapi.yaml` per suite
- **RERP**: Scans `openapi/{suite}/*/openapi.yaml` per suite
- Maps to services in suite-specific config
- Generates separate BFF spec per suite

### Configuration Examples

**Configuration**:
- `architecture: multi-suite`
- Per-suite `bff-config.yaml` in each suite directory
- Per-suite BFF spec output
- Per-suite BFF service

**Discovery**:
- Scans `openapi/{suite}/*/openapi.yaml` per suite
- Each suite has its own config
- Generates separate BFF spec per suite

**Example Structure**:

**PriceWhisperer**:
```
pricewhisperer/
├── microservices/
│   ├── openapi/
│   │   ├── trader/
│   │   │   ├── bff-config.yaml          # Trader suite config
│   │   │   ├── idam/
│   │   │   ├── marketing/
│   │   │   └── ... (18 services)
│   │   ├── platform/
│   │   │   ├── bff-config.yaml          # Platform suite config (prepared)
│   │   │   └── ... (future services)
│   │   ├── openapi_trader_bff.yaml      # Generated BFF spec for trader
│   │   └── openapi_platform_bff.yaml    # Generated BFF spec for platform (future)
│   └── bff/
│       ├── traderBFF/                   # BFF service for trader
│       └── platformBFF/                 # BFF service for platform (prepared)
```

**RERP**:
```
rerp/
├── openapi/
│   ├── accounting/
│   │   ├── bff-config.yaml          # Accounting suite config
│   │   ├── openapi_bff.yaml         # Generated BFF spec
│   │   ├── general-ledger/
│   │   └── invoice/
│   ├── sales/
│   │   ├── bff-config.yaml          # Sales suite config
│   │   ├── openapi_bff.yaml         # Generated BFF spec
│   │   ├── crm/
│   │   └── quotations/
│   └── hr/
│       ├── bff-config.yaml          # HR suite config
│       ├── openapi_bff.yaml         # Generated BFF spec
│       └── ...
```

## Advanced Features

### 1. Automatic Service Discovery

If `bff-config.yaml` doesn't specify all services, auto-discover from directory:

```yaml
# Minimal config with auto-discovery
suite: accounting
architecture: multi-suite
auto_discover: true  # Discover services from directory
openapi_base_dir: openapi/accounting
output_path: openapi/accounting/openapi_bff.yaml

# Only specify overrides
services:
  general-ledger:
    base_path: /api/general-ledger  # Override default
    port: 8001
```

### 2. Port Registry Integration

Integrate with RERP's port registry system:

```python
# Read from port-registry.json
port_registry = load_port_registry('scripts/port-registry.json')
config.services['general-ledger'].port = port_registry['general-ledger']
```

### 3. Schema Conflict Resolution

Enhanced schema prefixing and conflict detection:

```yaml
# Config option
schema_prefixing:
  strategy: service-name  # or 'suite-service-name' for multi-suite
  conflict_resolution: prefix  # or 'error', 'warn'
```

### 4. BFF Service Generation

After generating BFF spec, optionally generate BFF service code:

```bash
bff-generator generate \
  --config bff-config.yaml \
  --suite accounting \
  --generate-service \
  --brrtrouter-path ../BRRTRouter/target/debug/brrtrouter-gen \
  --service-output microservices/bff/accountingBFF
```

## Migration Path

### Phase 1: Extract and Package

1. Extract BFF generation logic from PriceWhisperer and RERP
2. Create `bff-generator` Python package
3. Implement configuration system
4. Add CLI interface
5. Publish to GitHub (or PyPI if public)

### Phase 2: Update PriceWhisperer

1. Create `bff-config.yaml` for PriceWhisperer
2. Update Tiltfile to use `bff-generator` CLI
3. Remove old `generate_bff_spec.py` script
4. Test and verify

### Phase 3: Update RERP

1. Create per-suite `bff-config.yaml` files
2. Update Tiltfile to generate BFFs for each suite
3. Remove old `generate_bff_spec.py` script
4. Test and verify

### Phase 4: Enhancements

1. Add automatic service discovery
2. Integrate with port registry
3. Add BFF service code generation
4. Add validation and linting

## Benefits of Externalization

1. **Reusability**: Single tool for all Microscaler systems
2. **Maintainability**: Centralized BFF generation logic
3. **Consistency**: Same generation logic across systems
4. **Versioning**: Can version BFF generator independently
5. **Testing**: Can test BFF generation in isolation
6. **Documentation**: Centralized documentation
7. **Community**: Can be open-sourced if desired

## Package Distribution

### Option A: GitHub Releases

- Release Python wheels via GitHub Releases
- Install via: `pip install https://github.com/microscaler/bff-generator/releases/download/v1.0.0/bff-generator-1.0.0-py3-none-any.whl`

### Option B: Git Dependency

- Install via: `pip install git+https://github.com/microscaler/bff-generator.git@v1.0.0`

### Option C: PyPI (if public)

- Publish to PyPI: `pip install bff-generator`

## Dependencies

**Python Dependencies**:
- `pyyaml` - YAML parsing
- `click` or `argparse` - CLI interface
- `jsonschema` - Configuration validation (optional)
- `pydantic` - Configuration models (optional)

**External Dependencies**:
- BRRTRouter (for BFF service generation, optional)

## Testing Strategy

1. **Unit Tests**: Test individual functions (discovery, generation, schema merging)
2. **Integration Tests**: Test with real OpenAPI specs
3. **Fixture Tests**: Test with sample PriceWhisperer and RERP structures
4. **Regression Tests**: Ensure idempotency and deterministic output

## Documentation

1. **README.md**: Overview, installation, quick start
2. **USAGE.md**: Detailed usage guide
3. **CONFIGURATION.md**: Configuration file reference
4. **ARCHITECTURE.md**: Architecture support (single vs multi-suite)
5. **EXAMPLES/**: Example configurations for PriceWhisperer and RERP

## Open Questions

1. **License**: Should this be open source (PolyForm Shield) or proprietary?
   - **Recommendation**: Open source (PolyForm Shield) - it's infrastructure tooling

2. **Versioning**: Semantic versioning (v1.0.0, v1.1.0, v2.0.0)?
   - **Recommendation**: Yes, semantic versioning

3. **BRRTRouter Integration**: Should BFF service generation be part of this tool or separate?
   - **Recommendation**: Optional feature - can generate spec only, or spec + service

4. **Configuration Format**: YAML or TOML?
   - **Recommendation**: YAML (already used, more readable for complex configs)

5. **Backward Compatibility**: Should it support existing scripts' behavior exactly?
   - **Recommendation**: Yes, maintain idempotent clobber behavior

## Recommendation Summary

**✅ Python Package (PyPI/GitHub) - Option 1**

**Rationale**:
- Python is already used for BFF generation
- Easy to package and distribute
- Can be used as CLI or library
- Well-understood by developers
- Supports both architectures (single/multi-suite)
- Can integrate with existing workflows

**Implementation Priority**:
1. Extract and package core generation logic
2. Add configuration system
3. Add CLI interface
4. Support both architectures
5. Add BFF service generation (optional)
6. Integrate with existing systems

---

**Last Updated**: 2026-01-23  
**Status**: Design Proposal - Awaiting Approval
