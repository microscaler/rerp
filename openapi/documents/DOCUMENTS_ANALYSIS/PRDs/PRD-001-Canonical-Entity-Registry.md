# PRD-001: Canonical Entity Registry

## Meta

- **Status:** Draft
- **Author:** Engineering Design
- **Created:** 2026-05-11
- **Related:** document-ingestion, ocr-extraction, data-extraction, classification, storage-management, workflow-automation, search-discovery, integration-api, security-compliance, reporting-analytics
- **Priority:** P0 — Blocks all implementation

## Problem

Entity definitions are duplicated across components, creating conflicts:

- `StorageBackendEntity` (ingestion) vs `DocumentStorage` (storage-management) — different concepts, unclear naming
- `Document Version` referenced in ingestion but defined in storage-management
- `Data Redaction` orphaned in security-compliance but logically belongs in OCR/extraction pipeline
- `User`, `Team`, `Tenant` entities referenced everywhere but never defined
- `Processing Queue` bridges components but has no clear event-driven contract

No single source of truth for entity definitions means:
- Entity A redefined with different fields in Component B
- Foreign key relationships are ambiguous
- OpenAPI spec generation produces conflicting schemas
- Database migrations will break

## Solution

Create a **Canonical Entity Registry** (`ENTITY_REGISTRY.md`) that is the single authoritative source for every entity in the Documents suite. Components reference it by name; they never redefine entities. This registry will feed directly into OpenAPI spec generation.

## Scope

### Entities to Define (Foundational)

These are the "infrastructure" entities every component depends on. Defined once, referenced everywhere.

#### Tenant Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (255) | Yes | Tenant/organization name |
| `slug` | String (64) | Yes | URL-safe identifier (unique per tenant) |
| `status` | Enum: [ACTIVE, SUSPENDED, TRIAL] | Yes | Tenant lifecycle state |
| `region` | String (64) | No | Data residency region |
| `max_documents` | Integer | No | Document limit (0 = unlimited) |
| `max_storage_gb` | Integer | No | Storage limit in GB (0 = unlimited) |
| `plan_type` | Enum: [FREE, STARTER, BUSINESS, ENTERPRISE] | Yes | Billing plan |
| `settings` | JSONB | No | Tenant-specific configuration |
| `created_at` | DateTime | Yes | Creation timestamp |
| `updated_at` | DateTime | Yes | Last update |

#### User Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `tenant_id` | Foreign Key: Tenant | Yes | Owner tenant |
| `email` | String (255) | Yes | Login email (unique per tenant) |
| `password_hash` | String (255) | Yes | BCrypt/Argon2 hash |
| `full_name` | String (255) | No | Display name |
| `role` | Enum: [ADMIN, EDITOR, VIEWER, API_ONLY] | Yes | Access level |
| `is_active` | Boolean | Yes | Account status (default: true) |
| `mfa_enabled` | Boolean | No | MFA status (default: false) |
| `last_login_at` | DateTime | No | Last login timestamp |
| `created_at` | DateTime | Yes | Creation timestamp |
| `updated_at` | DateTime | Yes | Last update |

#### Team Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `tenant_id` | Foreign Key: Tenant | Yes | Owner tenant |
| `name` | String (255) | Yes | Team name |
| `description` | Text | No | Team description |
| `created_at` | DateTime | Yes | Creation timestamp |
| `updated_at` | DateTime | Yes | Last update |

#### Team Member Entity (Junction: Team ↔ User)

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `team_id` | Foreign Key: Team | Yes | Team |
| `user_id` | Foreign Key: User | Yes | User |
| `role_in_team` | Enum: [OWNER, EDITOR, VIEWER] | Yes | Role within this team |
| `joined_at` | DateTime | Yes | Join timestamp |

### Entity Merges and Renames

Current entity | Location | Issue | Resolution
---|---|---|---
`StorageBackendEntity` | document-ingestion/ | Config entity, not a data entity | **Rename to `StorageBackendConfig`**
`DocumentStorage` | storage-management/ | Data entity, same name | Keep name (now unambiguous)
`Processing Queue` | document-ingestion/ | Bridges components | **Rename to `ProcessingJob`**, move to canonical registry
`Document Version` | storage-management/ | Referenced by ingestion | Move to canonical registry
`Data Redaction` | security-compliance/ | Orphaned from pipeline | Move to canonical registry, note it's invoked by OCR/extraction pipeline

### All Entities in the System (Full Inventory)

Below is the complete entity inventory organized by subsystem. Every entity defined here is authoritative. No other document may redefine these entities.

#### Foundational (Canonical)

| Entity | Purpose | Referenced By |
|--------|---------|---------------|
| `Tenant` | Multi-tenancy | All components |
| `User` | Authentication/authorization | All components |
| `Team` | Grouping users | storage-management, security-compliance |
| `TeamMember` | User-Team membership | storage-management, security-compliance |
| `StorageBackendConfig` | Available storage backends | storage-management |
| `DocumentStorage` | Where each document is stored | All components |
| `DocumentVersion` | Version tracking | storage-management, document-ingestion |
| `DataRedaction` | PII/PHI redaction records | ocr-extraction, security-compliance |
| `ProcessingJob` | Document processing queue | All components |

#### Document Ingestion

| Entity | Purpose |
|--------|---------|
| `Document` | Central document entity (id, filename, content_type, file_size, checksum_sha256, source, status, pages, metadata, tags, remote_id, created_at, created_by, updated_at, error_message) |
| `EmailIngestion` | Email attachment processing |

#### OCR & Text Extraction

| Entity | Purpose |
|--------|---------|
| `OcrResult` | Per-page OCR result (id, document_id, page_number, text_content, language, confidence, model_version, created_at, duration_ms) |
| `OcrTextElement` | Per-element OCR data (id, ocr_result_id, text, confidence, text_unit_type, semantic_type, bounding_box, rotation, page_order, parent_id) |
| `Layout` | Document layout structure (id, document_id, page_number, element_type, bounding_box, page_order, text, confidence) |

#### Data Extraction & Standardization

| Entity | Purpose |
|--------|---------|
| `ExtractionSchema` | Extraction schemas (id, name, version, schema[JSONB], document_type, confidence_threshold, is_active, natural_language_prompt, created_at, updated_at) |
| `ExtractionResult` | Per-document extraction result (id, document_id, schema_id, extracted_data[JSONB], confidence_scores[JSONB], validation_status, reviewed_by, reviewed_at, review_notes) |
| `ExtractionField` | Per-field extraction data (id, extraction_result_id, field_name, field_value, field_type, confidence, page_number, bounding_box, normalized_value) |
| `DataStandardization` | Field normalization records (id, extraction_result_id, field_name, original_value, standardized_value, standardization_rule, applied_at) |

#### Classification

| Entity | Purpose |
|--------|---------|
| `DocumentType` | Document type taxonomy (id, name, parent_id, description, icon, color, extraction_schema_id, is_active, created_at) |
| `DocumentClassification` | Classification results (id, document_id, document_type_id, confidence, method, is_confirmed, confirmed_by, confirmed_at, created_at) |
| `ClassificationRule` | Rule-based classification (id, name, condition[structured fields], target_type_id, priority, is_active, created_at) |

#### Storage & Management

| Entity | Purpose |
|--------|---------|
| `RetentionPolicy` | Data retention rules (id, name, document_type_ids[UUID[]], retention_period_days, action_on_expire, is_active, created_at, last_applied_at) |
| `AccessControl` | Per-resource permissions (id, resource_type, resource_id, user_id, team_id, permission, granted_by, expires_at, created_at) |

#### Workflow Automation

| Entity | Purpose |
|--------|---------|
| `Workflow` | Workflow definition (id, name, definition[JSONB], is_active, document_types[], trigger_condition[JSONB], created_at, created_by, updated_at) |
| `WorkflowExecution` | Workflow run (id, workflow_id, document_id, status, current_node, started_at, completed_at, duration_ms, error_message) |
| `WorkflowExecutionStep` | Individual workflow stage (id, execution_id, node_name, status, started_at, completed_at, duration_ms, output[JSONB]) |
| `Approval` | Human approval (id, execution_id, approver_id, status, comments, created_at, completed_at, delegated_to) |

#### Search & Discovery

| Entity | Purpose |
|--------|---------|
| `SearchIndex` | Searchable document entry (id, document_id, content_hash, indexed_at, searchable_content, metadata_json, extracted_fields_json, vector_embedding) |
| `SearchQuery` | Query history (id, user_id, query_text, filters[JSONB], results_count, duration_ms, created_at) |
| `SavedSearch` | Bookmark queries (id, user_id, name, query[JSONB], is_public, created_at, last_run_at) |

#### Integration & API

| Entity | Purpose |
|--------|---------|
| `ApiKey` | API credential (id, name, key_prefix, key_hash, permissions[], rate_limit, is_active, created_at, last_used_at) |
| `WebhookSubscription` | Webhook registration (id, url, events[], secret, is_active, created_at, last_triggered_at) |
| `WebhookEvent` | Webhook delivery tracking (id, event_type, payload[JSONB], signature, status, attempts, next_retry_at, created_at, delivered_at) |
| `IntegrationEndpoint` | Downstream integration config (id, name, url, auth_type, payload_template[JSONB], is_active, created_at) |

#### Security & Compliance

| Entity | Purpose |
|--------|---------|
| `AuditLog` | Security audit trail (id, user_id, action, resource_type, resource_id, ip_address, user_agent, metadata[JSONB], severity, created_at) |
| `ComplianceFramework` | Certification tracking (id, framework, status, last_audit, next_audit, evidence[JSONB], created_at, updated_at) |
| `DataRedaction` | Redaction records (id, document_id, pattern_type, pattern_regex, occurrences, redacted_at, redaction_method) |

#### Reporting & Analytics

| Entity | Purpose |
|--------|---------|
| `Metric` | Time-series metric (id, metric_name, metric_value, dimension_type, dimension_id, timestamp, source, tags[]) |
| `Report` | Scheduled report (id, name, template[JSONB], schedule, recipients[], format, is_active, created_at, last_generated_at) |
| `Dashboard` | Dashboard config (id, name, widgets[JSONB], owner_id, is_default, is_public, created_at) |
| `DashboardWidget` | Individual widget (id, dashboard_id, name, widget_type, metric, config[JSONB], position_x, position_y, width) |

## OpenAPI Impact

The canonical entity registry becomes the source for all OpenAPI spec generation:
- Each entity maps to a `components/schemas/` entry
- Components reference entities by `$ref` (e.g., `"$ref": "#/components/schemas/Document"`)
- FK relationships become `x-foreign-key` metadata
- Enums are defined in `components/schemas/Enums` and referenced everywhere

## Entity Field Standardization

All entities MUST follow these conventions:

- Primary keys: `id` UUID, always present
- Timestamps: `created_at` (always), `updated_at` (for mutable entities)
- Soft deletes: `is_active` boolean, NOT hard delete
- Nullable fields: explicitly marked, never omitted
- Enum fields: defined in canonical list, never redefined locally
- JSONB fields: document the schema structure in comments

## Migration from Current Design

### Breaking changes:

1. Rename `StorageBackendEntity` → `StorageBackendConfig` (in document-ingestion README)
2. Add `User`, `Team`, `TeamMember`, `Tenant` entities (new)
3. Move `DocumentVersion` from storage-management to canonical registry
4. Move `DataRedaction` from security-compliance to canonical registry
5. Rename `Processing Queue` → `ProcessingJob` in document-ingestion
6. Split `element_type` enum in `OcrTextElement` into `text_unit_type` and `semantic_type`

### Non-breaking changes:

1. Add `tenant_id` FK to `User`, `Team`, `Tenant`
2. Add `user_id` FK to `TeamMember`
3. All existing component READMEs now reference the canonical registry instead of redefining entities

## Acceptance Criteria

- [ ] `ENTITY_REGISTRY.md` exists in DOCUMENTS_ANALYSIS root with all 30 entities
- [ ] No entity is defined in more than one location
- [ ] Every component README references entities from the registry, not redefines them
- [ ] `User`, `Team`, `TeamMember`, `Tenant` are defined
- [ ] All FK relationships are explicit (no implicit references)
- [ ] Enum values are centralized (not redefined in each component)
- [ ] Field naming conventions are standardized across all entities
- [ ] The registry document is referenced from the main README.md
