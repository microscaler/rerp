# RERP Components Structure

This document describes the crate structure for RERP, following the PriceWhisperer pattern.

## Overview

RERP uses a Rust workspace with **142 crates** (71 services × 2 crates each):
- **71 Generated Crates**: Auto-generated from OpenAPI specs
- **71 Implementation Crates**: Business logic implementations
- **1 Common Crate**: Shared utilities

## Directory Structure

```
components/
├── Cargo.toml                    # Workspace configuration
├── README.md                      # This file
├── STRUCTURE.md                   # Structure documentation
├── common/                        # Shared utilities crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── error.rs
│       ├── validation.rs
│       └── utils.rs
└── {system}/                      # System directory (e.g., auth, accounting)
    ├── {module}/                  # Generated crate
    │   ├── Cargo.toml
    │   ├── doc/
    │   │   └── openapi.yaml       # Symlink to ../../openapi/{system}/{module}/openapi.yaml
    │   └── src/
    │       └── lib.rs             # Placeholder (will be generated)
    └── {module}_impl/             # Implementation crate
        ├── Cargo.toml
        ├── config/
        │   └── config.yaml
        └── src/
            ├── main.rs
            └── controllers/
                ├── mod.rs
                └── example.rs
```

## Crate Naming

- **Generated crates**: `rerp_{system}_{module}`
  - Example: `rerp_auth_idam`, `rerp_accounting_general_ledger`
- **Implementation crates**: `rerp_{system}_{module}_impl`
  - Example: `rerp_auth_idam_impl`, `rerp_accounting_general_ledger_impl`
- **Common crate**: `rerp_common`

## PriceWhisperer Pattern

RERP follows the same structure as PriceWhisperer:

### PriceWhisperer Structure
```
microservices/
├── Cargo.toml                     # Workspace
├── trader/
│   ├── billing/                   # Generated crate
│   │   ├── Cargo.toml
│   │   ├── doc/openapi.yaml
│   │   └── src/                   # Generated code
│   └── billing_impl/              # Implementation crate
│       ├── Cargo.toml
│       ├── config/config.yaml
│       └── src/
│           ├── main.rs
│           └── controllers/       # Business logic
└── common/                         # Shared utilities
```

### RERP Structure
```
components/
├── Cargo.toml                     # Workspace
├── accounting/
│   ├── general-ledger/            # Generated crate
│   │   ├── Cargo.toml
│   │   ├── doc/openapi.yaml        # Link to ../../openapi/accounting/general-ledger/openapi.yaml
│   │   └── src/                    # Generated code
│   └── general-ledger_impl/       # Implementation crate
│       ├── Cargo.toml
│       ├── config/config.yaml
│       └── src/
│           ├── main.rs
│           └── controllers/        # Business logic
└── common/                         # Shared utilities
```

## Workspace Dependencies

All crates share workspace dependencies defined in `components/Cargo.toml`:

```toml
[workspace.dependencies]
brrtrouter = { path = "../../BRRTRouter" }
brrtrouter_macros = { path = "../../BRRTRouter/brrtrouter_macros" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"
config = "0.14"
http = "1.0"
may = "0.3"
may_minihttp = { git = "https://github.com/microscaler/may_minihttp.git", branch = "feat/configurable-max-headers" }
anyhow = "1.0"
clap = { version = "4.5.39", features = ["derive"] }
tikv-jemallocator = { version = "0.6", features = ["profiling"] }
```

## Code Generation Workflow

1. **Define OpenAPI Spec**: Create/update `openapi/{system}/{module}/openapi.yaml`
2. **Generate Code**: Run `brrtrouter-gen` to generate handlers, types, registry
3. **Implement Logic**: Add business logic in `{system}/{module}_impl/src/controllers/`
4. **Build & Test**: Use `cargo build` and `cargo test` in workspace

## Service Implementation Pattern

Each service follows this pattern:

1. **Generated Crate** (`{system}/{module}/`):
   - Auto-generated from OpenAPI spec
   - Contains: handlers, types, registry
   - DO NOT manually edit (will be overwritten)

2. **Implementation Crate** (`{system}/{module}_impl/`):
   - Contains business logic
   - Imports generated crate
   - Implements controllers that use generated handlers
   - Safe to edit freely

## Example: Auth IDAM Service

```
components/
├── auth/
│   ├── idam/                      # Generated crate
│   │   ├── Cargo.toml
│   │   ├── doc/
│   │   │   └── openapi.yaml       # Links to ../../openapi/auth/idam/openapi.yaml
│   │   └── src/
│   │       ├── lib.rs             # Generated
│   │       ├── handlers/         # Generated
│   │       ├── types.rs          # Generated
│   │       └── registry.rs       # Generated
│   └── idam_impl/                 # Implementation crate
│       ├── Cargo.toml
│       ├── config/
│       │   └── config.yaml
│       └── src/
│           ├── main.rs           # Service entry point
│           └── controllers/
│               ├── mod.rs
│               ├── login.rs       # Business logic
│               └── register.rs    # Business logic
```

## Next Steps

1. ✅ Workspace structure created
2. ✅ Cargo.toml files generated for all crates
3. ✅ Placeholder source files created
4. ⏳ Generate code from OpenAPI specs using BRRTRouter
5. ⏳ Implement business logic in controllers
6. ⏳ Add service-specific dependencies as needed

---

**Status**: Workspace structure complete - Ready for code generation
