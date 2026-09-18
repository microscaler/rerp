# Document Classification

> **Component:** Document type classification — determine what kind of document you have before deciding how to process it
> **Priority:** P1 — Classify before you extract; classification accuracy directly determines extraction path
> **Reference Competitors:** ABBYY CNN classifiers, Google Document AI processors, Azure Document Intelligence models, Rossum's AI classification, Nanonets classification

---

## The Pitch

**Buyer Question:** *Can I automatically determine whether an incoming document is an invoice, contract, purchase order, resume, or something else — and classify it to the right sub-type with a confidence score — without training a new model for every document type?*

Classification is the critical decision point in document processing. Misclassify an invoice as a receipt and your extraction pipeline pulls the wrong fields. Send a contract to a classification rule trained on financial documents and you get garbage. The question isn't whether you need classification — it's whether your classifier can handle new document types without retraining, whether it can distinguish between supplier and customer invoices, and whether it can handle multi-label classification (a document that is both a purchase order AND a contract). RERP's classification engine addresses these gaps with a hybrid approach: template-trained models for high-volume document types, and LLM-based zero-shot classification for everything else.

---

## What This Component Does

Document Classification determines the type and sub-type of every document in the pipeline. It is the second processing step after OCR (or ingestion for native-text documents):

1. **Document Type Classification** — Classify into primary categories: Invoice, Contract, Purchase Order, Receipt, Resume/CV, Form, Letter, ID Document, Email, Shipping Document, Tax Document, Financial Report, HR Document, Technical Manual
2. **Sub-Type Classification** — Drill down into sub-categories: Supplier Invoice vs. Customer Invoice vs. Intercompany Invoice; NDA Contract vs. Employment Contract vs. Service Agreement; W-2 vs. W-4 vs. 1099 tax forms
3. **Confidence Scoring** — Each classification includes a confidence score (0-100%) and a ranked list of alternative classifications. Low-confidence results (< 70%) are flagged for human review
4. **Multi-Label Classification** — Documents can belong to multiple categories simultaneously (e.g., a document that is both a Purchase Order and a Contract). Multi-label detection enables flexible routing to multiple downstream pipelines
5. **Custom Category Training** — Define custom document classes and train classifiers on organization-specific document types (e.g., "XYZ Company Vendor Invoice" vs. generic "Invoice"). Retraining is incremental — new categories are added without retraining the entire model
6. **Zero-Shot Classification with LLMs** — For document types with zero training examples, RERP uses an LLM-based zero-shot classifier. Describe your document type in natural language and the LLM classifies it. This is RERP's key competitive advantage: no pre-training required.
7. **Classification Rule Engine** — Configurable rules override or supplement model classifications. Example: "All documents from vendor@acme.com are Supplier Invoices" — a rule-based override that complements the ML classifier
8. **Classification History & Drift Detection** — Track classification accuracy over time, detect model drift, and automatically trigger retraining when accuracy drops below threshold

---

## Entity Model

### DocumentClass

Defines a document classification category and its metadata.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (255) | Yes | Human-readable class name (e.g., "Supplier Invoice") |
| `parent_class_id` | UUID | No | Parent class for hierarchy (nullable = root class) |
| `class_type` | Enum: [PRIMITIVE, COMPOSITE, CUSTOM] | Yes | Primitive (system-defined), Composite (multi-label), Custom (user-defined) |
| `description` | Text | No | Description of when this class applies |
| `sample_labels` | String Array | No | Keywords/phrases used for zero-shot classification |
| `is_active` | Boolean | No | Soft toggle |
| `created_at` | DateTime | Yes | Auto-set on creation |
| `updated_at` | DateTime | Yes | Auto-updated on change |

### ClassificationRule

Rule-based classification overrides and supplements.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (255) | Yes | Rule name |
| `condition` | JSONB | Yes | Rule condition (see below) |
| `class_id` | UUID | Yes | Target classification |
| `priority` | Int32 | Yes | Higher priority rules override lower priority |
| `is_active` | Boolean | No | Soft toggle |
| `created_at` | DateTime | Yes | Auto-set on creation |
| `updated_at` | DateTime | Yes | Auto-updated on change |

**Condition Examples:**
- `{ "type": "sender", "field": "sender_email", "operator": "equals", "value": "vendor@acme.com" }`
- `{ "type": "content_type", "field": "mime_type", "operator": "in", "value": ["application/pdf", "image/tiff"] }`
- `{ "type": "filename_pattern", "field": "filename", "operator": "regex", "value": "INV-[0-9]{4,}" }`
- `{ "type": "text_match", "field": "ocr_text", "operator": "contains", "value": "PURCHASE ORDER" }`

### ClassificationModel

The ML model used for document classification.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (255) | Yes | Model name |
| `model_type` | Enum: [CNN, TRANSFORMER, LLM_ZERO_SHOT, CUSTOM_TRAINABLE] | Yes | Model architecture type |
| `trained_classes` | UUID Array | Yes | Classes the model was trained on |
| `accuracy` | Float (0-100) | No | Validation accuracy on held-out test set |
| `training_samples` | Int32 | No | Number of training samples used |
| `is_active` | Boolean | No | Soft toggle |
| `created_at` | DateTime | Yes | Auto-set on creation |
| `updated_at` | DateTime | Yes | Auto-updated on change |

### DocumentClassification

The result of a classification operation on a document.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `ingest_job_id` | UUID | Yes | Source document ingestion job |
| `classification_model_id` | UUID | Yes | Model used for classification |
| `classification_rule_id` | UUID | No | Rule that triggered this classification (if rule-based) |
| `primary_class_id` | UUID | Yes | Primary classification |
| `primary_confidence` | Float (0-100) | Yes | Confidence for primary class |
| `is_multi_label` | Boolean | No | Whether multiple classes apply |
| `alternative_classes` | JSONB | No | Ranked list of alternative classifications |
| `requires_review` | Boolean | No | True if confidence below threshold |
| `reviewer_id` | UUID | No | User who confirmed/rejected classification |
| `review_action` | Enum: [CONFIRMED, CORRECTED, ESCALATED] | No | Human reviewer action |
| `review_notes` | Text | No | Reviewer comments |
| `created_at` | DateTime | Yes | Auto-set on creation |
| `updated_at` | DateTime | Yes | Auto-updated on review |

**Alternative Classes Format:**
```json
[
  { "class_id": "uuid-1", "class_name": "Invoice", "confidence": 92.5 },
  { "class_id": "uuid-2", "class_name": "Receipt", "confidence": 8.3 },
  { "class_id": "uuid-3", "class_name": "Form", "confidence": 1.2 }
]
```

---

## Entity Relationships

```
DocumentClassification
  ├── DocumentIngestJob (via ingest_job_id)          ← source document
  ├── ClassificationModel (via classification_model_id) ← model used
  ├── ClassificationRule (via classification_rule_id)   ← rule that matched (optional)
  ├── DocumentClass (via primary_class_id)                ← primary classification
  └── res.users (via reviewer_id)                         ← human reviewer

ClassificationRule
  └── DocumentClass (via class_id)                        ← class assigned by rule

ClassificationModel
  ├── DocumentClass × N (via trained_classes)             ← classes in model
  └── DocumentClassification × N                          ← classification history

DocumentClass
  ├── DocumentClass (via parent_class_id)                 ← self-hierarchy
  ├── ClassificationRule × N                              ← rules targeting this class
  └── DocumentClassification × N                          ← classified documents

DocumentIngestJob
  └── DocumentClassification × N                          ← one classification per job
```

---

## Required API Endpoints

### DocumentClassification

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/classification/results` | List classification results with filters |
| `GET` | `/classification/results/{id}` | Get full classification detail with alternatives |
| `POST` | `/classification/evaluate` | Classify a document (by job ID or file upload) |
| `GET` | `/classification/results/{id}/review` | Get review status and history |
| `PATCH` | `/classification/results/{id}/review` | Submit human review (confirm/correct/escalate) |
| `GET` | `/classification/results/review-queue` | List unreviewed low-confidence classifications |

### DocumentClass Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/classification/classes` | List all document classes (with hierarchy) |
| `POST` | `/classification/classes` | Create a new custom document class |
| `PATCH` | `/classification/classes/{id}` | Update class metadata |
| `DELETE` | `/classification/classes/{id}` | Deactivate a class |
| `GET` | `/classification/classes/tree` | Get full class hierarchy tree |

### ClassificationModel Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/classification/models` | List available classification models |
| `POST` | `/classification/models` | Register a new classification model |
| `PATCH` | `/classification/models/{id}` | Update model configuration |
| `POST` | `/classification/models/{id}/train` | Trigger model retraining |
| `GET` | `/classification/models/{id}/accuracy` | Get model accuracy metrics over time |
| `POST` | `/classification/models/{id}/deploy` | Activate model for production use |

### ClassificationRule Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/classification/rules` | List all classification rules |
| `POST` | `/classification/rules` | Create a new classification rule |
| `PATCH` | `/classification/rules/{id}` | Update rule conditions |
| `DELETE` | `/classification/rules/{id}` | Deactivate a rule |
| `POST` | `/classification/rules/test` | Test a rule against sample documents |

### Zero-Shot Classification

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/classification/zero-shot` | Zero-shot classify using LLM |
| `POST` | `/classification/zero-shot/define-class` | Define a new class via natural language description |
| `GET` | `/classification/zero-shot/available-classes` | List zero-shot available categories |

---

## Competitive Positioning

### Where RERP Wins

- **Zero-shot classification with LLMs** — Describe a new document type in plain language and classify it immediately. ABBYY CNN, Google Document AI processors, and Kofax all require pre-trained models. Nanonets requires 50+ labeled samples per class. RERP: zero samples needed.
- **Hybrid rule + ML classification** — Confidence scoring with rule-based overrides. ABBYY's CNN classifiers are pure ML; Google's processors are fixed. RERP lets you combine both.
- **Multi-label classification** — A document can be both a PO and a contract. Most competitors force a single-label classification.
- **Self-hosted, no per-document cost** — Classification runs locally. Google Document AI charges $1.50-250/page. Azure Document Intelligence charges $5-25/1,000 documents.
- **Custom category training** — Add organization-specific document types without vendor intervention.

### Where RERP Lags

- **Pre-trained models** — Google Document AI has pre-built invoice, receipt, ID, and contract processors. RERP starts with generic models that need training.
- **Production model accuracy** — ABBYY's CNN classifiers have been trained on billions of documents. RERP's initial models will have lower accuracy until trained on real data.
- **Form-specific classification** — Google and Azure have form-specific processors. RERP needs to build form detection as a separate capability.
- **Enterprise SLA** — Google, Azure, and AWS offer 99.9%+ uptime SLAs. RERP is self-hosted and depends on infrastructure reliability.

---

## Competitive Intelligence Deep Dive

### ABBYY CNN Classifiers

ABBYY's CNN classifiers are trained on deep neural networks and are considered the gold standard for document type classification. They achieve 97-99% accuracy on well-defined document categories. Key advantage: trained on billions of real-world documents over 30+ years. Disadvantage: proprietary, requires ABBYY software stack, no zero-shot capability. Pricing: included in ABBYY Vantage ($5,000-50,000/server). **For RERP: Must train comparable CNN models on organization-specific document sets. Target: 90%+ accuracy on first pass, 95%+ after 1,000 labeled samples.**

### Google Document AI Processors

Google's processors are pre-trained: Invoice Processor, Receipt Processor, ID Document Processor, Passport Processor, General Document. They require no training — just send a document and get back structured output. Pricing: $1.50/1,000 pages for General Document, $10-250/page for specialized processors. Key advantage: zero training. Key disadvantage: expensive for specialized processors, documents leave your environment. **For RERP: Zero-shot classification via LLMs is the RERP alternative to Google's pre-trained processors.**

### Azure Document Intelligence Models

Azure offers prebuilt models for Invoice, Receipt, Business Card, ID Document, and Document. Custom models require 5-10 labeled samples per field and 20-30 sample documents per document type. Pricing: $5/1,000 pages for prebuilt, $8.75-87.50/page for custom. Key advantage: prebuilt models work out of the box. Disadvantage: custom models require labeled data. **For RERP: RERP's zero-shot classification eliminates the labeled data requirement.**

### Rossum's AI Classification

Rossum uses "No Training Required" AI classification. It uses a combination of template analysis, key-value pattern matching, and ML to classify documents automatically. Claimed 90%+ auto-classification rate on the first document. Key advantage: genuinely requires no manual training. Key disadvantage: cloud-only, $4,000-10,000/year subscription. **For RERP: RERP's zero-shot classification is the open-source equivalent of Rossum's "no training" claim.**

### Nanonets Classification

Nanonets requires 50-100 labeled training samples per document class. Training takes 5-15 minutes. Accuracy: 93-96% after training. Pricing: $0.30/1,000 documents for classification, $49-99/month subscription. Key advantage: fast training, good accuracy. Key disadvantage: requires labeled samples, per-document pricing. **For RERP: Zero-shot classification vs. 50+ samples per class is the key differentiator.**

---

## Implementation Roadmap

### Phase 1: Core Classification (Weeks 1-4) — P1

1. Define `DocumentClass`, `ClassificationRule`, `ClassificationModel`, `DocumentClassification` entities in OpenAPI spec
2. Implement primitive document classes (Invoice, Contract, PO, Receipt, Resume, Form, Letter, ID Document)
3. Build rule-based classification engine with condition matching (sender, content-type, filename pattern, text match)
4. Implement confidence scoring (0-100%) with ranked alternatives
5. Implement low-confidence review queue (< 70% confidence)
6. Define classification result model with multi-label support
7. Generate Rust server stubs from OpenAPI spec

### Phase 2: ML Classification Models (Weeks 5-8) — P1

1. Integrate lightweight CNN classifier (MobileNet-based or DistilBERT-based for text documents)
2. Implement model training pipeline (feed labeled classified documents)
3. Implement custom category training (add new classes without retraining entire model)
4. Add model accuracy tracking and drift detection
5. Implement model versioning (A/B testing between model versions)
6. Add classification history and accuracy reporting

### Phase 3: Zero-Shot LLM Classification (Weeks 9-12) — P1

1. Integrate LLM-based zero-shot classifier (local LLM via vLLM or external API)
2. Implement natural language class definition (describe document type → create classification)
3. Implement zero-shot classification endpoint (send document text → get classification)
4. Implement hybrid classification (ML model + zero-shot LLM → weighted result)
5. Add confidence calibration (map LLM logits to calibrated confidence scores)
6. Benchmark zero-shot accuracy against pre-trained models

### Phase 4: Advanced Features (Weeks 13-16) — P1

1. Implement multi-label classification (document belongs to multiple categories)
2. Add hierarchical classification (parent → child class prediction)
3. Implement automated retraining trigger (when accuracy drops below threshold)
4. Add classification export (export labeled dataset for model improvement)
5. Build classification dashboard (accuracy trends, class distribution, review queue)
6. Add integration tests with known document benchmarks

---

## Key Takeaway for Buyers

RERP's Document Classification offers a fundamentally different approach than competitors: **zero-shot LLM classification** means you can classify any document type on day one — no training data, no pre-built processor, no vendor lock-in. While ABBYY requires you to train CNN classifiers on your documents, and Google/Azure charge per document for pre-built processors, RERP's zero-shot approach means your first document gets classified immediately.

The hybrid approach (ML + rules + zero-shot LLM) is the key differentiator. Rule-based classification handles 80% of routine documents (vendor emails, known filename patterns). ML models handle the remaining 20% with 90%+ accuracy. Zero-shot LLM handles everything else — new document types, custom forms, unexpected formats — with reasonable accuracy that improves over time as you collect labeled data.

**For buyers: If your document types change frequently (new vendors, new contract templates, new regulatory forms), RERP's zero-shot classification is the only solution that adapts without retraining. You describe the document type, and it works.**
