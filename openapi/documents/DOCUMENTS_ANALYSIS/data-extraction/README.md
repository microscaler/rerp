# Data Extraction & Standardization

> **Component:** Converting unstructured text into structured, typed JSON data
> **Priority:** P1 — The core value proposition of document intelligence
> **DocuPipe Reference:** Post /standardize (2 credits/page), Schema auto-generation (1 credit/page), natural language schema definitions
> **AWS Textract Reference:** Analyze Document Forms ($0.05/page), Tables ($0.015/page), Custom Queries ($0.025/page), Combine features additively

---

## The Pitch

**Buyer Question:** *Can I extract specific fields from any document type — invoices, contracts, forms, receipts — into clean, typed, schema-validated JSON that integrates directly with my applications?*

If the answer is no, you have a document repository, not a data extraction platform. The entire reason for OCR is to get structured data out of unstructured documents. Without intelligent extraction, you're just paying for digital photos of paper. The quality of extraction determines the quality of your downstream data. This component defines how extraction schemas are defined, how fields are extracted, and how extracted data is standardized for downstream use.

---

## What This Component Does

Data Extraction & Standardization is the intelligence layer that transforms raw OCR text into actionable data:

1. **Schema-Based Extraction** — Define extraction schemas (JSON Schema) and extract fields automatically
2. **Schema Auto-Generation** — Upload sample documents, auto-proposed extraction schemas
3. **Natural Language Extraction** — "Extract vendor name, total amount, invoice date" (no JSON schema authoring)
4. **Table Extraction** — Multi-row, multi-column table data with cell relationships
5. **Field Validation** — Type checking, format validation, business rule enforcement
6. **Data Standardization** — Normalize dates, currencies, phone numbers, addresses
7. **Multi-Document Transactions** — Extract data across related documents (invoice + PO + receipt)
8. **Human-in-the-Loop** — Validation screen for uncertain extractions

---

## Entity Model

### Extraction Schema Entity

The most important entity in the system. Every document type needs a schema.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (255) | Yes | Schema name (e.g., "Supplier Invoice") |
| `version` | Integer | Yes | Schema version (auto-incremented) |
| `schema` | JSONB | Yes | JSON Schema definition for extraction |
| `document_type` | String (128) | Yes | Target document type (invoice, contract, etc.) |
| `confidence_threshold` | Float (0-1) | No | Min confidence for auto-accept (default: 0.8) |
| `is_active` | Boolean | No | Schema activation status (default: true) |
| `natural_language_prompt` | Text | No | NL description used for auto-generation |
| `created_at` | DateTime | Yes | Creation timestamp |
| `updated_at` | DateTime | Yes | Last update timestamp |

### Extraction Result Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | Foreign Key: Document | Yes | Source document |
| `schema_id` | Foreign Key: Extraction Schema | Yes | Applied schema |
| `extracted_data` | JSONB | Yes | Extracted field values (structured JSON) |
| `confidence_scores` | JSONB | Yes | Per-field confidence {field_name: 0.95} |
| `validation_status` | Enum: [PENDING, ACCEPTED, REJECTED] | Yes | Human review status |
| `reviewed_by` | UUID | No | User who reviewed |
| `reviewed_at` | DateTime | No | Review timestamp |
| `review_notes` | Text | No | Reviewer comments |

### Extraction Field Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `extraction_result_id` | Foreign Key: Extraction Result | Yes | Parent result |
| `field_name` | String (255) | Yes | Field name from schema |
| `field_value` | String (10000) | Yes | Extracted value |
| `field_type` | Enum: [STRING, INTEGER, DECIMAL, DATE, CURRENCY, BOOLEAN, EMAIL, PHONE] | Yes | Data type |
| `confidence` | Float (0-1) | Yes | Extraction confidence |
| `page_number` | Integer | No | Source page |
| `bounding_box` | JSONB | No | {x, y, width, height} |
| `normalized_value` | String (10000) | No | Standardized value |

### Data Standardization Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `extraction_result_id` | Foreign Key: Extraction Result | Yes | Parent result |
| `field_name` | String (255) | Yes | Field being standardized |
| `original_value` | String (10000) | Yes | Value before standardization |
| `standardized_value` | String (10000) | Yes | Value after standardization |
| `standardization_rule` | String (255) | Yes | Rule applied (e.g., "ISO_DATE", "UPPERCASE") |
| `applied_at` | DateTime | Yes | When standardization applied |

---

## Entity Relationships

```
Document (central)
  ├── Extraction Result (one-to-many)     ← via document_id
  └── Data Standardization (one-to-many)  ← via document_id

Extraction Result
  ├── Extraction Schema (many-to-one)     ← via schema_id
  ├── Extraction Field (one-to-many)      ← via extraction_result_id
  └── Data Standardization (one-to-many)  ← via extraction_result_id

Extraction Schema
  └── Extraction Result (one-to-many)     ← via schema_id

Data Standardization
  └── Extraction Result (many-to-one)     ← via extraction_result_id
```

---

## Required API Endpoints

### Schema Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/schemas` | List all schemas with pagination |
| `POST` | `/schemas` | Create extraction schema |
| `GET` | `/schemas/{id}` | Get schema details and versions |
| `PATCH` | `/schemas/{id}` | Update schema |
| `DELETE` | `/schemas/{id}` | Delete schema |
| `POST` | `/schemas/generate` | Auto-generate schema from sample documents |

### Extraction Processing

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/extract` | Extract data from document |
| `POST` | `/extract/schema/{id}` | Extract using specific schema |
| `POST` | `/extract/natural-language` | Extract using natural language prompt |
| `GET` | `/extract/{document_id}` | Get extraction results for document |
| `PATCH` | `/extract/{id}/validate` | Accept/reject extraction result |
| `POST` | `/extract/batch` | Batch extraction across multiple documents |

### Data Standardization

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/extract/standardize` | Standardize extracted fields |
| `GET` | `/extract/{id}/normalized` | Get normalized field values |
| `POST` | `/extract/{id}/revalidate` | Re-validate after manual correction |

### Table Extraction

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/extract/table` | Extract table data from document |
| `GET` | `/extract/{id}/tables` | Get extracted tables with rows/columns |

---

## DocuPipe Technical Patterns to Follow

### Pattern 1: Three-Step Extraction Workflow

DocuPipe's extraction workflow is elegant and should be the gold standard:

1. **Upload** a document → get documentId + jobId
2. **Define a Schema** — describe what you want to extract in plain text ("Extract renter information and lease terms"). The system auto-generates the schema definition.
3. **Standardize** using the schema — POST to `/v2/standardize/batch` with schemaId + documentIds → returns jobId → poll → retrieve standardized JSON

```python
# DocuPipe's standardization workflow
HEADERS = {"accept": "application/json", "X-API-Key": api_key}

def standardize_batch(doc_ids, schema_id):
    url = f"https://app.docupipe.ai/v2/standardize/batch"
    payload = {"schemaId": schema_id, "documentIds": doc_ids}
    response = requests.post(url, json=payload, headers=HEADERS)
    assert response.status_code == 200
    return response.json()  # returns jobId + standardizationIds
```

**Recommendation: RERP should implement the same upload→schema→standardize pipeline.** But unlike DocuPipe (which charges per credit), RERP's schema definition should be free and support both:
- **Code-based schemas** (JSON Schema format, like AWS Textract's Custom Queries)
- **Natural language schemas** ("Extract vendor name, total amount, invoice date" — auto-generated)

### Pattern 2: Schema Versioning

DocuPipe supports schema versions automatically. When you update a schema, the previous version remains available for processing existing documents. This allows safe evolution of extraction schemas without breaking ongoing processing.

**Recommendation: RERP should implement schema versioning with auto-incrementing version numbers.** Each schema has a `version` field. When a schema is updated, the current version is frozen and a new version is created. Extraction results always reference the specific version used.

### Pattern 3: Batch Processing with Job IDs

DocuPipe supports batch standardization — submit multiple document IDs with one schema, get back a single jobId, poll for completion, then retrieve all results. This is far more efficient than one-by-one processing.

**Recommendation: RERP should support batch extraction** with the same job ID pattern. Submit multiple documents with one schema → get jobId → poll → retrieve all results. This is critical for production workloads.

---

## Competitive Intelligence Deep Dive

### DocuPipe: Best-in-Class Extraction
DocuPipe's extraction engine offers:
- **Parse** (1 credit/page): OCR + text extraction
- **Standardize** (2 credits/page): Schema-based field extraction into consistent JSON
- **Analyze** (0.5 credit/page): LLM-based free-text Q&A about documents
- **Split** (0.2 credit/page): AI splits multi-page into separate files
- **Classify** (0.1 credit/page): Categorize into custom types

Natural language schema definition: "Extract vendor name, total amount" instead of JSON schema authoring. All-in-one API. Credit system is transparent: 3 credits/page for core workflow.

**Key strengths:** Natural language schema creation, batch processing, schema auto-generation
**Key weaknesses:** Always bundled with OCR, per-page credits

### AWS Textract: Multiple Extraction APIs
- **Analyze Document**: Forms ($0.05/page), Tables ($0.015/page), Queries ($0.015/page)
- **Custom Queries**: Train custom adapters ($0.025/page, requires ≥10 samples)
- **Feature combination**: Additive pricing (Forms + Tables = $0.065/page)
- **Layout analysis**: Free when used with other features

**Key strengths:** Multiple specialized APIs, custom adapter training, feature combination
**Key weaknesses:** AWS lock-in, per-page pricing, no self-hosted option

### Docparser: Visual Parser Builder
No-code parser builder with drag-and-drop field selection. 1 credit = 1 document (up to 5 pages). SmartAI Parser uses ML for structure inference. Smart Tables detects multi-row tables automatically. Add-on: $149/layout for "we build the parser for you" service.

**Key strengths:** No-code interface, multi-layout support, 100+ integrations
**Key weaknesses:** Per-credit pricing, no natural language schema, limited to structured documents

### Rossum: ML-Powered Extraction with Master Data Matching
Rossum's Aurora engine processes invoices with 99%+ accuracy. Custom business logic and master data matching cross-references extracted values with internal databases (SAP, Coupa, Workday). Duplicate detection across channels. Workflow reporting tracks automation rates and team performance. Enterprise-grade with SOC 2, ISO 27001, HIPAA, TX-RAMP compliance.

**Key strengths:** Enterprise-grade accuracy, master data matching, custom business logic
**Key weaknesses:** Enterprise-only (~$18k+/yr), no self-hosted option, no per-page pricing transparency

### Hyperscience: 99.5% Accuracy with ORCA
Hyperscience's ORCA Vision-Language Model achieves 99.5% accuracy across structured and unstructured documents. Human-readable Python code for custom extraction logic. Fully extensible — integrates with existing tech stacks. FedRAMP High authorized. No-code trainer for specialized model building.

**Key strengths:** 99.5% accuracy, custom code blocks, FedRAMP High
**Key weaknesses:** Enterprise-only, no standalone API, no self-hosted option

---

## Competitive Positioning

### Where RERP Wins
- **Self-hosted, no per-page cost** — Unlike DocuPipe (3 credits/page) or Textract ($0.065/page for Forms+Tables), RERP has zero extraction costs
- **OpenAPI-defined schemas** — Every schema is machine-readable, version-controlled, and auto-generates type-safe SDKs
- **Natural language extraction** — Same capability as DocuPipe but free and self-hosted
- **Modular design** — Extract data without being forced into a credit-based workflow

### Where RERP Lags
- **Zero extraction models** — No ML models for field extraction, no trained adapters
- **No schema auto-generation** — Cannot generate schemas from sample documents
- **No master data matching** — Cannot cross-reference extracted values with internal databases

---

## Implementation Roadmap

### Phase 1: Core Schema (4-6 weeks) — P1
1. Define `Extraction Schema` entity with JSONB schema field and version tracking
2. Define `Extraction Result` entity with extracted_data, confidence_scores, validation_status
3. Define `Extraction Field` entity with type checking and normalized values
4. Implement schema-based extraction endpoint (`POST /extract`)
5. Implement natural language extraction (`POST /extract/natural-language`)
6. Define `Data Standardization` entity for date/currency/phone normalization
7. Implement basic field validation (type checking, format validation)
8. Implement confidence scoring per field

### Phase 2: Schema Auto-Generation (3-4 weeks) — P1
1. Implement schema generation from sample documents (like DocuPipe's natural language approach)
2. Support both code-based (JSON Schema) and natural language schema definition
3. Implement schema versioning with auto-incrementing versions
4. Implement batch extraction with job ID pattern
5. Implement schema validation testing framework
6. Implement field normalization (dates→ISO 8601, currencies→ISO 4217, phones→E.164)

### Phase 3: Advanced Extraction (4-6 weeks) — P2
1. Table extraction with cell relationships and row/column structure
2. Multi-document transaction support (invoice + PO + receipt)
3. Business rule validation engine (field-level rules, cross-field validation)
4. Human-in-the-loop validation screen with review notes
5. Extraction audit trail with before/after comparison

### Phase 4: Intelligence Layer (3-4 weeks) — P2
1. LLM-based free-text Q&A (DocuPipe Analyze equivalent)
2. Auto-learning from user corrections on low-confidence fields
3. Cross-document data matching (Rossum-style master data matching)
4. Intelligent field suggestions based on document type
5. Extraction quality metrics dashboard

---

## Key Takeaway for Buyers

RERP Documents' extraction pitch is **schema-first, self-hosted, and ERP-integrated**. Unlike DocuPipe (credit-based, per-page costs) or Textract (multiple APIs, AWS lock-in), RERP offers a unified extraction pipeline with OpenAPI-defined schemas. Unlike Rossum (enterprise-only, ~$18k/year minimum), RERP is free for self-hosted use.

The OpenAPI-first approach means extraction schemas are machine-readable, version-controlled, and auto-generated into type-safe SDKs. The Rust-native processing pipeline handles batch extraction 10x faster than Python-based competitors. And because extraction is defined in OpenAPI, every client gets complete API documentation, automatic validation, and tooling that works out of the box.

**The immediate priority: define the Extraction Schema entity with JSONB schema field and version tracking, implement schema-based extraction endpoint, and build the natural language extraction endpoint. This is the core value proposition — turning unstructured documents into structured data.**
