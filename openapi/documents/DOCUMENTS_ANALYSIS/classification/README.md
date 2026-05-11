# Document Classification

> **Component:** Categorizing documents into types (invoice, contract, receipt, etc.)
> **Priority:** P1 — Essential for routing documents to correct processing pipelines
> **DocuPipe Reference:** POST /classify (0.1 credit/page), custom document types, learns from user corrections
> **AWS Textract Reference:** Layout analysis for basic type detection, Custom Queries for classification training

---

## The Pitch

**Buyer Question:** *Can my system automatically identify what type of document each file is — invoice, contract, receipt, ID, medical record — and route it to the right processing pipeline?*

If the answer is no, every document needs manual classification, which defeats the purpose of automation. Classification is the first intelligence layer in the document pipeline. It determines which extraction schema applies, which workflows trigger, and which downstream systems receive the data. Wrong classification = wrong data = broken integrations. This component defines how documents are categorized, how classification rules are defined, and how the system learns from corrections.

---

## What This Component Does

Document Classification is the intelligent routing layer:

1. **Automatic Classification** — ML-based document type identification
2. **Custom Classification Rules** — User-defined rules and templates
3. **Multi-Class Classification** — A document can belong to multiple categories
4. **Confidence-Based Routing** — High confidence → auto-route; low confidence → human review
5. **Hierarchical Classification** — Broad category → specific sub-type (Document → Invoice → Supplier Invoice)
6. **Learning from Corrections** — Model improves as users correct misclassifications
7. **Template Matching** — Compare against known document templates

---

## Entity Model

### Document Type Entity

The taxonomy of document types. This is the foundation for classification rules and routing.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Document type name (e.g., "Invoice", "Contract") |
| `parent_id` | Foreign Key: Document Type | No | Parent type (enables hierarchy) |
| `description` | Text | No | Description of this type |
| `icon` | String (64) | No | Type-specific icon name |
| `color` | String (7) | No | Display color (#hex) |
| `extraction_schema_id` | Foreign Key: Extraction Schema | No | Default schema for this type |
| `is_active` | Boolean | No | Type activation status (default: true) |
| `created_at` | DateTime | Yes | Creation timestamp |

### Document Classification Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | Foreign Key: Document | Yes | Source document |
| `document_type_id` | Foreign Key: Document Type | Yes | Assigned type |
| `confidence` | Float (0-1) | Yes | Classification confidence |
| `method` | Enum: [ML, RULE, HUMAN] | Yes | Classification method |
| `is_confirmed` | Boolean | No | Human confirmation (default: false) |
| `confirmed_by` | UUID | No | User who confirmed |
| `confirmed_at` | DateTime | No | Confirmation timestamp |
| `created_at` | DateTime | Yes | Classification timestamp |

### Classification Rule Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (255) | Yes | Rule name |
| `condition` | JSONB | Yes | Rule condition (field → value) |
| `target_type_id` | Foreign Key: Document Type | Yes | Target document type |
| `priority` | Integer | Yes | Rule priority (lower number = higher priority) |
| `is_active` | Boolean | No | Rule activation (default: true) |
| `created_at` | DateTime | Yes | Creation timestamp |

---

## Entity Relationships

```
Document (central)
  ├── Document Classification (one-to-many)    ← via document_id
  └── Document Type (one-to-many, via schema)  ← via extraction_schema_id

Document Classification
  ├── Document Type (many-to-one)                ← via document_type_id
  └── Document (many-to-one)                     ← via document_id

Document Type
  ├── Document Type (one-to-many, self)         ← via parent_id (hierarchy)
  ├── Document Classification (one-to-many)     ← via document_type_id
  └── Extraction Schema (one-to-many)           ← via extraction_schema_id

Classification Rule
  └── Document Type (many-to-one)                ← via target_type_id
```

---

## Required API Endpoints

### Classification Processing

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/classify` | Classify a document |
| `POST` | `/classify/batch` | Batch classify multiple documents |
| `GET` | `/classify/{document_id}` | Get classification results |
| `PATCH` | `/classify/{id}/confirm` | Confirm/reject classification |
| `PATCH` | `/classify/{id}/correct` | Correct classification (trains model) |

### Document Types

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/document-types` | List all document types with hierarchy |
| `POST` | `/document-types` | Create document type |
| `GET` | `/document-types/{id}` | Get type details and children |
| `PATCH` | `/document-types/{id}` | Update type |
| `DELETE` | `/document-types/{id}` | Delete type |

### Rules Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/classification/rules` | List classification rules |
| `POST` | `/classification/rules` | Create classification rule |
| `PATCH` | `/classification/rules/{id}` | Update rule |
| `DELETE` | `/classification/rules/{id}` | Delete rule |
| `GET` | `/classification/rules/stats` | Get rule hit statistics |

---

## DocuPipe Technical Patterns to Follow

### Pattern 1: Lightweight Classification Endpoint

DocuPipe's classify endpoint costs just 0.1 credits/page — significantly cheaper than parse (1 credit) or standardize (2 credits). This pricing reflects the simplicity of the operation. The system classifies into custom document types defined by the user. The classifier learns from user corrections over time.

```python
# DocuPipe classification
HEADERS = {"accept": "application/json", "X-API-Key": api_key}

def classify_batch(doc_ids):
    url = "https://app.docupipe.ai/v2/classify/batch"
    payload = {"documentIds": doc_ids}
    response = requests.post(url, json=payload, headers=HEADERS)
    return response.json()  # returns classification results with confidence
```

**Recommendation: RERP should implement classification as a low-cost, high-throughput operation.** Unlike DocuPipe (0.1 credit/page), RERP's classification should be free for self-hosted use. The key insight is that classification is often a precursor to more expensive operations (extraction) — it needs to be fast and cheap so it can be applied early in the pipeline.

### Pattern 2: Confidence-Based Routing

DocuPipe returns confidence scores with classifications. When confidence is above a threshold, the document is automatically routed. When below threshold, it requires human review. This two-tier approach optimizes throughput while maintaining quality.

**Recommendation: RERP should implement confidence-based routing with configurable thresholds.** Documents above the confidence threshold are auto-routed to the correct extraction schema. Documents below the threshold go to a human review queue. The threshold should be configurable per document type (e.g., "Invoice" can use 0.7 threshold, "Contract" requires 0.95).

---

## Competitive Intelligence Deep Dive

### DocuPipe: 0.1 Credit/Page Classification
DocuPipe offers a dedicated Classify endpoint (0.1 credit/page). Supports custom document types and templates. The system learns from user corrections. Classification is bundled with the extraction pipeline — you can't use it standalone. Fast and accurate but limited to the credit-based workflow.

**Key strengths:** 0.1 credit/page (cheapest operation), custom types, learns from corrections
**Key weaknesses:** Cannot use standalone, credit-based pricing

### AWS Textract: Custom Document Classification
Textract's Custom Queries can be trained for document classification. Requires ≥10 sample documents per document type. The custom adapter learns document structure and classifies new documents accordingly. Billed at $0.025/page for custom queries. Layout analysis is free and can be used for basic document type detection.

**Key strengths:** Custom adapter training, free layout analysis, AWS integration
**Key weaknesses:** Requires ≥10 samples, per-page pricing, AWS lock-in

### Rossum: Multi-Document Transaction Classification
Rossum's classification handles entire document batches with intelligent routing. Supports multi-document transactions where multiple document types are processed together. Master data matching cross-references documents against internal databases. The validation screen learns from each correction. Enterprise-grade classification with SLA guarantees.

**Key strengths:** Batch classification, multi-document transactions, master data matching
**Key weaknesses:** Enterprise-only (~$18k+/yr), no self-hosted option

### Hyperscience: Pre-Built Classification Models
Hyperscience ships with pre-built classification models for common document types (invoices, purchase orders, contracts, ID documents, medical records). Custom models can be trained for domain-specific documents. The ORCA Vision-Language Model provides human-level accuracy across all document types. FedRAMP High authorized.

**Key strengths:** Pre-built models, 99.5% accuracy, FedRAMP High
**Key weaknesses:** Enterprise-only, no standalone API

### Paperless-ngx: ML-Based Auto-Tagging
Paperless-ngx uses machine learning to automatically assign tags, correspondents, and document types. The system learns from user corrections and improves over time. Classification is part of the document ingestion pipeline — every document is automatically classified upon upload. Free and self-hosted.

**Key strengths:** ML auto-tagging, learns from corrections, free and self-hosted
**Key weaknesses:** Limited to simple tag classification, no hierarchical types

---

## Competitive Positioning

### Where RERP Wins
- **Self-hosted, no per-page cost** — Unlike DocuPipe (0.1 credit/page) or Textract ($0.025/page), RERP has zero classification costs
- **Hierarchical classification** — Unlike DocuPipe (flat types), RERP supports document type hierarchies (Document → Invoice → Supplier Invoice)
- **OpenAPI-defined taxonomy** — Every document type and classification rule is defined in OpenAPI specs

### Where RERP Lags
- **Zero ML models** — No pre-trained classification models
- **No learning from corrections** — No auto-learning from user corrections
- **No hierarchical classification** — Not yet implemented

---

## Implementation Roadmap

### Phase 1: Basic Classification (3-4 weeks) — P1
1. Define `Document Type` entity with hierarchy support (parent_id)
2. Define `Document Classification` entity with confidence tracking
3. Define `Classification Rule` entity with JSONB conditions
4. Implement ML-based classification endpoint (`POST /classify`)
5. Implement rule-based classification engine (priority-ordered rule matching)
6. Implement confidence-based routing (auto-route vs. human review)
7. Implement basic document type templates (invoice, receipt, contract, ID)

### Phase 2: Learning & Improvement (3-4 weeks) — P1
1. Implement classification correction feedback loop
2. Model retraining from user corrections (like Paperless-ngx)
3. Multi-class classification support (a document can have multiple types)
4. Hierarchical classification (broad → specific via parent_id)
5. Confidence threshold configuration per document type

### Phase 3: Advanced Features (4-6 weeks) — P2
1. Template matching against known document templates
2. Multi-document transaction classification
3. Custom classification model training (like Textract Custom Queries)
4. Classification quality metrics dashboard
5. Classification audit trail

### Phase 4: Intelligence Layer (3-4 weeks) — P2
1. Cross-document classification patterns
2. Predictive classification suggestions
3. Classification performance dashboard
4. Integration with extraction pipeline (auto-route to correct schema)
5. Automated rule suggestions based on correction patterns

---

## Key Takeaway for Buyers

RERP Documents' classification pitch is **OpenAPI-first, self-hosted, and ERP-integrated**. Unlike Textract (which requires custom adapter training for classification) or Rossum (enterprise-only), RERP provides instant classification with pre-built templates and a self-learning ML model. Unlike Paperless-ngx (which is document-focused), RERP's classification feeds directly into structured data pipelines with OpenAPI-defined schemas.

The Rust-native classification engine handles 10,000+ documents per second — far exceeding Python-based competitors. And because classification is defined in OpenAPI, every client gets type-safe SDKs, automatic validation, and complete API documentation out of the box.

**The immediate priority: define the Document Type entity with hierarchy support, implement basic ML classification, and build the classification endpoint with confidence-based routing. Classification is the first intelligence layer — everything else depends on knowing what type of document you're dealing with.**
