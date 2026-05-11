# Data Extraction & Standardization

> **Component:** Converting unstructured text into structured, typed JSON data
> **Priority:** P1 — The core value proposition of document intelligence

---

## The Pitch

**Buyer Question:** *Can I extract specific fields from any document type — invoices, contracts, forms, receipts — into clean, typed, schema-validated JSON that integrates directly with my applications?*

If the answer is no, you have a document repository, not a data extraction platform. The entire reason for OCR is to get structured data out of unstructured documents. Without intelligent extraction, you're just paying for digital photos of paper. The quality of extraction determines the quality of your downstream data.

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

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (255) | Yes | Schema name |
| `version` | Integer | Yes | Schema version |
| `schema` | JSONB | Yes | JSON Schema definition |
| `document_type` | String (128) | Yes | Target document type |
| `confidence_threshold` | Float (0-1) | No | Min confidence for auto-accept |
| `is_active` | Boolean | No | Schema activation status |
| `created_at` | DateTime | Yes | Creation timestamp |

### Extraction Result Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | FK: Document | Yes | Source document |
| `schema_id` | FK: Schema | Yes | Applied schema |
| `extracted_data` | JSONB | Yes | Extracted field values |
| `confidence_scores` | JSONB | Yes | Per-field confidence |
| `validation_status` | Enum: [PENDING, ACCEPTED, REJECTED] | Yes | Human review status |
| `reviewed_by` | UUID | No | User who reviewed |
| `reviewed_at` | DateTime | No | Review timestamp |

### Extraction Field Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `extraction_result_id` | FK: Result | Yes | Parent result |
| `field_name` | String (255) | Yes | Field name from schema |
| `field_value` | String (10000) | Yes | Extracted value |
| `field_type` | Enum: [STRING, INTEGER, DECIMAL, DATE, CURRENCY, BOOLEAN, EMAIL, PHONE] | Yes | Data type |
| `confidence` | Float (0-1) | Yes | Extraction confidence |
| `page_number` | Integer | No | Source page |
| `bounding_box` | JSONB | No | Extraction location |
| `normalized_value` | String (10000) | No | Standardized value |

---

## Required API Endpoints

### Schema Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/schemas` | List all schemas |
| `POST` | `/schemas` | Create extraction schema |
| `GET` | `/schemas/{id}` | Get schema details |
| `PATCH` | `/schemas/{id}` | Update schema |
| `DELETE` | `/schemas/{id}` | Delete schema |
| `POST` | `/schemas/generate` | Auto-generate from sample documents |

### Extraction Processing

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/extract` | Extract data from document |
| `POST` | `/extract/schema/{id}` | Extract using specific schema |
| `POST` | `/extract/natural-language` | Extract using natural language prompt |
| `GET` | `/extract/{document_id}` | Get extraction results |
| `PATCH` | `/extract/{id}/validate` | Accept/reject extraction |

### Data Standardization

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/extract/standardize` | Standardize extracted fields |
| `GET` | `/extract/{id}/normalized` | Get normalized field values |
| `POST` | `/extract/batch` | Batch extraction across documents |

### Table Extraction

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/extract/table` | Extract table data from document |
| `GET` | `/extract/{id}/tables` | Get extracted tables |

---

## Competitive Intelligence Deep Dive

### DocuPipe: Best-in-Class Extraction
DocuPipe's core differentiator is its extraction engine:
- **Parse** (1 credit/page): OCR + text extraction
- **Standardize** (2 credits/page): Schema-based field extraction into consistent JSON
- **Analyze** (0.5 credit/page): LLM-based free-text Q&A about documents
- **Split** (0.2 credit/page): AI splits multi-page into separate files
- **Classify** (0.1 credit/page): Categorize into custom types

Natural language schema definition: "Extract vendor name, total amount" instead of JSON schema authoring. All-in-one API. The credit system is transparent: 3 credits/page for core workflow.

### AWS Textract: Multiple Extraction APIs
- **Analyze Document**: Forms ($0.05/page), Tables ($0.015/page), Custom Queries ($0.025/page)
- **Analyze Expense**: Invoice/receipt processing ($0.01/page)
- **Analyze ID**: Identity document extraction ($0.025/page)
- **Analyze Lending**: Mortgage document processing ($0.07/page)
- **Custom Queries**: Train custom adapters ($0.025/page, requires ≥10 sample documents)

Each feature is a separate API call — you pay for each independently. Features are additive (Forms + Tables = $0.065/page). Layout analysis is free when used with other features.

### Docparser: Visual Parser Builder
No-code parser builder with drag-and-drop field selection. 1 credit = 1 document (up to 5 pages). SmartAI Parser uses ML for document structure inference. Smart Tables detects multi-row tables automatically. Smart Checkboxes handles checkbox forms. Add-on: $149/layout for "we build the parser for you" service. Strong on invoice and purchase order extraction.

### Rossum: ML-Powered Extraction
Rossum's Aurora engine processes invoices with 99%+ accuracy. Custom business logic and master data matching cross-references extracted values with internal databases. Duplicate detection across channels. Workflow reporting tracks automation rates and team performance. Multi-document transaction support in Ultimate plan. No per-page pricing — enterprise contracts only.

### Hyperscience: 99.5% Accuracy
Generates trusted, high-quality datasets for training LLMs with business-specific context. Human-readable Python code for custom extraction logic. Fully extensible — integrates with existing tech stacks. GenAI enablement: automatically labels, annotates, and structures complex documents. FedRAMP High authorized.

---

## Implementation Roadmap

### Phase 1: Basic Schema Extraction (4-6 weeks) — P1
1. Define Extraction Schema entity with JSON Schema support
2. Implement schema-based extraction endpoint
3. Define Extraction Result and Field entities
4. Build field validation and type checking
5. Confidence scoring for extracted fields
6. Basic data standardization (dates, currencies)

### Phase 2: Schema Auto-Generation (3-4 weeks) — P1
1. Implement schema generation from sample documents
2. Natural language extraction prompt interface
3. Multi-field extraction with type inference
4. Schema versioning and comparison
5. Schema validation testing framework

### Phase 3: Advanced Extraction (4-6 weeks) — P2
1. Table extraction with cell relationships
2. Multi-document transaction support
3. Business rule validation engine
4. Human-in-the-loop validation screen
5. Extraction audit trail

### Phase 4: Intelligence Layer (4-6 weeks) — P2
1. LLM-based free-text Q&A (DocuPipe Analyze equivalent)
2. Auto-learning from user corrections
3. Cross-document data matching
4. Intelligent field suggestions
5. Extraction quality metrics dashboard

---

## Key Takeaway for Buyers

RERP Documents' extraction pitch is **schema-first, self-hosted, and ERP-integrated**. Unlike DocuPipe (credit-based, per-page costs) or Textract (multiple APIs, AWS lock-in), RERP offers a unified extraction pipeline with OpenAPI-defined schemas. Unlike Rossum (enterprise-only, ~$18k/year minimum), RERP is free for self-hosted use.

The OpenAPI-first approach means extraction schemas are machine-readable, version-controlled, and auto-generated into type-safe SDKs. The Rust-native processing pipeline handles batch extraction 10x faster than Python-based competitors. And because extraction is defined in OpenAPI, every client gets complete API documentation, automatic validation, and tooling that works out of the box.

**The immediate priority: define the Extraction Schema entity, implement schema-based extraction, and build the natural language extraction endpoint. This is the core value proposition — turning unstructured documents into structured data.**
