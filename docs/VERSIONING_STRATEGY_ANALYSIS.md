# Versioning Strategy Analysis: Semantic Versioning vs. Kubernetes-Style API Versioning

## Executive Summary

**Recommendation: Use Semantic Versioning (X.Y.Z) for application releases, with optional Kubernetes-style API versioning for OpenAPI specs.**

For RERP as an enterprise application, semantic versioning is the appropriate choice for application releases. However, Kubernetes-style API versioning can be valuable for OpenAPI specifications to communicate API stability guarantees to enterprise customers.

---

## Understanding the Two Approaches

### Semantic Versioning (X.Y.Z)
**Purpose**: Version application/product releases

**Format**: `MAJOR.MINOR.PATCH`
- **MAJOR (X)**: Breaking changes (incompatible API changes, major architectural changes)
- **MINOR (Y)**: New features (backward compatible)
- **PATCH (Z)**: Bug fixes (backward compatible)

**Examples**: `1.0.0`, `1.2.3`, `2.0.0-rc.1`

**Use Cases**:
- Application releases
- Library/package versions
- Container images
- Binary distributions
- Dependency management

### Kubernetes-Style API Versioning (v1alpha1, v1beta1, v1)
**Purpose**: Version API contracts and communicate stability guarantees

**Format**: `v{version}{stability}{number}`
- **v1alpha1, v1alpha2**: Experimental APIs, breaking changes allowed
- **v1beta1, v1beta2**: More stable, but breaking changes still possible
- **v1**: Stable API, no breaking changes (only additive changes)

**Examples**: `/api/v1alpha1/accounts`, `/api/v1beta1/invoices`, `/api/v1/customers`

**Use Cases**:
- API endpoint versioning
- API contract stability communication
- OpenAPI specification versioning
- Service-to-service API contracts

---

## What RERP Needs to Version

### 1. Application Releases (Current: Semantic Versioning ✅)
- **Container images**: `ghcr.io/owner/rerp-accounting-general-ledger:0.39.0`
- **Binary distributions**: CLI tools, service binaries
- **Helm charts**: Chart versions
- **Git tags**: `v0.39.0`, `v0.39.0-rc.2`
- **GitHub Releases**: Release artifacts and notes

**Current State**: Using semantic versioning (X.Y.Z with RC support)

**Recommendation**: ✅ **Keep semantic versioning**
- Standard for enterprise applications
- Clear communication to customers about release types
- Works with dependency management (Cargo.toml)
- Industry standard (Docker, Kubernetes, most enterprise software)

### 2. OpenAPI Specifications (Current: Not explicitly versioned)
- **71 microservice APIs**: Each has `openapi.yaml`
- **BFF specs**: System-level aggregated APIs
- **API contracts**: What customers integrate with

**Current State**: OpenAPI specs exist but don't have explicit API versioning in paths

**Recommendation**: ⚠️ **Consider adding Kubernetes-style API versioning to OpenAPI paths**

---

## Detailed Analysis

### Option 1: Pure Semantic Versioning (Current Approach)

**Pros:**
- ✅ Simple and familiar to developers
- ✅ Standard for application releases
- ✅ Works well with dependency management (Cargo.toml)
- ✅ Clear release cadence (patch = bug fix, minor = feature, major = breaking)
- ✅ Industry standard (Docker, Kubernetes itself uses semver for releases)
- ✅ Works with container registries and package managers
- ✅ Clear communication: "Upgrade from 0.39.0 to 0.40.0 for new features"

**Cons:**
- ❌ Doesn't communicate API stability guarantees
- ❌ Enterprise customers can't tell if an API is stable or experimental
- ❌ Breaking API changes require major version bump (even if API is experimental)
- ❌ Doesn't distinguish between "application breaking change" vs "API breaking change"

**Example:**
```
Application: v0.39.0
API: /api/accounts (no version indicator)
→ Customer doesn't know if this API is stable or experimental
```

### Option 2: Kubernetes-Style API Versioning

**Pros:**
- ✅ Clear API stability communication
- ✅ Enterprise customers understand stability guarantees
- ✅ Can evolve APIs without breaking application versioning
- ✅ Industry standard for API versioning (Kubernetes, Istio, etc.)
- ✅ Supports multiple API versions simultaneously (v1alpha1, v1beta1, v1)

**Cons:**
- ❌ More complex versioning scheme
- ❌ Not standard for application releases
- ❌ Requires API path changes (`/api/v1alpha1/...` vs `/api/v1/...`)
- ❌ Customers need to understand alpha/beta/stable lifecycle
- ❌ Doesn't replace semantic versioning for application releases

**Example:**
```
Application: v0.39.0 (semantic versioning)
API: /api/v1alpha1/accounts (API versioning)
→ Customer knows this API is experimental
```

### Option 3: Hybrid Approach (Recommended)

**Application Releases**: Semantic Versioning (X.Y.Z)
- Container images: `rerp-accounting-general-ledger:0.39.0`
- Binaries: CLI tools, service executables
- Helm charts: Chart version `0.39.0`
- Git tags: `v0.39.0`

**API Contracts**: Kubernetes-Style Versioning (v1alpha1, v1beta1, v1)
- OpenAPI paths: `/api/v1alpha1/accounts`, `/api/v1beta1/invoices`, `/api/v1/customers`
- API stability communicated via path version
- Multiple API versions can coexist

**Benefits:**
- ✅ Application versioning remains simple and standard
- ✅ API stability clearly communicated
- ✅ Can evolve APIs independently of application releases
- ✅ Enterprise customers get clear stability guarantees
- ✅ Follows industry best practices (Kubernetes, Istio, etc.)

---

## Enterprise Application Context

### What Enterprise Customers Need

1. **Application Versioning** (Semantic Versioning):
   - "What version of RERP am I running?"
   - "Is this a major upgrade with breaking changes?"
   - "What's the latest stable release?"
   - **Answer**: `v0.39.0` (semantic version)

2. **API Stability** (Kubernetes-Style):
   - "Is this API stable or experimental?"
   - "Can I rely on this API not changing?"
   - "When will this API be stable?"
   - **Answer**: `/api/v1alpha1/...` (experimental) vs `/api/v1/...` (stable)

### Real-World Examples

**Kubernetes** (the project):
- **Application releases**: Semantic versioning (`v1.28.0`, `v1.29.0`)
- **API versioning**: Kubernetes-style (`/api/v1/pods`, `/apis/apps/v1/deployments`)

**Istio**:
- **Application releases**: Semantic versioning (`1.19.0`, `1.20.0`)
- **API versioning**: Kubernetes-style (`networking.istio.io/v1alpha3`, `networking.istio.io/v1beta1`)

**RERP** (proposed):
- **Application releases**: Semantic versioning (`0.39.0`, `0.40.0`)
- **API versioning**: Kubernetes-style (`/api/v1alpha1/accounts`, `/api/v1/accounts`)

---

## Recommendation for RERP

### Primary Strategy: Semantic Versioning for Application Releases ✅

**Keep your current approach** for:
- Container images
- Binary distributions
- Helm charts
- Git tags
- GitHub Releases

**Rationale:**
- Standard for enterprise applications
- Clear and familiar to customers
- Works with all tooling (Docker, Kubernetes, package managers)
- Industry standard

### Secondary Strategy: Consider API Versioning for OpenAPI Specs ⚠️

**Add Kubernetes-style versioning to OpenAPI paths** when:
- You have experimental APIs that may change
- Enterprise customers need stability guarantees
- You want to evolve APIs independently of application releases

**Implementation:**
```yaml
# openapi/accounting/general-ledger/openapi.yaml
paths:
  /api/v1alpha1/accounts:  # Experimental API
    get:
      ...
  /api/v1/ledger-entries:  # Stable API
    get:
      ...
```

**Migration Path:**
1. Start with all APIs at `v1alpha1` (experimental)
2. Promote to `v1beta1` when API is more stable
3. Promote to `v1` when API is stable (no breaking changes)
4. Support multiple versions during transition

---

## Decision Matrix

| Aspect | Semantic Versioning | Kubernetes-Style | Hybrid |
|--------|-------------------|------------------|--------|
| **Application Releases** | ✅ Excellent | ❌ Not standard | ✅ Excellent |
| **API Stability Communication** | ❌ Poor | ✅ Excellent | ✅ Excellent |
| **Enterprise Customer Clarity** | ⚠️ Moderate | ✅ Excellent | ✅ Excellent |
| **Complexity** | ✅ Low | ⚠️ Moderate | ⚠️ Moderate |
| **Industry Standard** | ✅ Yes (apps) | ✅ Yes (APIs) | ✅ Yes (both) |
| **Dependency Management** | ✅ Excellent | ❌ Not applicable | ✅ Excellent |
| **Container Images** | ✅ Standard | ❌ Not applicable | ✅ Standard |

---

## Implementation Considerations

### If You Choose Pure Semantic Versioning (Current)

**Pros:**
- ✅ No changes needed
- ✅ Simple and familiar
- ✅ Works for most use cases

**Cons:**
- ❌ Can't communicate API stability
- ❌ Breaking API changes require major version bump
- ❌ Enterprise customers may be hesitant without stability guarantees

### If You Choose Hybrid Approach

**Changes Needed:**
1. Update OpenAPI specs to include version in paths:
   ```yaml
   paths:
     /api/v1alpha1/accounts:  # Instead of /api/accounts
   ```

2. Update BFF generation to preserve API versions

3. Document API versioning policy:
   - `v1alpha1`: Experimental, breaking changes allowed
   - `v1beta1`: More stable, breaking changes possible
   - `v1`: Stable, no breaking changes

4. Support multiple API versions during transitions

**Benefits:**
- ✅ Clear API stability communication
- ✅ Can evolve APIs independently
- ✅ Enterprise customers get stability guarantees
- ✅ Follows industry best practices

---

## Final Recommendation

### For RERP: **Hybrid Approach**

1. **Keep Semantic Versioning** for application releases (containers, binaries, charts)
   - Current approach is correct
   - Standard for enterprise applications
   - No changes needed

2. **Add Kubernetes-Style API Versioning** to OpenAPI specifications
   - Start with `v1alpha1` for all APIs (experimental)
   - Document promotion path: `v1alpha1` → `v1beta1` → `v1`
   - Update OpenAPI paths to include version
   - Update BFF generation to preserve versions

3. **Communication Strategy**
   - Application version: `RERP v0.39.0`
   - API version: `/api/v1alpha1/accounts` (experimental)
   - Clear documentation on what each means

### Why This Works for Enterprise Applications

- **Application versioning** (semantic) tells customers about the product release
- **API versioning** (Kubernetes-style) tells customers about API stability
- **Both are needed** for enterprise customers who integrate with your APIs
- **Industry standard** approach (Kubernetes, Istio, etc.)

---

## Next Steps

1. **Short Term**: Keep semantic versioning for application releases (no changes)
2. **Medium Term**: Evaluate adding API versioning to OpenAPI specs
3. **Long Term**: Implement hybrid approach if enterprise customers need API stability guarantees

---

## References

- [Semantic Versioning 2.0.0](https://semver.org/)
- [Kubernetes API Versioning](https://kubernetes.io/docs/reference/using-api/api-concepts/#api-versioning)
- [Istio API Versioning](https://istio.io/latest/docs/reference/config/)
