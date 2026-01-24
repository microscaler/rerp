# Agent Guide for RERP Development

This document provides agentic AI systems with essential context and references for working on the RERP project.

---

## Project Overview

**RERP** (Rust Enterprise Resource Planning) is a cloud-native, microservices-based ERP system built with Rust. The project consists of **71 independent microservices**, each with its own OpenAPI specification, organized into a Rust workspace with 142 crates (71 generated + 71 implementation).

---

## Suites and BFF (Important)

**RERP is composed of suites of systems.** Each **suite** has:

- **Microservices**: `openapi/{suite}/{name}/openapi.yaml` and corresponding `microservices/` / `components/` crates
- **One BFF per suite**: `openapi/{suite}/bff-suite-config.yaml` and generated `openapi/{suite}/openapi_bff.yaml` that aggregates that suite’s services

Example: the **accounting** suite has microservices (general-ledger, invoice, accounts-receivable, accounts-payable, bank-sync, asset, budget, edi, financial-reports) and one **BFF** that fronts them. Other suites (e.g. HR, sales) will have their own BFF when implemented.

Scripts infer suites **dynamically**: they list `openapi/` subdirs that contain `bff-suite-config.yaml`, read `bff_service_name` from each config, and walk `openapi/{suite}/{name}/openapi.yaml` for microservices. **No hardcoded suite names or BFF mappings** in `assign-port.py`. When adding a new suite with a BFF, add `openapi/{suite}/bff-suite-config.yaml` with `bff_service_name` and the script will pick it up.

---

## Key Architecture Principles

1. **OpenAPI-First**: All services are defined in OpenAPI 3.1.0 specifications
2. **Code Generation**: Services are generated from OpenAPI specs using BRRTRouter
3. **Two-Crate Model**: Each service has a generated crate and an implementation crate
4. **Microservices**: Independent, deployable services with well-defined APIs
5. **Suite-Level BFF**: Each suite has one BFF; `openapi/{suite}/bff-suite-config.yaml` and `openapi_bff.yaml` are per-suite

---

## Agentic AI Planning & Analysis Documents

All planning, analysis, and implementation status documents are located in [`docs/ai/`](docs/ai/):

### Implementation Status

- **[docs/ai/OPENAPI_GENERATION_COMPLETE.md](docs/ai/OPENAPI_GENERATION_COMPLETE.md)** - OpenAPI specification generation status for all 71 services
- **[docs/ai/BFF_GENERATION_COMPLETE.md](docs/ai/BFF_GENERATION_COMPLETE.md)** - System-level BFF generation completion status
- **[docs/ai/FIRST_CI_AUTOMATION.md](docs/ai/FIRST_CI_AUTOMATION.md)** - First CI automation implementation and status

### Architecture & Planning

- **[docs/ai/SYSTEM_BFF_GENERATION.md](docs/ai/SYSTEM_BFF_GENERATION.md)** - System-level BFF generation architecture and process
- **[docs/ai/TOP_LEVEL_SPECS_PLAN.md](docs/ai/TOP_LEVEL_SPECS_PLAN.md)** - Planning document for top-level OpenAPI specifications
- **[docs/ai/CI_AUTOMATION_SETUP.md](docs/ai/CI_AUTOMATION_SETUP.md)** - CI/CD automation setup and configuration

### Analysis & Research

- **[docs/ai/ODOO_MODULES_ANALYSIS.md](docs/ai/ODOO_MODULES_ANALYSIS.md)** - Comprehensive analysis of Odoo modules and architecture patterns
- **[docs/ai/MICROSERVICE_MATRIX_AUDIT.md](docs/ai/MICROSERVICE_MATRIX_AUDIT.md)** - Microservice matrix audit and service organization analysis

---

## Development Workflow for Agents

### 1. Understanding the Project

1. Read [README.md](README.md) for project overview
2. Review [RERP_MUSINGS.md](RERP_MUSINGS.md) for module breakdown and market analysis
3. Check [components/README.md](components/README.md) for crate structure
4. Review relevant AI planning documents in `docs/ai/`

### 2. Working with Services

Each service follows this structure:
- **Generated crate**: `components/{system}/{module}/` - Auto-generated from OpenAPI
- **Implementation crate**: `components/{system}/{module}_impl/` - Business logic
- **OpenAPI spec**: `openapi/{system}/{module}/openapi.yaml`

### 3. Code Generation

```bash
# Generate code from OpenAPI spec
cd components/{system}/{module}
brrtrouter-gen --spec ../../openapi/{system}/{module}/openapi.yaml
```

### 4. Suite BFF Generation

Each suite has its own BFF. For the **accounting** suite:

```bash
bff-generator generate-spec --config openapi/accounting/bff-suite-config.yaml --output openapi/accounting/openapi_bff.yaml
```

For other suites, use `openapi/{suite}/bff-suite-config.yaml` and `openapi/{suite}/openapi_bff.yaml`. The `generate_system_bff.py` and CI workflows may generate one or more suite BFFs.

---

## Important File Locations

### Core Project Files

- `components/Cargo.toml` - Workspace root configuration
- `components/common/` - Shared utilities crate
- `openapi/` - All OpenAPI specifications
- `scripts/` - Generation and automation scripts

### Documentation

- `README.md` - User-facing project overview
- `CONTRIBUTING.md` - Developer contribution guidelines
- `RERP_MUSINGS.md` - Module breakdown and market analysis
- `docs/EXECUTIVE_SUMMARY.md` - Executive overview
- `docs/adrs/` - Architecture Decision Records

### CI/CD

- `.github/workflows/ci.yml` - Validate OpenAPI (incl. BFF generation dry run), build, test, multi-arch

---

## Service Organization

RERP services are organized into **6 implementation phases**:

1. **Phase 1: Core Foundation** (7 services) - Auth, infrastructure, product
2. **Phase 2: Business Operations** (14 services) - CRM, sales, purchase, inventory
3. **Phase 3: Financial & HR** (16 services) - Accounting, HR
4. **Phase 4: Advanced Operations** (7 services) - Manufacturing, project
5. **Phase 5: Customer-Facing** (10 services) - Marketing, website, POS, helpdesk
6. **Phase 6: Extensions** (5 services) - Marketplace, analytics

Plus **12 additional services** for AI, automation, data, documents, ESG, IoT, and localization.

---

## Key Scripts

### OpenAPI Generation

- `scripts/generate_complete_openapi.py` - Generates individual service OpenAPI specs
- `scripts/generate_system_bff.py` - Generates system-level BFF specs

### Requirements

- `scripts/requirements.txt` - Python dependencies for generation scripts

---

## Current Project Status

- ✅ **71 Services Defined**: Complete OpenAPI specifications
- ✅ **Crate Structure**: 142 crates organized in workspace
- ✅ **BFF Generation**: Auto-generated BFF per suite (accounting has `openapi_bff.yaml`)
- ✅ **CI Automation**: GitHub Actions for BFF generation
- ⏳ **Code Generation**: Services ready for BRRTRouter generation
- ⏳ **Implementation**: Business logic implementation in progress

---

## Best Practices for Agents

1. **Always check AI docs first**: Review relevant documents in `docs/ai/` before making changes
2. **Follow the two-crate model**: Don't modify generated crates directly
3. **Update OpenAPI specs**: Changes to APIs should be in OpenAPI specs first
4. **Regenerate BFF specs**: After updating service specs in a suite, regenerate that suite’s BFF (`openapi/{suite}/openapi_bff.yaml` from `bff-suite-config.yaml`)
5. **Test changes**: Ensure all tests pass before committing
6. **Update documentation**: Keep AI planning docs updated with status changes

---

## Lifeguard Entities and Indices (entities/)

RERP uses **Lifeguard** (`lifeguard_derive::LifeModel`) for ORM entities in `entities/`. Migration SQL is generated from these structs. Follow these rules to avoid broken migrations:

### Index and column rules

- **`#[index = "idx_name(column)"]` and `#[indexed]` must reference only columns that exist on that struct.**  
  The derive and migration generator do not validate that index columns exist. Indices on non-existent columns produce `CREATE INDEX` on a missing column and **migration application fails** (e.g. PostgreSQL: "column does not exist").

- **Child/specialized entities that link to a base entity via a foreign key do not inherit the base entity’s columns.**  
  Example: `CustomerInvoice` and `VendorInvoice` link to the base `invoices` table via `invoice_id`. They do **not** have `invoice_number`, `due_date`, `status`, or `payment_state`; those exist only on `invoices`.  
  - ✅ Index on `customer_id` on `CustomerInvoice` (column exists on the struct).  
  - ❌ Index on `invoice_number` on `CustomerInvoice` (column exists only on `invoices`).

- **To query by base-entity fields on a child table, use a JOIN.**  
  Index the child’s own columns (e.g. `customer_id`, `vendor_id`, `invoice_id` if you need faster lookups from child to parent). Put indices for `invoice_number`, `due_date`, `status`, etc. on the base `Invoice` entity only.

### Checklist when adding or changing entities

1. List the struct’s fields. Every `#[index = "idx_name(x)"]` and `#[indexed]` must use one of those names.
2. If the struct is a child of another (e.g. `invoice_id` → `invoices`), do not copy indices from the parent; only index columns that exist on the child.
3. After changing indices, run migration generation (e.g. from the `entities/` directory: `cargo run --bin generate-migrations`) and apply migrations in a test DB to confirm they apply cleanly.

---

## Quick Reference

```bash
# View all services
ls openapi/*/openapi.yaml

# Generate BFF for a suite (e.g. accounting)
bff-generator generate-spec --config openapi/accounting/bff-suite-config.yaml --output openapi/accounting/openapi_bff.yaml

# Build workspace
cd components && cargo build --workspace

# Run tests
cd components && cargo test --workspace
```

---

## Questions or Issues?

- Review AI planning documents in `docs/ai/`
- Check [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines
- Review [components/README.md](components/README.md) for crate structure

---

*This document is maintained for agentic AI systems working on RERP development.*
