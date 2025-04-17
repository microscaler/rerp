# Agent Guide for RERP Development

This document provides agentic AI systems with essential context and references for working on the RERP project.

---

## Project Overview

**RERP** (Rust Enterprise Resource Planning) is a cloud-native, microservices-based ERP system built with Rust. The project consists of **71 independent microservices**, each with its own OpenAPI specification, organized into a Rust workspace with 142 crates (71 generated + 71 implementation).

---

## Key Architecture Principles

1. **OpenAPI-First**: All services are defined in OpenAPI 3.1.0 specifications
2. **Code Generation**: Services are generated from OpenAPI specs using BRRTRouter
3. **Two-Crate Model**: Each service has a generated crate and an implementation crate
4. **Microservices**: Independent, deployable services with well-defined APIs
5. **System-Level BFF**: Auto-generated Backend-for-Frontend specs aggregate sub-services

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

### 4. System BFF Generation

```bash
# Regenerate all system-level BFF specs
python3 scripts/generate_system_bff.py
```

This automatically:
- Aggregates sub-service paths
- Prefixes schemas to avoid conflicts
- Updates references
- Adds routing metadata

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

- `.github/workflows/generate-bff-specs.yml` - Auto-generates BFF specs on changes

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
- ✅ **BFF Generation**: Auto-generated system-level specs
- ✅ **CI Automation**: GitHub Actions for BFF generation
- ⏳ **Code Generation**: Services ready for BRRTRouter generation
- ⏳ **Implementation**: Business logic implementation in progress

---

## Best Practices for Agents

1. **Always check AI docs first**: Review relevant documents in `docs/ai/` before making changes
2. **Follow the two-crate model**: Don't modify generated crates directly
3. **Update OpenAPI specs**: Changes to APIs should be in OpenAPI specs first
4. **Regenerate BFF specs**: After updating service specs, regenerate system BFF specs
5. **Test changes**: Ensure all tests pass before committing
6. **Update documentation**: Keep AI planning docs updated with status changes

---

## Quick Reference

```bash
# View all services
ls openapi/*/openapi.yaml

# Generate BFF specs
python3 scripts/generate_system_bff.py

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
