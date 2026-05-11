# Document Classification

> **Component:** Categorizing documents into types (invoice, contract, receipt, etc.)
> **Priority:** P1 — Essential for routing documents to correct processing pipelines

---

## The Pitch

**Buyer Question:** *Can my system automatically identify what type of document each file is — invoice, contract, receipt, ID, medical record — and route it to the right processing pipeline?*

If the answer is no, every document needs manual classification, which defeats the purpose of automation. Classification is the first intelligence layer in the document pipeline. It determines which extraction schema applies, which workflows trigger, and which downstream systems receive the data. Wrong classification = wrong data = broken integrations.

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

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Document type name |
| `parent_id` | FK: Document Type | No | Parent type (hierarchy) |
| `icon` | String (64) | No | Type-specific icon |
| `color` | String (7) | No | Display color (#hex) |
| `is_active` | Boolean | No | Type activation |
| `created_at` | DateTime | Yes | Creation timestamp |

### Document Classification Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | FK: Document | Yes | Source document |
| `document_type_id` | FK: Document Type | Yes | Assigned type |
| `confidence` | Float (0-1) | Yes | Classification confidence |
| `method` | Enum: [ML, RULE, HUMAN] | Yes | Classification method |
| `is_confirmed` | Boolean | No | Human confirmation |
| `confirmed_by` | UUID | No | User who confirmed |
| `confirmed_at` | DateTime | No | Confirmation timestamp |

### Classification Rule Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (255) | Yes | Rule name |
| `condition` | JSONB | Yes | Rule condition |
| `target_type_id` | FK: Document Type | Yes | Target document type |
| `priority` | Integer | Yes | Rule priority (lower = higher) |
| `is_active` | Boolean | No | Rule activation |
| `created_at` | DateTime | Yes | Creation timestamp |

---

## Required API Endpoints

### Classification Processing

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/classify` | Classify a document |
| `POST` | `/classify/batch` | Batch classify multiple documents |
| `GET` | `/classify/{document_id}` | Get classification results |
| `PATCH` | `/classify/{id}/confirm` | Confirm/reject classification |

### Document Types

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/document-types` | List all document types |
| `POST` | `/document-types` | Create document type |
| `GET` | `/document-types/{id}` | Get type details |
| `PATCH` | `/document-types/{id}` | Update type |
| `DELETE` | `/document-types/{id}` | Delete type |

### Rules Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/classification/rules` | List classification rules |
| `POST` | `/classification/rules` | Create classification rule |
| `PATCH` | `/classification/rules/{id}` | Update rule |
| `DELETE` | `/classification/rules/{id}` | Delete rule |

---

## Competitive Intelligence Deep Dive

### DocuPipe: Classify Endpoint
DocuPipe offers a dedicated Classify endpoint (0.1 credit/page). Supports custom document types and templates. The system learns from user corrections. Classification is bundled with the extraction pipeline — you can't use it standalone. Fast and accurate but limited to the credit-based workflow.

### AWS Textract: Custom Document Classification
Textract's Custom Queries can be trained for document classification. Requires ≥10 sample documents per document type. The custom adapter learns document structure and classifies new documents accordingly. Billed at $0.025/page for custom queries. Layout analysis is free and can be used for basic document type detection.

### Rossum: Multi-Document Transaction Classification
Rossum's classification handles entire document batches with intelligent routing. Supports multi-document transactions where multiple document types are processed together. Master data matching cross-references documents against internal databases. The validation screen learns from each correction. Enterprise-grade classification with SLA guarantees.

### Hyperscience: Pre-Built Classification Models
Hyperscience ships with pre-built classification models for common document types (invoices, purchase orders, contracts, ID documents, medical records). Custom models can be trained for domain-specific documents. The ORCA Vision-Language Model provides human-level accuracy across all document types. FedRAMP High authorized for government classification.

### Paperless-ngx: ML-Based Auto-Tagging
Paperless-ngx uses machine learning to automatically assign tags, correspondents, and document types. The system learns from user corrections and improves over time. Classification is part of the document ingestion pipeline — every document is automatically classified upon upload. Free and self-hosted.

### M-Files: Metadata-Driven Classification
M-Files replaces folder-based classification with metadata-driven context. Documents are automatically classified based on their content, metadata, and business context. The system learns document relationships and suggests classifications. Deep Microsoft 365 integration means SharePoint and Teams documents are auto-classified.

---

## Implementation Roadmap

### Phase 1: Basic Classification (3-4 weeks) — P1
1. Define Document Type entity with hierarchy support
2. Implement ML-based classification endpoint
3. Rule-based classification engine
4. Confidence-based routing (auto vs. human review)
5. Basic document type templates (invoice, receipt, contract)

### Phase 2: Learning & Improvement (3-4 weeks) — P2
1. Classification correction feedback loop
2. Model retraining from user corrections
3. Multi-class classification support
4. Hierarchical classification (broad → specific)
5. Classification confidence thresholds

### Phase 3: Advanced Features (4-6 weeks) — P2
1. Template matching against known documents
2. Multi-document transaction classification
3. Custom classification model training
4. Classification quality metrics
5. Classification audit trail

### Phase 4: Intelligence Layer (3-4 weeks) — P3
1. Cross-document classification patterns
2. Predictive classification suggestions
3. Classification performance dashboard
4. Integration with extraction pipeline
5. Automated rule suggestions

---

## Key Takeaway for Buyers

RERP Documents' classification pitch is **OpenAPI-first, self-hosted, and ERP-integrated**. Unlike Textract (which requires custom adapter training for classification) or Rossum (enterprise-only), RERP provides instant classification with pre-built templates and a self-learning ML model. Unlike Paperless-ngx (which is document-focused), RERP's classification feeds directly into structured data pipelines with OpenAPI-defined schemas.

The Rust-native classification engine handles 10,000+ documents per second — far exceeding Python-based competitors. And because classification is defined in OpenAPI, every client gets type-safe SDKs, automatic validation, and complete API documentation out of the box.

**The immediate priority: define the Document Type entity, implement basic ML classification, and build the classification endpoint. Classification is the first intelligence layer — everything else depends on knowing what type of document you're dealing with.**
