# DOCUMENTS_ANALYSIS — PRD Index

## Overview

This directory contains the product requirement documents (PRDs) that address the architecture audit findings for the DOCUMENTS_ANALYSIS project. Each PRD maps to specific audit recommendations and must be implemented in order to bring the design to production quality.

## Audit Coverage Matrix

| Audit Finding | PRD | Status |
|--------------|-----|--------|
| **P0 — Critical Issues** | | |
| Cross-component entity duplication | PRD-001 | ✅ Draft |
| Missing core types (User, Team, Tenant) | PRD-001 | ✅ Draft |
| No event bus / integration architecture | PRD-002 | ✅ Draft |
| No document lifecycle / data flow | PRD-005 | ✅ Draft |
| **P1 — Design Issues** | | |
| StorageBackendEntity naming conflict | PRD-001 + PRD-006 | ✅ Draft |
| JSONB abuse (ClassificationRule.condition) | PRD-006 | ✅ Draft |
| Denormalized M2M (RetentionPolicy.document_types) | PRD-006 | ✅ Draft |
| OCR enum bloat (element_type) | PRD-006 | ✅ Draft |
| Inconsistent URL patterns | PRD-003 | ✅ Draft |
| Missing pagination | PRD-003 | ✅ Draft |
| Missing idempotency | PRD-003 | ✅ Draft |
| Missing status code documentation | PRD-003 | ✅ Draft |
| No enum consolidation | PRD-006 | ✅ Draft |
| No field naming standardization | PRD-006 | ✅ Draft |
| **P2 — Content Issues** | | |
| Missing competitors (ABBYY, Google, UiPath) | PRD-004 | ✅ Draft |
| Mischaracterized DocuPipe OCR capability | PRD-004 | ✅ Draft |
| Mischaracterized Paperless-ngx certifications | PRD-004 | ✅ Draft |
| Overclaimed performance metrics | PRD-004 | ✅ Draft |
| Duplicate positioning claims | PRD-004 | ✅ Draft |
| **P3 — Operational Issues** | | |
| No health check endpoints | PRD-007 | ✅ Draft |
| No database migration strategy | PRD-007 | ✅ Draft |
| No data seeding strategy | PRD-007 | ✅ Draft |
| No graceful degradation | PRD-007 | ✅ Draft |
| No monitoring/observability | PRD-007 | ✅ Draft |
| No schema evolution strategy | PRD-007 | ✅ Draft |
| No governed document output or rendition boundary | PRD-008 | ✅ Draft |

## PRD List (Implementation Order)

### Phase 1: Foundation (P0 — Must do first)

#### PRD-001: Canonical Entity Registry
**What:** Single authoritative source for all 30 entities across the system
**Why:** Components redefine entities with conflicting fields — no single source of truth
**Impact:** Every other PRD depends on this — entity definitions must be stable before API, events, or data flow can be designed
**Changes:**
- Creates `ENTITY_REGISTRY.md` in DOCUMENTS_ANALYSIS root
- Defines 4 new foundational entities: `User`, `Team`, `TeamMember`, `Tenant`
- Renames `StorageBackendEntity` → `StorageBackendConfig` (config vs data distinction)
- Moves `DocumentVersion` and `DataRedaction` from component READMEs to canonical registry
- Adds `ProcessingJob` entity (renamed from `Processing Queue`)
- Full inventory of all 30 entities with complete field lists

#### PRD-002: Event Bus Architecture
**What:** PostgreSQL LISTEN/NOTIFY based event system connecting all 10 components
**Why:** Components are silos — documents flow via pull-based polling, not event-driven
**Impact:** Enables the processing pipeline: ingestion → OCR → classification → extraction → storage
**Changes:**
- Defines 11 event types with common envelope schema
- Updates `ProcessingJob` stage enum from pull-based to event-driven
- Defines `EventLog` table for durability + dead-letter queue
- Documents component event subscriptions and publications
- Adds `/events` API endpoints

#### PRD-005: Cross-Component Data Flow
**What:** Document lifecycle diagram, data propagation matrix, graceful degradation
**Why:** Components designed in isolation with no cross-component integration
**Impact:** Ensures documents actually flow through the pipeline instead of getting stuck
**Changes:**
- Defines document lifecycle: QUEUED → OCR → CLASSIFY → EXTRACT → STORAGE → COMPLETED
- Defines 8 data propagation rules (when component X completes, what data goes where)
- Defines graceful degradation for all 5 failure modes
- Defines `DocumentEventLog` entity for pipeline audit trail
- Defines component dependency graph for phased implementation

### Phase 2: API & Schema (P1 — Required before implementation)

#### PRD-003: API Standardization
**What:** Unified API contract for all 10 components
**Why:** Inconsistent URL patterns, no pagination, no idempotency, no status codes
**Impact:** Required for OpenAPI spec generation and SDK generation
**Changes:**
- Standardizes to resource-oriented URLs (`/documents` not `/extract`)
- Adds pagination to all list endpoints
- Adds idempotency key support to all POST endpoints
- Defines standard response formats (success/error)
- Defines API versioning (`/api/v1/` prefix)
- Standardizes permission scopes and error codes
- Creates `IdempotencyKey` entity in canonical registry

#### PRD-006: Entity Field Standardization
**What:** Split OCR enums, structured classification rules, proper M2M tables, central enum registry
**Why:** JSONB abuse, enum bloat, denormalized M2M, inconsistent field naming
**Impact:** Enables indexed queries, type-safe OpenAPI schemas, proper FK relationships
**Changes:**
- Splits `OcrTextElement.element_type` into `text_unit_type` + `semantic_type`
- Replaces JSONB `ClassificationRule.condition` with structured fields
- Replaces `RetentionPolicy.document_types UUID[]` with junction table
- Creates Enum Registry with 22 centralized enums
- Defines field naming conventions

### Phase 3: Operational Readiness (P1 — Required before production)

#### PRD-007: Operational Concerns & Migration Strategy
**What:** Health checks, migrations, seeding, degradation, monitoring, schema evolution
**Why:** No operational concerns addressed in any component design
**Impact:** Ensures the system is deployable, observable, and maintainable
**Changes:**
- Defines `/health` and `/ready` endpoints for all components
- Defines migration directory structure, naming convention, schema versioning
- Defines seed scripts for initial install
- Defines graceful degradation for 5 failure modes
- Defines structured logging format and `/metrics` endpoint
- Defines request ID propagation across components
- Defines schema evolution strategy with backward compatibility rules

### Phase 4: Content Enrichment (P2 — Improves product positioning)

#### PRD-004: Competitive Intelligence Enrichment
**What:** Add missing competitors, correct mischaracterizations, remove unverifiable claims
**Why:** 5 major competitors missing, several mischaracterizations, overclaimed performance
**Impact:** Strengthens market positioning for sales and launch
**Changes:**
- Adds ABBYY Vantage, Google Document AI, UiPath Document Understanding
- Corrects DocuPipe OCR capability characterization
- Reframes Paperless-ngx "no certifications" as irrelevant
- Replaces 7 unverifiable performance claims with accurate Rust benefit statements
- Rewrites all 10 "Where RERP Wins" sections with component-specific positioning

### Phase 5: Document Output (P0 for invoice delivery)

#### PRD-008: Document Generation and Rendition
**What:** A suite-owned API for external HTML/CSS templates, immutable generated renditions, explicit copies, and future trust services
**Why:** Rich document production is a cross-suite Documents capability; placing it in Accounting would create a migration and duplicate the platform boundary
**Impact:** Gives Accounting and every later RERP suite one governed output path while preserving source-suite ownership of business facts
**Changes:**
- Adds the `documents/render` product and API component
- Defines post-commit, idempotent handoff of frozen typed render models
- Defines published template versions and immutable source/template/snapshot lineage
- Defines original-versus-copy semantics and post-MVP PAdES sealing/timestamping
- Prohibits tenant selectors in requests; tenant and actor context come from Sesame

## Implementation Sequence

```
Week 1-2: PRD-001 (Canonical Entity Registry)
  → All 30 entities defined, no duplicates, no ambiguities
  
Week 2-3: PRD-002 (Event Bus) + PRD-005 (Data Flow)
  → Events defined, lifecycle diagram, degradation strategies
  
Week 3-4: PRD-003 (API Standardization) + PRD-006 (Field Standardization)
  → URLs, pagination, idempotency, enums, naming
  
Week 4-5: PRD-007 (Operational Concerns)
  → Health checks, migrations, monitoring, seeding
  
Week 5-6: PRD-004 (Content Enrichment)
  → Competitive landscape updated, claims corrected
```

## Deliverables

Each PRD produces concrete changes to the DOCUMENTS_ANALYSIS:

| PRD | Output Files |
|-----|-------------|
| PRD-001 | `ENTITY_REGISTRY.md` (new file in DOCUMENTS_ANALYSIS root) |
| PRD-002 | Updated `ProcessingJob` entity, `event_log` table spec, component README updates |
| PRD-003 | Updated API endpoints in all component READMEs |
| PRD-004 | Updated `README.md` (competitive landscape) + component README updates |
| PRD-005 | Document lifecycle diagram, `DocumentEventLog` entity, component README updates |
| PRD-006 | Updated entity definitions in `ENTITY_REGISTRY.md`, enum registry |
| PRD-007 | Operational standards document, migration templates, seed scripts |
| PRD-008 | `render/openapi.yaml`, Documents render scaffold, ADR 002, and this generation/rendition PRD |

## How to Use This Index

1. Read PRDs in Phase order (1 → 4)
2. Each PRD is self-contained — implement it fully before moving to the next
3. Track implementation status in the PRD files themselves
4. When a PRD is complete, mark it with `**Status:** Implemented` and check off the acceptance criteria
