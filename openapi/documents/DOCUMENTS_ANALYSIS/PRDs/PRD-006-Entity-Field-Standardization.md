# PRD-006: Entity Field Standardization & Enum Consolidation

## Meta

- **Status:** Draft
- **Author:** Engineering Design
- **Created:** 2026-05-11
- **Related:** PRD-001 (Canonical Entity Registry)
- **Priority:** P1 — Required for clean OpenAPI generation
- **Blocks:** Schema validation, data type safety

## Problem

The audit identified several field-level issues:

1. **Enum bloat in `OcrTextElement.element_type`:** Mixing hierarchy levels (WORD, LINE, PARAGRAPH) with semantic types (TABLE, FORM, SIGNATURE). These are orthogonal concerns that should be separate fields.

2. **JSONB abuse in `ClassificationRule.condition`:** Conditions are known at design time ("content_type contains invoice", "filename matches pattern X") but stored as opaque JSON. This means:
   - No indexing on condition fields
   - No query for "find all rules targeting invoices"
   - No type safety
   - Rules engine is a JSON parser, not a logic engine

3. **Denormalized M2M in `RetentionPolicy.document_types`:** `UUID[]` array of document type IDs means:
   - No referential integrity
   - Cannot query "which retention policies apply to this document?" without scanning all policies
   - Hard to audit which policies are affected when a document type is deleted

4. **Inconsistent naming:** `storage_backend` in StorageBackendConfig vs `storage_backend` in DocumentStorage. Same concept, same name, different contexts. Fine, but must be consistent with `storage_path` (a string) vs `bucket_or_path` (also a string) — different contexts, same semantic meaning.

## Solution

### 1. Split OCR Text Element Types

Current (single enum):
```
element_type: Enum: [WORD, LINE, PARAGRAPH, TABLE, FORM, SIGNATURE]
```

New (two fields):
```
text_unit_type: Enum: [WORD, LINE, PARAGRAPH, BLOCK]
    ← Hierarchy level of this text element

semantic_type: Enum: [BODY, TITLE, HEADER, FOOTER, TABLE, FORM_FIELD, SIGNATURE, IMAGE_CAPTION]
    ← Semantic meaning of this text element

is_table_cell: Boolean  ← if semantic_type = TABLE, marks if this is a cell vs. table header
```

Examples:
| Old `element_type` | New `text_unit_type` | New `semantic_type` |
|--------------------|---------------------|---------------------|
| WORD | WORD | BODY |
| LINE | LINE | BODY |
| PARAGRAPH | PARAGRAPH | BODY |
| TABLE (header) | PARAGRAPH | HEADER |
| TABLE (cell) | LINE | TABLE (with is_table_cell=true) |
| FORM (field label) | LINE | FORM_FIELD |
| FORM (field value) | LINE | BODY |
| SIGNATURE | PARAGRAPH | SIGNATURE |

### 2. Structured Classification Rules

Replace JSONB condition with structured fields:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (255) | Yes | Rule name |
| `condition_field` | Enum: [FILENAME_PATTERN, CONTENT_TYPE, SOURCE, METADATA_KEY, EXTRACTED_FIELD] | Yes | Which field to match |
| `condition_value` | String (1000) | Yes | Pattern/value to match (glob/wildcard for patterns, exact for enum) |
| `condition_operator` | Enum: [CONTAINS, STARTS_WITH, ENDS_WITH, EQUALS, MATCHES_REGEX, EXISTS] | Yes | How to match |
| `target_type_id` | Foreign Key: DocumentType | Yes | Target document type |
| `priority` | Integer | Yes | Rule priority (lower = higher priority) |
| `is_active` | Boolean | No | Rule activation (default: true) |
| `created_at` | DateTime | Yes | Creation timestamp |

Example rules:
- Filename matches `*.pdf` AND content_type = `application/pdf` → classify as PDF document
- Filename contains `invoice` → classify as Invoice
- Metadata key `document_type` exists AND equals `receipt` → classify as Receipt

This enables indexed queries: `SELECT * FROM classification_rule WHERE condition_field = 'FILENAME_PATTERN' AND condition_value LIKE '%invoice%'`

### 3. Proper M2M for Retention Policies

Replace `RetentionPolicy.document_types: UUID[]` with junction table:

**DocumentTypeRetentionPolicy Entity (Junction):**

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `retention_policy_id` | Foreign Key: RetentionPolicy | Yes | Policy |
| `document_type_id` | Foreign Key: DocumentType | Yes | Document type |
| `created_at` | DateTime | Yes | Creation timestamp |

**RetentionPolicy:** Remove `document_types UUID[]` field.

Query: "Which retention policies apply to invoices?"
```sql
SELECT rp.* FROM retention_policy rp
JOIN document_type_retention_policy drp ON drp.retention_policy_id = rp.id
JOIN document_type dt ON dt.id = drp.document_type_id
WHERE dt.name = 'Invoice';
```

### 4. Enum Consolidation

Create a central Enum Registry in the canonical entity registry. All components reference these enums; none redefine them.

**Enum Registry:**

| Enum Name | Values | Used By |
|-----------|--------|---------|
| `DocumentSource` | UPLOAD, API, EMAIL, SCAN, WEBHOOK, IMPORT | Document entity |
| `DocumentStatus` | QUEUED, PROCESSING, COMPLETED, FAILED | Document entity |
| `ProcessingStage` | QUEUED, OCR_IN_PROGRESS, CLASSIFY_IN_PROGRESS, EXTRACT_IN_PROGRESS, STORAGE_IN_PROGRESS, COMPLETED, FAILED | ProcessingJob entity |
| `StorageBackendType` | LOCAL, S3, GCS, AZURE_BLOB | StorageBackendConfig, DocumentStorage |
| `OcrTextUnitType` | WORD, LINE, PARAGRAPH, BLOCK | OcrTextElement |
| `OcrSemanticType` | BODY, TITLE, HEADER, FOOTER, TABLE, FORM_FIELD, SIGNATURE, IMAGE_CAPTION | OcrTextElement |
| `LayoutElementType` | PARAGRAPH, TITLE, HEADER, FOOTER, LIST, TABLE, IMAGE, COLUMN, SPLASH_PAGE | Layout entity |
| `ExtractionFieldEnum` | STRING, INTEGER, DECIMAL, DATE, CURRENCY, BOOLEAN, EMAIL, PHONE | ExtractionField entity |
| `ValidationStatus` | PENDING, ACCEPTED, REJECTED | ExtractionResult entity |
| `WorkflowStatus` | PENDING, RUNNING, COMPLETED, FAILED, CANCELLED | WorkflowExecution entity |
| `WorkflowStepStatus` | PENDING, RUNNING, COMPLETED, FAILED, SKIPPED | WorkflowExecutionStep entity |
| `ApprovalStatus` | PENDING, APPROVED, REJECTED | Approval entity |
| `WebhookEventStatus` | PENDING, SENT, FAILED, RETRYING | WebhookEvent entity |
| `AuditSeverity` | INFO, WARNING, ERROR, CRITICAL | AuditLog entity |
| `Permission` | READ, WRITE, ADMIN, DELETE | AccessControl entity |
| `ApiKeyRole` | ADMIN, EDITOR, VIEWER, API_ONLY | User entity |
| `TeamMemberRole` | OWNER, EDITOR, VIEWER | TeamMember entity |
| `TenantStatus` | ACTIVE, SUSPENDED, TRIAL | Tenant entity |
| `TenantPlan` | FREE, STARTER, BUSINESS, ENTERPRISE | Tenant entity |
| `ComplianceFrameworkStatus` | PENDING, IN_PROGRESS, COMPLIANT, NON_COMPLIANT | ComplianceFramework entity |
| `RedactionMethod` | BLACKOUT, REPLACE, HASH, TOKENIZE | DataRedaction entity |
| `RetentionAction` | DELETE, ARCHIVE, NOTIFY | RetentionPolicy entity |
| `DashboardWidgetType` | COUNTER, LINE_CHART, BAR_CHART, TABLE, GAUGE | DashboardWidget entity |
| `ReportFormat` | PDF, CSV, HTML | Report entity |

### 5. Field Naming Conventions

| Convention | Rule | Example |
|------------|------|---------|
| Primary key | Always `id` UUID | `id UUID PRIMARY KEY` |
| Foreign keys | `{table_name}_id` | `document_id UUID REFERENCES document(id)` |
| Timestamps | `created_at` (always), `updated_at` (mutable entities) | `created_at TIMESTAMP DEFAULT NOW()` |
| Soft deletes | `is_active` boolean, default true | `is_active BOOLEAN DEFAULT true` |
| Nullable fields | Explicitly `NULL` with default `NULL` | `metadata JSONB NULL DEFAULT NULL` |
| Boolean fields | Prefix with `is_` or `has_` | `is_active`, `has_errors` |
| Enum fields | Lowercase with underscore | `status`, `source` |
| Timestamp columns | Use `TIMESTAMP` not `DATETIME` | `created_at TIMESTAMP` |

## Acceptance Criteria

- [ ] `OcrTextElement` split into `text_unit_type` + `semantic_type` fields
- [ ] `ClassificationRule.condition` (JSONB) replaced with structured `condition_field`, `condition_value`, `condition_operator` fields
- [ ] `RetentionPolicy.document_types` (UUID[]) replaced with `DocumentTypeRetentionPolicy` junction table
- [ ] Enum Registry created in canonical entity registry with all 22 enums listed above
- [ ] All components reference canonical enums; no local enum redefinitions
- [ ] Field naming conventions documented and applied to all 30 entities
- [ ] All FK naming follows `{table_name}_id` convention
