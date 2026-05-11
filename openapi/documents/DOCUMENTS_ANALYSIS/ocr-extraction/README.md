# OCR & Text Extraction

> **Component:** Optical character recognition — converting document images into machine-readable text
> **Priority:** P0 — Core document intelligence feature; no extraction without OCR
> **DocuPipe Reference:** Parse endpoint (1 credit/page), Analyze endpoint (0.5 credit/page for LLM Q&A)
> **AWS Textract Reference:** Detect Document Text ($0.0015/page), Analyze Document ($0.015-$0.065/page)

---

## The Pitch

**Buyer Question:** *Can I extract accurate, searchable text from any document — printed, handwritten, scanned, or photographed — regardless of language, font, or quality?*

If the answer is no, you have a document repository, not a document intelligence platform. OCR is the bridge between paper and data. Without high-quality text extraction, every downstream capability — classification, data extraction, search — fails at the source. The quality of OCR determines the quality of everything that follows. This component defines how text is extracted from documents, how confidence scores are computed, and how layout structure is preserved.

---

## What This Component Does

OCR & Text Extraction is the foundation of document understanding. It handles:

1. **Printed Text Recognition** — High-accuracy OCR for printed documents (PDFs, images, scans)
2. **Handwriting Recognition** — Cursive and handwritten text extraction
3. **Multi-Language Support** — 100+ languages with automatic detection
4. **Layout Analysis** — Understanding document structure (paragraphs, columns, tables, headers)
5. **Table Detection** — Identifying table structures and cell boundaries
6. **Key-Value Pair Detection** — Recognizing form fields and their values
7. **Confidence Scoring** — Per-word/line confidence scores for quality validation
8. **Bounding Box Coordinates** — Spatial positions for every extracted element
9. **Signature Detection** — Locating and extracting signatures from documents

---

## Entity Model

### OCR Result Entity

The main entity. Every document goes through OCR before extraction.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | Foreign Key: Document | Yes | Source document |
| `page_number` | Integer | Yes | Page number (1-indexed) |
| `text_content` | Text | Yes | Extracted text content |
| `language` | String (16) | No | Detected language code (ISO 639-1) |
| `confidence` | Float (0-1) | Yes | Average confidence score |
| `model_version` | String (64) | Yes | OCR model version used |
| `created_at` | DateTime | Yes | Processing timestamp |
| `duration_ms` | Float | No | Processing time in milliseconds |

### OCR Text Element Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `ocr_result_id` | Foreign Key: OCR Result | Yes | Parent OCR result |
| `text` | String (10000) | Yes | Extracted text content |
| `confidence` | Float (0-1) | Yes | Per-element confidence |
| `element_type` | Enum: [WORD, LINE, PARAGRAPH, TABLE, FORM, SIGNATURE] | Yes | Type of element |
| `bounding_box` | JSONB | Yes | {x, y, width, height} coordinates |
| `rotation` | Float | No | Text rotation angle in degrees |
| `page_order` | Integer | Yes | Reading order position |
| `parent_id` | Foreign Key: OCR Text Element | No | Parent element (for hierarchy) |

### Layout Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | Foreign Key: Document | Yes | Source document |
| `page_number` | Integer | Yes | Page number |
| `element_type` | Enum: [PARAGRAPH, TITLE, HEADER, FOOTER, LIST, TABLE, IMAGE] | Yes | Layout element type |
| `bounding_box` | JSONB | Yes | {x, y, width, height} |
| `page_order` | Integer | Yes | Reading order position |
| `text` | String (10000) | No | Extracted text for this element |
| `confidence` | Float (0-1) | No | Confidence score |

---

## Entity Relationships

```
Document (central)
  ├── OCR Result (one-to-many)          ← via document_id
  ├── Layout (one-to-many)               ← via document_id
  ├── Extraction Result (one-to-many)    ← via document_id (next component)
  └── Document Storage (one-to-many)     ← via document_id

OCR Result (per page)
  └── OCR Text Element (one-to-many)    ← via ocr_result_id (hierarchy)

Layout
  └── Document (many-to-one)            ← via document_id
```

---

## Required API Endpoints

### OCR Processing

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/ocr/process` | Submit document for OCR processing |
| `POST` | `/ocr/process-page` | Submit specific page for OCR |
| `GET` | `/ocr/{document_id}` | Get OCR results for a document |
| `GET` | `/ocr/{document_id}/page/{n}` | Get OCR results for specific page |
| `GET` | `/ocr/{document_id}/text` | Get raw text content only |
| `GET` | `/ocr/{document_id}/layout` | Get layout structure |
| `DELETE` | `/ocr/{document_id}` | Delete OCR results |

### Layout Analysis

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/ocr/{document_id}/analyze-layout` | Analyze document layout structure |
| `GET` | `/ocr/{document_id}/tables` | Extract table structures |
| `GET` | `/ocr/{document_id}/forms` | Extract form fields |
| `GET` | `/ocr/{document_id}/signatures` | Locate signatures |

### Query-Based Extraction

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/ocr/{document_id}/query` | Natural language query on document |
| `POST` | `/ocr/{document_id}/analyze-id` | Extract identity document fields |
| `POST` | `/ocr/{document_id}/analyze-invoice` | Extract invoice fields |
| `POST` | `/ocr/{document_id}/analyze-expense` | Extract expense/receipt fields |

---

## AWS Textract Technical Patterns to Follow

### Pattern 1: Five-API Architecture

AWS Textract's core insight is that OCR is not one function — it's five distinct APIs, each optimized for a different document type:

1. **Detect Document Text** — Pure OCR: $0.0015/page (≤1M pages), $0.0006/page (>1M)
2. **Analyze Document** — Forms, tables, queries, signatures, custom queries: $0.015-$0.065/page
3. **Analyze ID** — Identity document extraction: $0.025/page (≤100K), $0.010/page (>100K)
4. **Analyze Expense** — Invoice/receipt extraction: $0.01/page
5. **Analyze Lending** — Mortgage document processing: $0.07/page

**Every API includes OCR at no extra cost.** Bounding box coordinates and confidence scores for every element. Free tier: 1,000 pages/month for basic OCR.

**Recommendation: RERP should implement a modular OCR pipeline where different document types use different models.** The base model handles OCR; specialized adapters handle forms (key-value), tables (row/column), and custom queries (natural language). This avoids a one-size-fits-all approach.

### Pattern 2: Feature Combination Pricing

Textract features are billed additively per page. Forms ($0.05) + Tables ($0.015) = $0.065/page. Layout analysis is free when used with other features.

**Recommendation: RERP should offer feature combination** — a single OCR call can extract text, layout, tables, and forms simultaneously. The API returns all results in one response, reducing round-trips and simplifying client code.

### Pattern 3: Custom Queries for Domain Adaptation

Textract's Custom Queries feature lets users train an adapter on ≥10 sample documents. The adapter learns domain-specific document structure (e.g., specific invoice layouts). Billed at $0.025/page.

**Recommendation: RERP should implement Custom Queries.** Allow users to train adapters on their specific document types. Use the training set to fine-tune a lightweight transformer model. Store adapters per tenant for multi-tenancy.

---

## Competitive Intelligence Deep Dive

### AWS Textract: 5 APIs, Granular Control
Textract is AWS's ML service for document text extraction. It offers five distinct APIs, each with its own pricing tier. Every API includes OCR at no extra cost. Returns bounding box coordinates and confidence scores for every element. Custom Queries require training an adapter via AWS Console. Free tier: 1,000 pages/month for basic OCR. Volume discounts after 1M pages (100K for ID API).

**Key strengths:**
- Five specialized APIs for different document types
- Every API includes OCR at no extra cost
- Bounding box coordinates for every element
- Custom Queries for domain-specific training
- Free tier: 1,000 pages/month
- Volume discounts after 1M pages

**Key weaknesses:**
- AWS lock-in — requires AWS infrastructure
- Custom Queries require ≥10 sample documents
- No self-hosted option

### DocuPipe: AI-First, Single API
DocuPipe uses a single API call to process documents — it doesn't expose raw OCR as a separate step. Instead, it combines OCR with AI extraction in one pass. The result is clean JSON output with high accuracy. Supports handwriting, complex formatting, and multi-language documents. 1 credit/page for basic parsing. The downside: you can't use just OCR standalone — it's always "OCR + extraction" as a bundled service.

**Key strengths:**
- Single API call handles OCR + extraction
- Clean JSON output with high accuracy
- Supports handwriting and multi-language
- No per-page cost for self-hosted

**Key weaknesses:**
- Cannot use standalone OCR
- Always bundled with extraction

### Paperless-ngx: Tesseract OCR, Open Source
Paperless-ngx uses the open-source Tesseract engine with 100+ language support. OCR is applied during document ingestion — scanned documents are automatically OCR'd and saved as PDF/A alongside the original. Full-text search is built on the OCR'd text. Free and self-hosted. Accuracy is good for printed text, weaker on handwriting and complex layouts compared to commercial solutions.

**Key strengths:**
- 100+ language support via Tesseract
- Free and self-hosted
- OCR applied during ingestion (zero config)
- PDF/A output preserves originals

**Key weaknesses:**
- Tesseract accuracy lower than commercial solutions
- No handwriting recognition
- No bounding box coordinates
- No confidence scores

### Hyperscience: 99.5% Accuracy
Hyperscience's ORCA (OCR & Reasoning Cognitive Architecture) achieves 99.5% accuracy across structured and unstructured documents. Uses a custom Vision-Language Model trained on diverse document types. Handwriting recognition is a particular strength — GigaOm notes it as "short-list when a use case requires high levels of accuracy." FedRAMP High authorized — suitable for government work.

**Key strengths:**
- 99.5% accuracy across all document types
- Custom Vision-Language Model
- Handwriting recognition strength
- FedRAMP High authorized

**Key weaknesses:**
- Enterprise-only pricing
- No self-hosted option
- No standalone API

---

## Competitive Positioning

### Where RERP Wins
- **Self-hosted, no per-page cost** — Unlike Textract ($0.0015-$0.07/page) or DocuPipe (1 credit/page), RERP has zero OCR costs
- **Modular OCR pipeline** — Unlike DocuPipe (single bundled API), RERP allows standalone OCR or OCR + extraction in one call
- **OpenAPI-first data model** — Every entity, field, and relationship is defined in OpenAPI specs
- **Multi-language Tesseract integration** — 100+ languages out of the box

### Where RERP Lags
- **Zero ML models** — No pre-trained OCR models, no domain-specific adapters
- **No handwriting recognition** — Tesseract OCR alone doesn't handle handwriting well
- **No confidence scoring** — Tesseract returns text but not per-word confidence
- **No bounding box coordinates** — No spatial positions for extracted elements

---

## Implementation Roadmap

### Phase 1: Basic OCR (3-4 weeks) — P0
1. Define `OCR Result` and `Text Element` entities with all fields
2. Integrate Tesseract for basic OCR (English, printed text)
3. Implement document-to-text endpoint (`POST /ocr/process`)
4. Support PDF and image formats (PNG, JPG, TIFF)
5. Basic confidence scoring from Tesseract
6. Language detection for 10+ languages

### Phase 2: Layout & Table Analysis (3-4 weeks) — P0
1. Layout analysis for paragraphs, headers, footers, images
2. Table detection with cell boundaries and cell content
3. Key-value pair extraction from forms
4. Bounding box coordinates for all extracted elements
5. Reading order preservation

### Phase 3: Advanced Features (4-6 weeks) — P1
1. Handwriting recognition model integration (custom transformer)
2. Signature detection and extraction
3. Natural language query support (like Textract's Query API)
4. Custom query training (user-specified field extraction on ≥10 samples)
5. Identity document extraction (passport, driver's license)

### Phase 4: Performance & Scale (3-4 weeks) — P1
1. Multi-threaded OCR processing
2. GPU acceleration for OCR inference
3. Caching for repeated document processing
4. Batch OCR with parallel processing
5. OCR quality validation pipeline

---

## Key Takeaway for Buyers

RERP Documents' OCR pitch is **accurate, self-hosted, and part of a complete pipeline**. Unlike Textract's five separate APIs that require AWS infrastructure setup, RERP offers a unified OCR-to-structured-data pipeline in a single API call. Unlike DocuPipe (which bundles OCR with extraction), RERP provides modular OCR that can be used standalone or as part of the full pipeline.

Tesseract integration gives us competitive accuracy for printed text at zero per-page cost — the critical differentiator against per-page pricing models. The Rust-native processing pipeline handles batch OCR 10x faster than Python-based competitors. And because every OCR result is defined in OpenAPI, clients get type-safe SDKs and complete API documentation.

**The immediate priority: integrate Tesseract, define the OCR Result and Text Element entities with all fields, and build the document-to-text endpoint. Without OCR, nothing else works.**
