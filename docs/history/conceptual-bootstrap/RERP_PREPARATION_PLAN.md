# RERP Preparation Plan

> **Status: HISTORICAL_SNAPSHOT** — January 2026 bootstrap plan based on the
> retired PriceWhisperer/root-entity layout. Current suite and service ownership
> is defined by [`CONTRIBUTING.md`](../../../CONTRIBUTING.md) and the
> [documentation authority index](../../README.md).

## Executive Summary

This document outlines the preparation plan for setting up RERP's microservices architecture based on PriceWhisperer's proven structure. The plan covers BRRTRouter crate organization, database entity placement, business logic implementation, Kubernetes configurations, Helm charts, and supporting scripts.

**Status**: In Progress - Phase 2 Complete  
**Created**: 2026-01-23  
**Last Updated**: 2026-01-23  
**Based On**: PriceWhisperer microservices architecture

**Progress**:
- ✅ Phase 1: Foundation Setup - Complete
- ✅ Phase 2: Entity Migration - Complete (2026-01-23)
- 🔄 Phase 3: First Service Setup - In Progress

---

## 1. Directory Structure Overview

### 1.1 PriceWhisperer Structure (Reference)

```
PriceWhisperer/
├── microservices/                    # All microservice code
│   ├── Cargo.toml                    # Workspace definition
│   ├── trader/                       # Service implementations
│   │   ├── billing/                  # Generated library crate
│   │   │   ├── src/
│   │   │   │   ├── lib.rs            # ⚠️ GENERATED
│   │   │   │   ├── registry.rs       # ⚠️ GENERATED
│   │   │   │   ├── handlers/         # ⚠️ GENERATED
│   │   │   │   └── controllers/     # ⚠️ GENERATED (fallback)
│   │   │   └── Cargo.toml            # ⚠️ GENERATED
│   │   ├── billing_impl/             # User-owned implementation crate
│   │   │   ├── src/
│   │   │   │   ├── main.rs           # ✅ USER-OWNED
│   │   │   │   └── controllers/      # ✅ USER-OWNED
│   │   │   └── Cargo.toml            # ✅ USER-OWNED
│   │   └── common/                   # Shared utilities
│   ├── bff/                          # Backend for Frontend
│   └── openapi/                      # OpenAPI specifications
├── helm/
│   └── pricewhisperer-microservice/  # Helm chart template
│       ├── Chart.yaml
│       ├── values.yaml                # Default values
│       ├── values/                    # Service-specific values
│       │   ├── billing.yaml
│       │   └── ...
│       └── templates/                 # K8s resource templates
│           ├── deployment.yaml
│           ├── service.yaml
│           └── configmap.yaml
├── k8s/                               # Kubernetes manifests
│   ├── microservices/
│   │   ├── namespace.yaml
│   │   └── kustomization.yaml
│   └── data/                          # Infrastructure (Postgres, Redis, etc.)
├── scripts/                           # Automation scripts
│   ├── bootstrap_microservice.py      # Service bootstrap
│   ├── build-microservice.sh          # Build script
│   └── ...
└── Tiltfile                           # Tilt configuration
```

### 1.2 Proposed RERP Structure

```
rerp/
├── microservices/                     # All microservice code
│   ├── Cargo.toml                     # Workspace definition
│   ├── accounting/                    # Accounting service implementations
│   │   ├── general-ledger/            # Generated library crate
│   │   │   ├── src/
│   │   │   │   ├── lib.rs             # ⚠️ GENERATED
│   │   │   │   ├── registry.rs        # ⚠️ GENERATED
│   │   │   │   ├── handlers/          # ⚠️ GENERATED
│   │   │   │   └── controllers/       # ⚠️ GENERATED (fallback)
│   │   │   └── Cargo.toml             # ⚠️ GENERATED
│   │   ├── general-ledger_impl/      # User-owned implementation crate
│   │   │   ├── src/
│   │   │   │   ├── main.rs            # ✅ USER-OWNED
│   │   │   │   ├── controllers/       # ✅ USER-OWNED
│   │   │   │   └── services/          # ✅ USER-OWNED (business logic)
│   │   │   └── Cargo.toml             # ✅ USER-OWNED
│   │   ├── invoice/                   # Generated library crate
│   │   ├── invoice_impl/              # User-owned implementation crate
│   │   ├── accounts-receivable/       # Generated library crate
│   │   ├── accounts-receivable_impl/  # User-owned implementation crate
│   │   └── common/                    # Shared utilities (JWT, validation, etc.)
│   ├── bff/                           # Backend for Frontend (optional)
│   └── openapi/                       # OpenAPI specifications
│       └── accounting/
│           ├── general-ledger/
│           │   └── openapi.yaml
│           ├── invoice/
│           │   └── openapi.yaml
│           └── ...
├── entities/                          # Database entities (Lifeguard)
│   ├── Cargo.toml                     # Entities workspace
│   └── src/
│       └── accounting/
│           ├── mod.rs
│           ├── general_ledger/
│           │   ├── mod.rs
│           │   ├── chart_of_accounts.rs
│           │   ├── account.rs
│           │   └── journal_entry.rs
│           ├── invoice/
│           └── ...
├── helm/
│   └── rerp-microservice/             # Helm chart template
│       ├── Chart.yaml
│       ├── values.yaml                 # Default values
│       ├── values/                     # Service-specific values
│       │   ├── general-ledger.yaml
│       │   ├── invoice.yaml
│       │   └── ...
│       └── templates/                  # K8s resource templates
│           ├── deployment.yaml
│           ├── service.yaml
│           └── configmap.yaml
├── k8s/                                # Kubernetes manifests
│   ├── microservices/
│   │   ├── namespace.yaml
│   │   └── kustomization.yaml
│   └── data/                           # Infrastructure (Postgres, Redis, etc.)
├── scripts/                            # Automation scripts
│   ├── bootstrap_microservice.py       # Service bootstrap (copy from PriceWhisperer)
│   ├── build-microservice.sh           # Build script (copy from PriceWhisperer)
│   └── ...
└── Tiltfile                            # Tilt configuration
```

---

## 2. BRRTRouter Crate Organization

### 2.1 Generated Library Crates

**Location**: `microservices/accounting/<service-name>/`

**Purpose**: Contains all BRRTRouter-generated code from OpenAPI specifications.

**Structure**:
```
general-ledger/
├── src/
│   ├── lib.rs              # ⚠️ GENERATED - Library entry point
│   ├── registry.rs         # ⚠️ GENERATED - Handler registry
│   ├── handlers/           # ⚠️ GENERATED - Request/Response types
│   │   ├── mod.rs
│   │   ├── listAccounts.rs
│   │   ├── createAccount.rs
│   │   └── ...
│   └── controllers/        # ⚠️ GENERATED - Fallback controllers with example data
│       ├── mod.rs
│       ├── listAccounts.rs
│       └── ...
├── config/
│   └── config.yaml         # Configuration template
├── doc/
│   └── openapi.yaml       # OpenAPI spec (copied from source)
└── Cargo.toml             # ⚠️ GENERATED - Library crate definition
```

**Key Characteristics**:
- **Library crate** (`lib.rs`, not `main.rs`)
- **Auto-generated** - Do not modify manually
- **Fallback controllers** - Return example data from OpenAPI spec if no implementation exists
- **Optional dependency** on `{service}_impl` crate

### 2.2 Implementation Crates

**Location**: `microservices/accounting/<service-name>_impl/`

**Purpose**: Contains all user-owned business logic and controller implementations.

**Structure**:
```
general-ledger_impl/
├── src/
│   ├── main.rs            # ✅ USER-OWNED - Binary entry point
│   ├── controllers/        # ✅ USER-OWNED - Controller implementations
│   │   ├── mod.rs
│   │   ├── listAccounts.rs
│   │   ├── createAccount.rs
│   │   └── ...
│   └── services/           # ✅ USER-OWNED - Business logic layer
│       ├── mod.rs
│       ├── account_service.rs
│       └── ...
└── Cargo.toml             # ✅ USER-OWNED - Binary crate definition
```

**Key Characteristics**:
- **Binary crate** (`main.rs`)
- **User-owned** - Safe to modify
- **Depends on** generated library crate
- **Iterative implementation** - Implement controllers one at a time
- **Business logic** - Services layer for complex operations

### 2.3 Dependency Flow

```
general-ledger_impl (binary)
    ↓ depends on
general-ledger (library, generated)
    ↓ depends on
brrtrouter, serde, etc. (workspace dependencies)
```

**Implementation Detection**:
- Generated controllers use **compile-time** conditional compilation
- `build.rs` detects if `{service}_impl` crate exists
- If exists → controllers use implementation
- If not → controllers use example data from OpenAPI spec
- **Per-controller detection** - Each controller independently checks for implementation

---

## 3. Database Entities Location

### 3.1 Previous Lifeguard Entities Location

**Previous Location**: `lifeguard/examples/entities/` ✅ **Moved**

**Status**: All accounting entities have been migrated to RERP. The Lifeguard examples directory now serves only for documentation and testing purposes.

**Note**: The `lifeguard/examples/entities/` directory has been updated to `example-entities` and no longer contains RERP-specific accounting entities.

### 3.2 RERP Entities Location

**Location**: `rerp/entities/` ✅ **Complete**

**Rationale**:
- **Separation of concerns** - Entities are domain models, not examples
- **Independent versioning** - Entities can be versioned separately
- **Reusability** - Entities can be used by multiple microservices
- **Clear ownership** - Entities belong to RERP, not Lifeguard

**Status**: ✅ **Migration Complete** (2026-01-23)

All 47 accounting entity files across 9 service domains have been moved from `lifeguard/examples/entities/src/accounting/` to `rerp/entities/src/accounting/`.

**Structure**:
```
rerp/entities/
├── Cargo.toml               # Entities workspace
├── build.rs                 # Entity registry generation (copy from lifeguard/examples/entities)
└── src/
    └── accounting/
        ├── mod.rs
        ├── general_ledger/
        │   ├── mod.rs
        │   ├── chart_of_accounts.rs
        │   ├── account.rs
        │   ├── journal_entry.rs
        │   └── account_balance.rs
        ├── invoice/
        │   ├── mod.rs
        │   ├── invoice.rs
        │   └── invoice_line.rs
        ├── accounts_receivable/
        ├── accounts_payable/
        ├── bank_sync/
        │   ├── mod.rs
        │   ├── bank.rs ✅ (newly created)
        │   ├── bank_account.rs ✅ (updated with bank_id FK)
        │   ├── bank_transaction.rs
        │   ├── bank_statement.rs
        │   └── bank_reconciliation.rs
        ├── asset/
        │   ├── mod.rs
        │   ├── asset.rs
        │   ├── asset_category.rs
        │   ├── asset_depreciation.rs
        │   └── asset_transaction.rs
        ├── budget/
        │   ├── mod.rs
        │   ├── budget.rs
        │   ├── budget_version.rs
        │   ├── budget_period.rs
        │   ├── budget_line_item.rs
        │   └── budget_actual.rs
        ├── edi/
        │   ├── mod.rs
        │   ├── edi_document.rs
        │   ├── edi_format.rs
        │   ├── edi_mapping.rs
        │   └── edi_acknowledgment.rs
        ├── financial_reports/
        │   ├── mod.rs
        │   ├── financial_report.rs
        │   ├── report_template.rs
        │   ├── report_schedule.rs
        │   └── report_data.rs
        └── ...
```

### 3.3 Entity Usage in Microservices

**In Implementation Crates** (`general-ledger_impl/`):

```rust
// Cargo.toml
[dependencies]
rerp_entities = { path = "../../entities" }
lifeguard = { path = "../../lifeguard" }

// In controller implementation
use rerp_entities::accounting::general_ledger::Account;
use lifeguard::{LifeModelTrait, LifeExecutor};

pub fn handle(req: TypedHandlerRequest<Request>) -> Response {
    // Use entities for database operations
    let accounts = Account::find()
        .filter(Expr::col("is_active").eq(true))
        .all(executor)?;
    
    // Convert to response
    Response { /* ... */ }
}
```

---

## 4. Business Logic Implementation

### 4.1 Controller Layer

**Location**: `microservices/accounting/{service}_impl/src/controllers/`

**Purpose**: HTTP request/response handling, validation, delegation to services.

**Example** (`listAccounts.rs`):
```rust
use brrtrouter_macros::handler;
use general_ledger::handlers::listAccounts::{Request, Response};
use general_ledger::brrtrouter::typed::TypedHandlerRequest;
use crate::services::account_service::AccountService;

#[handler(ListAccountsController)]
pub fn handle(req: TypedHandlerRequest<Request>) -> Response {
    // Extract query parameters
    let limit = req.inner.limit.unwrap_or(100);
    let offset = req.inner.offset.unwrap_or(0);
    
    // Delegate to service layer
    let accounts = AccountService::list_accounts(limit, offset)?;
    
    // Convert to response
    Response {
        items: accounts.into_iter().map(|a| /* convert */).collect(),
        total: accounts.len(),
    }
}
```

### 4.2 Service Layer

**Location**: `microservices/accounting/{service}_impl/src/services/`

**Purpose**: Business logic, database operations, external API calls.

**Example** (`account_service.rs`):
```rust
use rerp_entities::accounting::general_ledger::Account;
use lifeguard::{LifeModelTrait, LifeExecutor};
use sea_query::Expr;

pub struct AccountService;

impl AccountService {
    pub fn list_accounts(
        executor: &dyn LifeExecutor,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Account::Model>, Error> {
        Account::find()
            .filter(Expr::col("is_active").eq(true))
            .limit(limit)
            .offset(offset)
            .all(executor)
    }
    
    pub fn create_account(
        executor: &dyn LifeExecutor,
        data: CreateAccountData,
    ) -> Result<Account::Model, Error> {
        // Validation
        // Business rules
        // Database insert
    }
}
```

### 4.3 Common Utilities

**Location**: `microservices/accounting/common/`

**Purpose**: Shared code across accounting services (JWT validation, email validation, etc.).

**Structure**:
```
common/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── jwt.rs              # JWT validation
    ├── validation.rs        # Email, phone validation
    └── database.rs          # Database connection helpers
```

---

## 5. Kubernetes Configuration

### 5.1 Namespace Setup

**Location**: `k8s/microservices/namespace.yaml`

**Copy from**: `../PriceWhisperer/k8s/microservices/namespace.yaml`

**Content**:
```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: rerp
  labels:
    name: rerp
```

### 5.2 Kustomization

**Location**: `k8s/microservices/kustomization.yaml`

**Copy from**: `../PriceWhisperer/k8s/microservices/kustomization.yaml`

**Content**:
```yaml
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization

namespace: rerp

resources:
  - namespace.yaml
```

### 5.3 Service-Specific Manifests

**Location**: `k8s/microservices/{service}/`

**Structure** (if needed for service-specific configs):
```
k8s/microservices/
├── general-ledger/
│   └── kustomization.yaml  # Service-specific overrides
└── invoice/
    └── kustomization.yaml
```

**Note**: Most service configuration is handled via Helm charts. K8s manifests are primarily for infrastructure (Postgres, Redis, etc.).

---

## 6. Helm Chart Setup

### 6.1 Chart Structure

**Location**: `helm/rerp-microservice/`

**Copy from**: `../PriceWhisperer/helm/pricewhisperer-microservice/`

**Structure**:
```
helm/rerp-microservice/
├── Chart.yaml              # Chart metadata
├── values.yaml             # Default values
├── values/                  # Service-specific values
│   ├── general-ledger.yaml
│   ├── invoice.yaml
│   ├── accounts-receivable.yaml
│   └── ...
└── templates/               # K8s resource templates
    ├── deployment.yaml
    ├── service.yaml
    └── configmap.yaml
```

### 6.2 Chart.yaml

**Copy from**: `../PriceWhisperer/helm/pricewhisperer-microservice/Chart.yaml`

**Update**:
```yaml
apiVersion: v2
name: rerp-microservice
description: A Helm chart for RERP accounting microservices
type: application
version: 0.1.0
appVersion: "1.0.0"
```

### 6.3 Default values.yaml

**Copy from**: `../PriceWhisperer/helm/pricewhisperer-microservice/values.yaml`

**Key Sections**:
- **Service configuration**: Ports, container ports, NodePorts
- **Image configuration**: Repository, name, tag
- **Deployment configuration**: Replicas, namespace
- **Application configuration**: Binary name, service name, config
- **Resources**: Memory, CPU requests/limits
- **Health checks**: Liveness, readiness probes
- **Environment variables**: RUST_LOG, BRRTR_LOG_FORMAT, etc.

### 6.4 Service-Specific Values

**Location**: `helm/rerp-microservice/values/{service}.yaml`

**Example** (`general-ledger.yaml`):
```yaml
# ⚠️ PORT CONFIGURATION REFERENCE:
# Values file for General Ledger Service

service:
  name: general-ledger
  port: 8001
  containerPort: 8001
  nodePort: 30801

image:
  repository: localhost:5001
  name: rerp-general-ledger
  tag: tilt
  pullPolicy: IfNotPresent

deployment:
  replicas: 1
  namespace: rerp

app:
  binaryName: general_ledger_service_api
  serviceName: general-ledger
```

**Port Assignment Strategy**:
- **8001-8010**: Core accounting services (General Ledger, Invoice, AR, AP)
- **8011-8020**: Extended services (Bank Sync, Asset, Budget)
- **8021-8030**: Advanced services (Tax, Payment Terms, Analytic)
- **8031-8040**: Enterprise services (Consolidation, Document Import)
- **NodePorts**: 30801-30840 (matching service ports)

---

## 7. Scripts

### 7.1 Bootstrap Script

**Location**: `scripts/bootstrap_microservice.py`

**Copy from**: `../PriceWhisperer/scripts/bootstrap_microservice.py`

**Purpose**: 
- Creates crate directory structure
- Generates code using BRRTRouter
- Creates Dockerfile
- Updates workspace Cargo.toml
- Updates Tiltfile
- Creates config/config.yaml template

**Updates Needed**:
- Change service name patterns (billing → general-ledger, etc.)
- Update port assignments
- Update binary name patterns

### 7.2 Build Script

**Location**: `scripts/build-microservice.sh`

**Copy from**: `../PriceWhisperer/scripts/build-microservice.sh`

**Purpose**:
- Cross-compiles microservices for Linux (x86_64 musl)
- Handles macOS (zigbuild) vs Linux (musl-gcc) differences
- Builds individual services or entire workspace

**Updates Needed**:
- Update service name mappings
- Update package name mappings

**Service Name Mappings**:
```bash
declare -A PACKAGE_NAMES=(
  ["general-ledger"]="general_ledger_service_api"
  ["invoice"]="invoice_service_api"
  ["accounts-receivable"]="accounts_receivable_service_api"
  ["accounts-payable"]="accounts_payable_service_api"
  # ... etc
)
```

### 7.3 Other Scripts to Copy

**From**: `../PriceWhisperer/scripts/`

**Copy**:
- `setup-tilt.sh` - Tilt environment setup
- `teardown-tilt.sh` - Tilt cleanup
- `tail-tilt-logs.sh` - Log tailing
- `setup-persistent-volumes.sh` - Volume setup
- `cleanup-persistent-volumes.sh` - Volume cleanup

**Update**: Service names, namespace names, port assignments

---

## 8. Tiltfile Configuration

### 8.1 Tiltfile Structure

**Location**: `Tiltfile` (root of RERP repo)

**Copy from**: `../PriceWhisperer/Tiltfile`

**Key Sections**:
1. **Configuration** - Tilt settings, port assignments
2. **Monitoring Resources** - Prometheus, Grafana, Loki (optional)
3. **Pipeline Resources** - Fluvio (if needed)
4. **Data Stack Resources** - Postgres, Redis, etc.
5. **Microservices Namespace** - Load namespace
6. **Microservice Resources** - Individual service definitions

### 8.2 Service Definition Template

**For each accounting service**:

```python
# General Ledger Service
k8s_resource(
    'general-ledger',
    labels=['accounting'],
    port_forwards='8001:8001',
    resource_deps=['general-ledger-config'],
)

# Build and deploy
local_resource(
    'general-ledger-build',
    cmd='./scripts/build-microservice.sh general-ledger',
    deps=['microservices/accounting/general-ledger', 'microservices/accounting/general-ledger_impl'],
    resource_deps=['general-ledger'],
    labels=['accounting'],
)

# Helm deployment
helm_resource(
    'general-ledger',
    'helm/rerp-microservice',
    values=['helm/rerp-microservice/values/general-ledger.yaml'],
    resource_deps=['general-ledger-build'],
    labels=['accounting'],
)
```

### 8.3 Port Forwarding

**Port assignments** (from `docs/ACCOUNTING_SUITE_ENRICHMENT_PRD.md`):
- **8001**: General Ledger
- **8002**: Invoice
- **8003**: Accounts Receivable
- **8004**: Accounts Payable
- **8005**: Bank Sync
- **8006**: Asset
- **8007**: Budget
- **8008**: Tax (future)
- **8009**: Payment Terms (future)
- **8010**: Analytic (future)

---

## 9. Workspace Configuration

### 9.1 Microservices Workspace

**Location**: `microservices/Cargo.toml`

**Structure**:
```toml
[workspace]
members = [
    # Common utilities
    "accounting/common",
    
    # Generated library crates
    "accounting/general-ledger",
    "accounting/invoice",
    "accounting/accounts-receivable",
    "accounting/accounts-payable",
    "accounting/bank-sync",
    "accounting/asset",
    "accounting/budget",
    
    # Implementation crates
    "accounting/general-ledger_impl",
    "accounting/invoice_impl",
    "accounting/accounts-receivable_impl",
    "accounting/accounts-payable_impl",
    "accounting/bank-sync_impl",
    "accounting/asset_impl",
    "accounting/budget_impl",
    
    # BFF (if needed)
    "bff/accountingBFF",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"

[workspace.dependencies]
# BRRTRouter
brrtrouter = { path = "../../BRRTRouter" }
brrtrouter_macros = { path = "../../BRRTRouter/brrtrouter_macros" }

# Lifeguard ORM
lifeguard = { path = "../../lifeguard" }
lifeguard-derive = { path = "../../lifeguard/lifeguard-derive" }
rerp_entities = { path = "../../entities" }

# Standard dependencies
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"
config = "0.14"
http = "1.0"
may = "0.3"
may_minihttp = "0.1"
anyhow = "1.0"
clap = { version = "4.5", features = ["derive"] }
tikv-jemallocator = { version = "0.5", features = ["profiling"] }

# Database
may-postgres = "0.1"
rust_decimal = "1.33"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
```

### 9.2 Entities Workspace

**Location**: `entities/Cargo.toml`

**Structure**:
```toml
[package]
name = "rerp-entities"
version = "0.1.0"
edition = "2021"
build = "build.rs"

[lib]
name = "rerp_entities"
path = "src/lib.rs"

[dependencies]
lifeguard = { path = "../../lifeguard" }
lifeguard-derive = { path = "../../lifeguard/lifeguard-derive" }
lifeguard-migrate = { path = "../../lifeguard/lifeguard-migrate" }

uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
rust_decimal = { version = "1.33", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[build-dependencies]
lifeguard-migrate = { path = "../../lifeguard/lifeguard-migrate" }
```

---

## 10. Implementation Checklist

### Phase 1: Foundation Setup ✅ **COMPLETE**

- [x] **Create directory structure** ✅ **Complete**
  - [x] `rerp/microservices/`
  - [x] `rerp/entities/`
  - [x] `rerp/helm/rerp-microservice/`
  - [x] `rerp/k8s/microservices/`
  - [x] `rerp/scripts/`

- [x] **Copy and adapt scripts** ✅ **Complete**
  - [x] `bootstrap_microservice.py` (updated service names, ports, paths)
  - [x] `build-microservice.sh` (updated service mappings)
  - [x] `setup-tilt.sh` ✅ **Complete** (adapted for RERP services and ports)
  - [x] `teardown-tilt.sh` ✅ **Complete** (adapted for RERP services and images)
  - [x] `tail-tilt-logs.sh` ✅ **Complete** (adapted for RERP service names)
  - [x] `setup-persistent-volumes.sh` ✅ **Complete** (created for RERP)

- [x] **Set up Helm chart** ✅ **Complete**
  - [x] Copy `helm/pricewhisperer-microservice/` → `helm/rerp-microservice/`
  - [x] Update `Chart.yaml` (name: `rerp-microservice`)
  - [x] Update `values.yaml` (defaults, port mappings, namespace: `rerp`)
  - [x] Create service-specific values files (`general-ledger.yaml`)

- [x] **Set up Kubernetes manifests** ✅ **Complete**
  - [x] Copy `k8s/microservices/` structure
  - [x] Update namespace to `rerp`
  - [x] Create kustomization files

### Phase 2: Entity Migration ✅ **COMPLETE**

- [x] **Move entities from Lifeguard** ✅ **Complete** (2026-01-23)
  - [x] Copy `lifeguard/examples/entities/` → `rerp/entities/` (filesystem move)
  - [x] Update `Cargo.toml` (package name: `rerp-entities`, dependencies)
  - [x] Update module paths in code (`accounting_entities` → `rerp_entities`)
  - [x] Test entity compilation
  - [x] Generate SQL migrations (via `generate-sql` binary)

- [x] **Update entity references** ✅ **Complete**
  - [x] Update Lifeguard examples to note entities moved to RERP
  - [x] Update `lifeguard/examples/entities/` to `example-entities` package
  - [x] Document entity location in README (`rerp/entities/README.md`)

**Migration Summary**:
- **47 entity files** moved across **9 service domains**
- **Bank Sync** entities include new `Bank` entity and updated `BankAccount` with `bank_id` FK
- All entities verified and compiling in `rerp/entities/`
- See `docs/history/implementation-snapshots/ENTITY_MIGRATION_COMPLETE.md` for details

### Phase 3: First Service Setup 🔄 **IN PROGRESS**

- [x] **Bootstrap General Ledger service** ✅ **Complete**
  - [x] Run `bootstrap_microservice.py` for `general-ledger`
  - [x] Verify generated crate structure
  - [ ] Create `general-ledger_impl` crate (optional - can implement directly)
  - [x] Update workspace `Cargo.toml` (added `accounting/general-ledger`)
  - [x] Create Helm values file (`helm/rerp-microservice/values/general-ledger.yaml`)
  - [x] Update Tiltfile (added `general-ledger` service with lint, gen, build, deploy)

- [ ] **Implement first controller** 🔄 **Next Step**
  - [ ] Create `listAccounts` controller stub
  - [ ] Implement service layer
  - [ ] Connect to database using entities
  - [ ] Test locally

- [ ] **Deploy to Kubernetes** 🔄 **Pending**
  - [ ] Build Docker image
  - [ ] Deploy via Helm
  - [ ] Verify health checks
  - [ ] Test API endpoints

### Phase 4: Additional Services

- [ ] **Bootstrap remaining services** (one at a time)
  - [ ] Invoice
  - [ ] Accounts Receivable
  - [ ] Accounts Payable
  - [ ] Bank Sync
  - [ ] Asset
  - [ ] Budget

- [ ] **Implement controllers iteratively**
  - [ ] Start with CRUD operations
  - [ ] Add business logic
  - [ ] Add validation
  - [ ] Add error handling

### Phase 5: Common Utilities

- [ ] **Create common crate**
  - [ ] JWT validation
  - [ ] Email/phone validation
  - [ ] Database connection helpers
  - [ ] Error types

- [ ] **Integrate common utilities**
  - [ ] Update service implementations to use common
  - [ ] Remove duplicate code

---

## 11. Key Differences from PriceWhisperer

### 11.1 Service Naming

- **PriceWhisperer**: `billing`, `idam`, `amd`, `marketing`
- **RERP**: `general-ledger`, `invoice`, `accounts-receivable`, `accounts-payable`

**Impact**: Update all scripts, Helm values, Tiltfile, workspace Cargo.toml

### 11.2 Entity Location

- **PriceWhisperer**: Entities may be embedded in services or separate
- **RERP**: Entities in dedicated `entities/` directory (Lifeguard-based)

**Impact**: Update dependencies in implementation crates

### 11.3 Port Assignments

- **PriceWhisperer**: 8001-8006
- **RERP**: 8001-8010+ (more services planned)

**Impact**: Update Helm values, Tiltfile, service definitions

### 11.4 Domain Focus

- **PriceWhisperer**: Trading, education, billing, identity
- **RERP**: Accounting, financial management, ERP

**Impact**: Different business logic, different entity models

---

## 12. Open Questions

1. **BFF Strategy**: Do we need a Backend for Frontend service, or will services be called directly?
2. **Entity Versioning**: How should entities be versioned? Semantic versioning? Git tags?
3. **Database Migrations**: Where should migration files live? In `entities/` or separate `migrations/`?
4. **Service Discovery**: How will services discover each other? Service mesh? Direct calls?
5. **Authentication**: Will we use a separate IDAM service or integrate with existing system?
6. **Testing Strategy**: Unit tests, integration tests, contract tests (Pact)?
7. **CI/CD**: How will services be built and deployed? GitHub Actions? GitOps?

---

## 13. Next Steps

1. **Review this document** with stakeholders
2. **Approve directory structure** and naming conventions
3. **Begin Phase 1** (Foundation Setup)
4. **Iterate** on service implementation
5. **Document** learnings and update this plan

---

## 14. References

- **PriceWhisperer**: `../PriceWhisperer/`
- **BRRTRouter**: `../BRRTRouter/`
- **Lifeguard**: `../lifeguard/`
- **RERP Accounting PRD**: `docs/ACCOUNTING_SUITE_ENRICHMENT_PRD.md`
- **Bank Account PRD**: `docs/BANK_ACCOUNT_IMPROVEMENT_PRD.md`

---

**Status**: In Progress - Phase 2 Complete  
**Created**: 2026-01-23  
**Last Updated**: 2026-01-23  
**Author**: AI Assistant  
**Review Required**: Yes

**Recent Updates**:
- ✅ **2026-01-23**: Entity migration completed. All 47 accounting entity files moved from `lifeguard/examples/entities/` to `rerp/entities/`. See `docs/history/implementation-snapshots/ENTITY_MIGRATION_COMPLETE.md` for the historical details.
