# IDAM Service Architecture Analysis

## Executive Summary

This document analyzes the best approach for handling the IDAM (Identity and Access Management) microservice that is critical for all Microscaler systems (PriceWhisperer, RERP, and future systems).

## Current State: PriceWhisperer IDAM

### Current Architecture

**Location**: `pricewhisperer/microservices/trader/idam/`

**Structure**:
- Part of PriceWhisperer monorepo workspace
- Generated from OpenAPI spec using BRRTRouter
- Depends on `common` crate for shared utilities
- Standalone microservice running on port 8003
- HTTP-based service (other services call it via HTTP)

**Key Characteristics**:
- **Service Type**: HTTP microservice (not a library)
- **Communication**: REST API over HTTP
- **Dependencies**: 
  - `common` crate (shared utilities: email/phone validation, JWT, Redis)
  - BRRTRouter framework
  - Supabase GoTrue (via HTTP)
- **Integration**: Other services call IDAM via HTTP endpoints
- **Deployment**: Independent Kubernetes service

**OpenAPI Spec**: `microservices/openapi/trader/idam/openapi.yaml`

**Key Features**:
- Email/password authentication
- OAuth (Google, GitHub, SAML)
- Phone OTP verification
- Dual OTP (email + phone)
- Email/phone validation (via Abstract API)
- Session management (Redis)
- JWT token generation
- User identity management

## The Challenge

IDAM is **critical infrastructure** that needs to be:
1. **Shared across systems**: PriceWhisperer, RERP, and future Microscaler systems
2. **Consistently maintained**: Single source of truth for authentication
3. **Independently deployable**: Can be updated without affecting dependent systems
4. **Versioned appropriately**: Breaking changes need careful management

## Option Analysis

### Option 1: Separate Git Repository + Git Dependency ⭐ **RECOMMENDED**

**Approach**: Create `microscaler/idam` repository, reference via `Cargo.toml` git dependency

**Structure**:
```
microscaler/
├── idam/                    # Separate repository
│   ├── Cargo.toml
│   ├── src/
│   ├── openapi/
│   └── README.md
├── pricewhisperer/          # References idam via git
│   └── microservices/
│       └── Cargo.toml        # idam = { git = "..." }
├── rerp/                    # References idam via git
│   └── microservices/
│       └── Cargo.toml        # idam = { git = "..." }
└── future-system/           # References idam via git
```

**Cargo.toml Dependency**:
```toml
[dependencies]
idam = { git = "https://github.com/microscaler/idam", branch = "main" }
# Or for version pinning:
idam = { git = "https://github.com/microscaler/idam", tag = "v1.0.0" }
```

**Pros**:
- ✅ **True separation of concerns**: IDAM is independent infrastructure
- ✅ **Independent versioning**: Can version IDAM separately (v1.0.0, v2.0.0)
- ✅ **Independent deployment**: Deploy IDAM updates without touching dependent systems
- ✅ **Clear ownership**: IDAM repo has its own CI/CD, issues, releases
- ✅ **Reusability**: Any Microscaler system can depend on it
- ✅ **Semantic versioning**: Breaking changes can be managed via version tags
- ✅ **Git dependency flexibility**: Can pin to branch, tag, or commit
- ✅ **No code duplication**: Single source of truth
- ✅ **Works with HTTP services**: IDAM remains an HTTP service, not a library

**Cons**:
- ⚠️ **Git dependency complexity**: Requires git access, can be slower than path deps
- ⚠️ **Version management**: Need to coordinate version updates across systems
- ⚠️ **Breaking changes**: Require careful versioning and migration planning
- ⚠️ **CI/CD coordination**: Changes to IDAM may require updates to dependent systems

**Best For**:
- Critical shared infrastructure
- Services that need independent deployment
- Systems that need version control
- Long-term maintainability

---

### Option 2: Monorepo with Shared Directory

**Approach**: Create `microscaler-shared/idam/` or keep in one repo, symlink/copy to others

**Structure**:
```
microscaler-shared/
├── idam/
│   ├── Cargo.toml
│   └── src/
├── pricewhisperer/
│   └── microservices/
│       └── Cargo.toml        # idam = { path = "../../idam" }
└── rerp/
    └── microservices/
        └── Cargo.toml        # idam = { path = "../../idam" }
```

**Pros**:
- ✅ **Simple path dependencies**: Fast builds, easy local development
- ✅ **Atomic changes**: Can update IDAM and dependent systems in one commit
- ✅ **Easy refactoring**: IDE can navigate across the entire codebase
- ✅ **Single CI/CD**: One pipeline for all changes

**Cons**:
- ❌ **Tight coupling**: All systems must be in same repo structure
- ❌ **Deployment coupling**: Can't deploy IDAM independently
- ❌ **Versioning difficulty**: Hard to version IDAM separately
- ❌ **Repository bloat**: All systems in one repo can get large
- ❌ **Access control**: Hard to restrict access to specific systems
- ❌ **Breaking changes**: Affect all systems simultaneously

**Best For**:
- Tightly coupled systems
- Rapid prototyping
- Single team ownership

---

### Option 3: Published Crate (crates.io)

**Approach**: Publish IDAM as a crate on crates.io

**Structure**:
```
microscaler/idam/            # Separate repository
├── Cargo.toml               # name = "microscaler-idam"
└── src/

# In dependent systems:
[dependencies]
microscaler-idam = "1.0.0"   # Published version
```

**Pros**:
- ✅ **Standard Rust ecosystem**: Uses standard crate publishing
- ✅ **Version management**: Semantic versioning via crates.io
- ✅ **Public availability**: Can be used by external projects
- ✅ **Caching**: Cargo caches published crates

**Cons**:
- ❌ **Public exposure**: IDAM would be public (unless using private registry)
- ❌ **Publishing overhead**: Need to publish releases
- ❌ **Not suitable for HTTP services**: IDAM is a service, not a library
- ❌ **Version lag**: Updates require publishing new versions

**Best For**:
- Public libraries
- Not suitable for microservices

---

### Option 4: Git Submodule

**Approach**: Use git submodules to include IDAM in each repository

**Structure**:
```
pricewhisperer/
├── .gitmodules
└── microservices/
    └── idam/                # Git submodule
        └── Cargo.toml       # idam = { path = "../idam" }

rerp/
├── .gitmodules
└── microservices/
    └── idam/                # Git submodule (same repo)
        └── Cargo.toml
```

**Pros**:
- ✅ **Code sharing**: Single source of code
- ✅ **Version pinning**: Can pin to specific commits

**Cons**:
- ❌ **Git submodule complexity**: Developers must manage submodules
- ❌ **Update overhead**: Need to update submodules in each repo
- ❌ **Merge conflicts**: Submodule updates can cause conflicts
- ❌ **Not recommended**: Git submodules are generally discouraged

**Best For**:
- Legacy workflows
- Not recommended for new projects

---

## Recommendation: Option 1 (Separate Repository + Git Dependency) ⭐

### Why This Is The Best Fit

1. **IDAM is Infrastructure, Not Application Code**
   - IDAM is critical shared infrastructure (like a database or message queue)
   - It should be managed independently from application code
   - Similar to how you'd manage a shared authentication service in a microservices architecture

2. **Independent Deployment**
   - IDAM can be deployed, updated, and scaled independently
   - Breaking changes can be versioned (v1.0.0 → v2.0.0)
   - Dependent systems can upgrade at their own pace

3. **Clear Boundaries**
   - IDAM has its own repository, CI/CD, issues, and releases
   - Clear ownership and responsibility
   - Easier to maintain and document

4. **HTTP Service Architecture**
   - IDAM is an HTTP microservice, not a library
   - Other services call it via HTTP (not direct code dependencies)
   - Git dependency is for **building/deploying** IDAM, not for importing code
   - The actual runtime dependency is HTTP-based

5. **Future-Proof**
   - New Microscaler systems can easily adopt IDAM
   - Can eventually publish to private crate registry if needed
   - Can be open-sourced separately if desired

### Implementation Plan

#### Phase 1: Create IDAM Repository

```bash
# Create new repository
mkdir microscaler-idam
cd microscaler-idam
git init

# Copy IDAM from PriceWhisperer
cp -r ../pricewhisperer/microservices/trader/idam/* .
cp -r ../pricewhisperer/microservices/openapi/trader/idam ./openapi/

# Create standalone Cargo.toml
# Remove PriceWhisperer-specific dependencies
# Add proper metadata

git add .
git commit -m "Initial IDAM service extraction"
git remote add origin https://github.com/microscaler/idam.git
git push -u origin main
```

#### Phase 2: Update PriceWhisperer

```toml
# pricewhisperer/microservices/Cargo.toml
[workspace]
members = [
    "trader/common",
    "trader/marketing",
    # "trader/idam",  # Remove from workspace
    # ...
]

# For building/deploying IDAM:
[dependencies]
idam = { git = "https://github.com/microscaler/idam", branch = "main" }
```

#### Phase 3: Add to RERP

```toml
# rerp/microservices/Cargo.toml
[dependencies]
idam = { git = "https://github.com/microscaler/idam", branch = "main" }
```

#### Phase 4: Versioning Strategy

```bash
# Tag releases in IDAM repo
git tag -a v1.0.0 -m "Initial IDAM release"
git push origin v1.0.0

# Pin in dependent systems
idam = { git = "https://github.com/microscaler/idam", tag = "v1.0.0" }
```

### Important Considerations

#### 1. IDAM is an HTTP Service

**Key Point**: IDAM is a **microservice**, not a library. Other services call it via **HTTP**, not by importing Rust code.

**What the Git Dependency Is For**:
- Building the IDAM service binary
- Deploying IDAM as a Kubernetes service
- **NOT** for importing IDAM code into other services

**Runtime Architecture**:
```
┌─────────────────┐
│  PriceWhisperer │
│   Microservices │
└────────┬────────┘
         │ HTTP
         │ (port 8003)
         ▼
┌─────────────────┐
│  IDAM Service   │  ← Built from git dependency
│  (Kubernetes)   │
└─────────────────┘
         │
         ▼
┌─────────────────┐
│  RERP Services  │
│  (HTTP calls)   │
└─────────────────┘
```

#### 2. Common Crate Dependency

**Current State**: IDAM depends on `common` crate for:
- Email/phone validation
- JWT token handling
- Redis client
- Supabase GoTrue client

**Options**:
- **Option A**: Move `common` to IDAM repo (if IDAM-specific)
- **Option B**: Create `microscaler-common` repository (if shared across systems)
- **Option C**: Keep `common` in PriceWhisperer, IDAM depends on it via git

**Recommendation**: Evaluate `common` crate:
- If IDAM-specific → Move to IDAM repo
- If shared across systems → Create `microscaler-common` repo
- If PriceWhisperer-specific → Keep in PriceWhisperer, IDAM depends via git

#### 3. Version Management

**Strategy**: Semantic Versioning

- **v1.0.0**: Initial stable release
- **v1.1.0**: New features (backward compatible)
- **v2.0.0**: Breaking changes

**Dependent Systems**:
```toml
# Pin to specific version for stability
idam = { git = "https://github.com/microscaler/idam", tag = "v1.0.0" }

# Or track main branch for latest (development)
idam = { git = "https://github.com/microscaler/idam", branch = "main" }
```

#### 4. CI/CD Considerations

**IDAM Repository**:
- Build and test IDAM service
- Generate Docker images
- Deploy to Kubernetes (dev/staging/prod)

**Dependent Systems**:
- Build IDAM from git dependency
- Include IDAM in deployment manifests
- Test integration with IDAM service

**Coordination**:
- Breaking changes in IDAM require version bump
- Dependent systems update to new version
- Can test compatibility before upgrading

#### 5. OpenAPI Spec Management

**Current**: OpenAPI spec in PriceWhisperer repo

**After Extraction**:
- OpenAPI spec in IDAM repo
- IDAM service generated from spec
- BFF specs in each system reference IDAM endpoints

**BFF Integration**:
```yaml
# pricewhisperer/microservices/openapi/openapi_bff.yaml
paths:
  /api/identity/login:
    $ref: 'https://raw.githubusercontent.com/microscaler/idam/main/openapi/openapi.yaml#/paths/~1api~1identity~1login'
```

Or generate BFF specs that include IDAM paths.

## Alternative: Hybrid Approach

If git dependencies prove problematic, consider:

### Option 1B: Separate Repo + Published Artifact

**Approach**: 
- IDAM in separate repository
- Build and publish Docker images
- Dependent systems reference Docker images (not git)

**Pros**:
- No git dependency complexity
- Standard container-based deployment
- Version via image tags

**Cons**:
- Can't build IDAM from source in dependent repos
- Requires container registry

### Option 2B: Monorepo with Workspace

**Approach**:
- Create `microscaler-monorepo/` with all systems
- IDAM as workspace member
- All systems reference IDAM via path dependency

**Pros**:
- Simple path dependencies
- Atomic changes across systems
- Single CI/CD

**Cons**:
- All systems in one repo
- Deployment coupling
- Harder to manage access control

## Decision Matrix

| Criteria | Separate Repo + Git | Monorepo | Published Crate | Git Submodule |
|----------|---------------------|----------|-----------------|---------------|
| **Independence** | ✅✅✅ | ❌ | ✅✅✅ | ✅ |
| **Versioning** | ✅✅✅ | ❌ | ✅✅✅ | ⚠️ |
| **Deployment** | ✅✅✅ | ❌ | ✅✅ | ⚠️ |
| **Simplicity** | ⚠️ | ✅✅✅ | ✅✅ | ❌ |
| **Maintainability** | ✅✅✅ | ⚠️ | ✅✅ | ❌ |
| **Future-Proof** | ✅✅✅ | ⚠️ | ✅✅ | ❌ |
| **HTTP Service Fit** | ✅✅✅ | ✅✅✅ | ❌ | ✅✅ |

## Final Recommendation

**✅ Separate Git Repository + Git Dependency (Option 1)**

**Rationale**:
1. IDAM is critical shared infrastructure
2. Needs independent deployment and versioning
3. HTTP service architecture fits this model
4. Future-proof for additional Microscaler systems
5. Clear ownership and maintenance boundaries

**Next Steps**:
1. Create `microscaler/idam` repository
2. Extract IDAM from PriceWhisperer
3. Set up CI/CD for IDAM
4. Update PriceWhisperer to use git dependency
5. Add IDAM to RERP
6. Establish versioning strategy
7. Document integration patterns

---

**Last Updated**: 2026-01-23  
**Status**: Analysis Complete - Awaiting Decision
