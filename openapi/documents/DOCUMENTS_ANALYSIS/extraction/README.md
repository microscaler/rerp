# Data Extraction

> **Component:** Structured data extraction — pull key-value pairs, table data, and form fields from classified documents
> **Priority:** P1 — Extract structured data from classified docs; the final step before data becomes usable
> **Reference Competitors:** DocuPipe (LLM-native, $0.001/request), Google Document AI custom extractor, Azure prebuilt/invoice models, ABBYY extraction, Nanonets ($0.30/run), Rossum extraction

---

## The Pitch

**Buyer Question:** *Can I extract invoice numbers, dates, line items, totals, vendor names, and other structured data from any document type — without writing extraction templates, without training custom models, and without sending my documents to a third-party API?*

Extraction is where documents become data. The difference between a scanned invoice and a structured record in your accounts payable system is extraction. If extraction requires template creation for every document variant, you're in maintenance hell. If it requires training custom models for every new vendor, you're burning engineering time. If it requires sending your documents to Google or Azure every time, you're paying per extraction and risking data exposure. RERP's extraction engine solves all three problems: schema-based extraction for known structures, LLM-native extraction for everything else, and self-hosted processing that keeps your data in your environment.

---

## What This Component Does

Data Extraction pulls structured fields from classified documents. It is the third processing step after OCR (for scanned docs) or ingestion (for native-text docs):

1. **Schema-Based Extraction** — Define extraction schemas with typed fields (string, number, date, currency, email, phone, address) and extraction rules. Schemas can be shared across document types or customized per document. Schema-driven extraction provides deterministic, auditable results.
2. **LLM-Based Extraction (No Templates Needed)** — Describe what fields you need in natural language, and the LLM extracts them. "Extract vendor name, invoice number, date, line items with quantities/prices, and total amount" — done. No template creation, no field mapping, no layout analysis. This is RERP's key differentiator.
3. **Table Extraction** — Extract table data from classified documents with row/column structure preserved. Each cell is typed and validated. Table extraction works on OCR-detected tables and native PDF tables. Supports nested tables, merged cells, and multi-row headers.
4. **Form Field Extraction** — Detect and extract form field values (label → value pairs). Handles checkboxes, radio buttons, dropdown selections, and free-text form fields. Supports PDF form fields, paper forms (post-OCR), and digital fillable forms.
5. **Multi-Page Extraction** — Extract data that spans multiple pages (e.g., line items on page 1, totals on page 5). Context-aware extraction maintains document structure across page boundaries.
6. **Confidence Scoring** — Each extracted field has a confidence score (0-100%). Fields below confidence threshold (default 70%) are flagged for human review. Overall document extraction score is computed from individual field scores.
7. **Human-in-the-Loop Verification** — Low-confidence fields are presented to a human reviewer with the source document highlighted. Reviewer can accept, reject, or correct values. Corrections are logged and fed back to improve extraction accuracy over time.
8. **Extraction Validation Against Business Rules** — Extracted values can be validated against business rules: "invoice total must equal sum of line items," "vendor must exist in vendor master," "invoice date must be within 90 days." Validation failures flag documents for review.

---

## Entity Model

### ExtractionSchema

Defines the structure for extraction — what fields to extract and their types.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (255) | Yes | Schema name (e.g., "Supplier Invoice Schema") |
| `description` | Text | No | Schema description and usage notes |
| `document_class_id` | UUID | No | Associated document class (nullable = general purpose) |
| `is_active` | Boolean | No | Soft toggle |
| `version` | Int32 | No | Schema version (incremented on change) |
| `fields` | JSONB | Yes | Array of field definitions (see below) |
| `validation_rules` | JSONB | No | Business validation rules applied after extraction |
| `created_at` | DateTime | Yes | Auto-set on creation |
| `updated_at` | DateTime | Yes | Auto-updated on change |

**Field Definition Format:**
```json
{
  "name": "invoice_number",
  "type": "string",
  "required": true,
  "confidence_threshold": 75,
  "description": "Invoice number from header"
}
```

### ExtractionTemplate

Pre-defined extraction configurations for specific document types.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (255) | Yes | Template name |
| `document_class_id` | UUID | Yes | Associated document class |
| `extraction_schema_id` | UUID | No | Linked schema (if using schema-based extraction) |
| `extraction_mode` | Enum: [SCHEMA, LLM_NATURAL_LANGUAGE, HYBRID] | Yes | How extraction is performed |
| `llm_prompt_template` | Text | No | Natural language prompt for LLM extraction |
| `is_active` | Boolean | No | Soft toggle |
| `created_at` | DateTime | Yes | Auto-set on creation |
| `updated_at` | DateTime | Yes | Auto-updated on change |

### ExtractionJob

Represents a single extraction processing request.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `ingest_job_id` | UUID | Yes | Source document ingestion job |
| `classification_id` | UUID | Yes | Associated classification result |
| `extraction_template_id` | UUID | No | Template used for extraction |
| `extraction_mode` | Enum: [SCHEMA, LLM_NATURAL_LANGUAGE, HYBRID] | Yes | Extraction method used |
| `status` | Enum: [QUEUED, PROCESSING, COMPLETED, FAILED, PARTIAL] | Yes | Processing state |
| `page_count` | Int32 | Yes | Total pages processed |
| `pages_processed` | Int32 | No | Pages successfully processed |
| `fields_extracted` | Int32 | No | Total fields extracted |
| `fields_validated` | Int32 | No | Fields that passed validation |
| `fields_flagged` | Int32 | No | Fields below confidence threshold |
| `processing_time_ms` | Float | No | Total processing time |
| `llm_token_count` | Int32 | No | LLM tokens consumed (for cost tracking) |
| `llm_cost_usd` | Float | No | LLM cost for this extraction (0.0 for self-hosted) |
| `error_details` | JSONB | No | Error message if failed |
| `created_at` | DateTime | Yes | Auto-set on creation |
| `updated_at` | DateTime | Yes | Auto-updated on status change |
| `completed_at` | DateTime | No | When processing finished |

### ExtractionResult

Stores the output of a completed extraction — the actual field values.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `extraction_job_id` | UUID | Yes | Parent extraction job |
| `field_name` | String (255) | Yes | Name of the extracted field |
| `field_value` | Text | Yes | Extracted value |
| `field_type` | String (64) | Yes | Type of the field (string, number, date, etc.) |
| `confidence` | Float (0-100) | Yes | Confidence score for this field |
| `source_page` | Int32 | Yes | Page number where value was found |
| `source_region` | JSONB | No | Bounding box in original document |
| `is_validated` | Boolean | No | Whether validation passed |
| `validation_errors` | JSONB | No | Validation failure details |
| `requires_review` | Boolean | No | True if confidence below threshold |
| `created_at` | DateTime | Yes | Auto-set on creation |
| `updated_at` | DateTime | Yes | Auto-updated on review/correction |

### ExtractionField

Defines the schema-level field definition for extraction.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `schema_id` | UUID | Yes | Parent extraction schema |
| `name` | String (255) | Yes | Field name (e.g., "invoice_number") |
| `label` | String (255) | Yes | Human-readable label (e.g., "Invoice Number") |
| `field_type` | Enum: [STRING, INTEGER, FLOAT, DATE, DATETIME, CURRENCY, EMAIL, PHONE, ADDRESS, BOOLEAN, JSON] | Yes | Data type |
| `is_required` | Boolean | No | Whether field must be extracted |
| `confidence_threshold` | Float (0-100) | No | Minimum confidence to accept without review |
| `regex_pattern` | String (512) | No | Regex pattern for format validation |
| `default_value` | Text | No | Default value if not found |
| `source_hint` | String (512) | No | LLM extraction hint (e.g., "look in top-right corner") |
| `created_at` | DateTime | Yes | Auto-set on creation |
| `updated_at` | DateTime | Yes | Auto-updated on change |

### ExtractionError

Captures extraction failures for a job or specific field.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `extraction_job_id` | UUID | Yes | Parent extraction job |
| `field_name` | String (255) | No | Affected field (nullable for job-level errors) |
| `error_code` | String (64) | Yes | Machine-readable error code |
| `error_message` | String (1024) | Yes | Human-readable error description |
| `error_category` | Enum: [EXTRACTION_FAILURE, VALIDATION_FAILURE, LLM_ERROR, TEMPLATE_NOT_FOUND, CONFIGURATION_ERROR] | Yes | Error category |
| `retry_count` | Int32 | No | Number of retry attempts |
| `created_at` | DateTime | Yes | Auto-set on creation |

---

## Entity Relationships

```
ExtractionJob
  ├── DocumentIngestJob (via ingest_job_id)          ← source document
  ├── DocumentClassification (via classification_id)  ← classified document
  ├── ExtractionTemplate (via extraction_template_id) ← template used
  ├── ExtractionResult × N                            ← per-field results
  ├── ExtractionError × N                             ← per-field/job errors
  └── res.users (via reviewer_id)                     ← human reviewer

ExtractionResult
  ├── ExtractionJob                                     ← parent job
  ├── ExtractionField                                   ← schema field definition
  └── OCRResult (via source_page)                       ← source page reference

ExtractionError
  └── ExtractionJob                                     ← error source

ExtractionTemplate
  ├── ExtractionSchema (via extraction_schema_id)       ← linked schema
  └── DocumentClass (via document_class_id)             ← document type
  └── ExtractionJob × N                                 ← usage history

ExtractionSchema
  ├── ExtractionField × N                               ← field definitions
  ├── ExtractionTemplate × N                            ← templates using this schema
  └── DocumentClass (via document_class_id)             ← applicable document type

DocumentIngestJob
  └── ExtractionJob × N                                 ← one extraction per job
```

---

## Required API Endpoints

### ExtractionJob Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/extraction/jobs` | List extraction jobs with status, template, filters |
| `GET` | `/extraction/jobs/{id}` | Get extraction job detail with all field results |
| `POST` | `/extraction/jobs` | Submit document for extraction |
| `DELETE` | `/extraction/jobs/{id}` | Cancel extraction job |
| `GET` | `/extraction/jobs/{id}/status` | Poll job status |
| `POST` | `/extraction/jobs/{id}/retry` | Retry failed extractions |

### Extraction Results

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/extraction/jobs/{id}/results` | Get all extracted fields for a job |
| `GET` | `/extraction/jobs/{id}/results/{field_name}` | Get specific field value and confidence |
| `GET` | `/extraction/jobs/{id}/results/highlights` | Get document highlights showing field locations |
| `PATCH` | `/extraction/results/{result_id}/review` | Submit human review/correction for a field |
| `GET` | `/extraction/jobs/{id}/results/summary` | Get summary: total fields, validated, flagged |

### Schema & Template Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/extraction/schemas` | List all extraction schemas |
| `POST` | `/extraction/schemas` | Create a new extraction schema |
| `PATCH` | `/extraction/schemas/{id}` | Update schema definition |
| `DELETE` | `/extraction/schemas/{id}` | Delete a schema |
| `GET` | `/extraction/templates` | List extraction templates |
| `POST` | `/extraction/templates` | Create a new extraction template |
| `PATCH` | `/extraction/templates/{id}` | Update template configuration |
| `DELETE` | `/extraction/templates/{id}` | Delete a template |

### LLM-Based Extraction

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/extraction/llm-extract` | LLM-native extraction (natural language prompt) |
| `POST` | `/extraction/llm-extract/define-schema` | Auto-generate schema from natural language |
| `GET` | `/extraction/llm-extract/cost-report` | Get LLM usage and cost report |

### Batch Extraction

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/extraction/jobs/batch` | Submit multiple documents for batch extraction |
| `GET` | `/extraction/jobs/batch/{batch_id}` | Get batch extraction status |

### Validation

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/extraction/jobs/{id}/validate` | Run business rule validation on extracted fields |
| `GET` | `/extraction/validation-rules` | List configured validation rules |
| `POST` | `/extraction/validation-rules` | Create new validation rule |
| `PATCH` | `/extraction/validation-rules/{id}` | Update validation rule |

---

## Competitive Positioning

### Where RERP Wins

- **LLM-native extraction with no templates** — DocuPipe pioneered this ($0.001/request), but RERP does it self-hosted and free. No template creation, no field mapping, no layout analysis. Describe what you need, get structured data.
- **No per-extraction pricing** — DocuPipe charges $0.001/request, Google Document AI charges $1.50-250/page, Azure charges $5-25/1,000 documents. RERP: $0 marginal cost. For 1M extractions: **$1,000 (DocuPipe), $1,500-250,000 (Google), $5,000-25,000 (Azure).**
- **Self-hosted LLM extraction** — Run extraction on your own LLM infrastructure. Documents never leave your environment. Cloud alternatives require sending documents to their servers.
- **Human-in-the-loop correction** — Reviewers correct low-confidence fields, and corrections improve the extraction pipeline over time. Built-in feedback loop.
- **Schema + LLM hybrid** — Use schemas for high-volume, stable document types (invoices, receipts) and LLM for ad-hoc, unpredictable documents. Best of both worlds.

### Where RERP Lags

- **Pre-built extraction models** — Google Document AI has prebuilt invoice extraction that works immediately. Azure has prebuilt invoice, receipt, business card models. RERP starts with schema/LLM only.
- **Production accuracy** — ABBYY and Google's extraction models have been tuned on millions of real invoices/receipts. RERP's models need training time.
- **Industry-specific extraction** — Healthcare (HIPAA forms), insurance (claim forms), government (tax forms) have specialized extraction models. RERP needs to build these.
- **Cloud API fallback** — For edge cases where self-hosted extraction underperforms, competitors have cloud APIs as fallback. RERP must build this fallback mechanism.

---

## Competitive Intelligence Deep Dive

### DocuPipe (LLM-Native, $0.001/request)

DocuPipe is the pioneer of LLM-native document extraction. Instead of templates or models, you describe the fields you need and the LLM extracts them. Pricing: $0.001/request (1,000 requests = $1.00). Key advantage: truly zero-configuration extraction. Any document type, any field set. Key disadvantage: documents go to DocuPipe's servers (cloud-native). **For RERP: RERP's self-hosted LLM extraction is the open-source, self-hosted equivalent. Same capability, no per-request fee, no data egress.**

### Google Document AI Custom Extractor

Google's custom document AI allows you to train custom extractors on your own document types. Requires 20-30 sample documents per document type and 5-10 labeled fields per sample. Pricing: $1.50/1,000 pages for custom processor. Key advantage: pre-trained on Google's massive dataset, strong accuracy. Key disadvantage: expensive, requires labeled data, documents leave your environment. **For RERP: Zero-shot LLM extraction replaces the need for custom processor training.**

### Azure Prebuilt Invoice Model

Azure's prebuilt invoice model extracts 35+ invoice fields automatically: vendor name, vendor address, customer name, invoice number, invoice date, due date, total, subtotal, tax, line items. Accuracy: 90-95% on standard invoices, 75-85% on non-standard. Pricing: $5/1,000 pages. Key advantage: works out of the box for invoices. Key disadvantage: limited to Azure ecosystem, per-page pricing. **For RERP: LLM-based extraction achieves similar accuracy without pre-training, at zero marginal cost.**

### ABBYY Extraction

ABBYY's extraction is template-based. You create extraction templates that define field locations, patterns, and validation rules. Templates can be shared across document variants. Accuracy: 95%+ on well-formatted documents, 80-90% on variable formats. Pricing: included in ABBYY Vantage ($5,000-50,000/server). Key advantage: mature, stable, auditable templates. Key disadvantage: template maintenance is expensive, no zero-shot capability. **For RERP: LLM extraction eliminates template maintenance — no templates to create, no templates to update.**

### Nanonets ($0.30/run)

Nanonets uses AI extraction with a simple pricing model: $0.30 per extraction run. Requires 50+ training samples per document type. Accuracy: 93-96% after training. Pricing: $49-99/month subscription + $0.30 per 1,000 documents. Key advantage: fast setup, good accuracy. Key disadvantage: per-document pricing, requires labeled training data. **For RERP: Zero training needed (LLM-based) vs. 50+ samples per class, and $0 marginal cost vs. $0.30/run.**

### Rossum Extraction

Rossum uses "No-Code" extraction — define fields via UI, and Rossum's AI learns from corrections. No templates, no training data required. Pricing: $4,000-10,000/year subscription. Key advantage: genuinely no-code, learns from corrections. Key disadvantage: cloud-only, subscription pricing, limited customization. **For RERP: Self-hosted, schema-based + LLM extraction with built-in human-in-the-loop learning.**

---

## Implementation Roadmap

### Phase 1: Core Extraction (Weeks 1-4) — P1

1. Define `ExtractionSchema`, `ExtractionTemplate`, `ExtractionJob`, `ExtractionResult`, `ExtractionField`, `ExtractionError` entities in OpenAPI spec
2. Implement schema-based extraction with typed fields (string, number, date, currency, email)
3. Implement confidence scoring per field (0-100%)
4. Implement low-confidence review queue (< 70% confidence threshold)
5. Add basic business rule validation (regex pattern matching, required fields)
6. Implement extraction result storage with field-level structure
7. Generate Rust server stubs from OpenAPI spec

### Phase 2: LLM-Based Extraction (Weeks 5-8) — P1

1. Integrate local LLM for extraction (via vLLM or ollama)
2. Implement natural language extraction prompt ("extract vendor, invoice number, total")
3. Implement structured output parsing (JSON schema enforcement)
4. Implement schema auto-generation from natural language description
5. Add LLM token counting and cost tracking (0.0 for self-hosted)
6. Implement LLM extraction fallback when schema extraction fails
7. Benchmark LLM accuracy against schema-based extraction

### Phase 3: Advanced Extraction (Weeks 9-12) — P1

1. Implement multi-page extraction (spanning page boundaries)
2. Add table extraction with cell-level structure
3. Implement form field extraction (checkbox, radio, dropdown, text)
4. Add human-in-the-loop correction workflow
5. Implement correction feedback loop (corrected values improve future extractions)
6. Add extraction validation against external data (vendor master, customer master)

### Phase 4: Scale & Optimization (Weeks 13-16) — P1

1. Implement batch extraction with parallel processing
2. Add extraction result caching (skip re-extraction for identical documents)
3. Implement extraction accuracy dashboard (per-field, per-document, per-template)
4. Add extraction audit trail (who extracted what, when, with what confidence)
5. Implement cloud API fallback integration (Google/Azure for edge cases)
6. Performance benchmark: target 100 extractions/minute sustained rate

---

## Key Takeaway for Buyers

RERP's Data Extraction offers a fundamentally different value proposition than competitors: **no templates, no training data, no per-extraction fee, no data egress.** While DocuPipe charges $0.001/request and Google/Azure charge $1.50-250/page for extraction, RERP delivers the same LLM-native capability self-hosted at zero marginal cost per extraction.

The hybrid approach (schema-based for stable, high-volume documents + LLM-based for ad-hoc, unpredictable documents) gives you the best of both worlds. Schema-based extraction is deterministic and auditable for known document types. LLM-based extraction handles anything you haven't seen before — new vendors, custom forms, unexpected layouts — without requiring a single line of template code.

Human-in-the-loop correction closes the loop: low-confidence extractions are reviewed by humans, corrections are logged, and the system learns over time. This is the same learning loop that Rossum offers, but RERP delivers it self-hosted and open-source.

**For buyers: If your document types are stable and known, schema-based extraction gives you deterministic, auditable results. If your document types are unpredictable and changing, LLM-based extraction adapts without template maintenance. Either way, you pay nothing per extraction and your data stays in your environment. That's a competitive advantage that no cloud-based extraction service can match.**
