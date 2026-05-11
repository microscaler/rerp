# OCR & Text Extraction

> **Component:** Optical character recognition — converting document images into machine-readable text
> **Priority:** P0 — Core document intelligence feature; no extraction without OCR

---

## The Pitch

**Buyer Question:** *Can I extract accurate, searchable text from any document — printed, handwritten, scanned, or photographed — regardless of language, font, or quality?*

If the answer is no, you have a document repository, not a document intelligence platform. OCR is the bridge between paper and data. Without high-quality text extraction, every downstream capability — classification, data extraction, search — fails at the source. The quality of OCR determines the quality of everything that follows.

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

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | FK: Document | Yes | Source document |
| `page_number` | Integer | Yes | Page number (1-indexed) |
| `text_content` | Text | Yes | Extracted text content |
| `language` | String (16) | No | Detected language code |
| `confidence` | Float (0-1) | Yes | Average confidence score |
| `model_version` | String (64) | Yes | OCR model version used |
| `created_at` | DateTime | Yes | Processing timestamp |

### OCR Text Element Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `ocr_result_id` | FK: OCR Result | Yes | Parent OCR result |
| `text` | String (10000) | Yes | Extracted text content |
| `confidence` | Float (0-1) | Yes | Per-element confidence |
| `element_type` | Enum: [WORD, LINE, PARAGRAPH, TABLE, FORM, SIGNATURE] | Yes | Type of element |
| `bounding_box` | JSONB | Yes | {x, y, width, height} coordinates |
| `rotation` | Float | No | Text rotation angle in degrees |
| `parent_id` | FK: OCR Text Element | No | Parent element (for hierarchy) |

### Layout Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | FK: Document | Yes | Source document |
| `page_number` | Integer | Yes | Page number |
| `element_type` | Enum: [PARAGRAPH, TITLE, HEADER, FOOTER, LIST, TABLE, IMAGE] | Yes | Layout element type |
| `bounding_box` | JSONB | Yes | {x, y, width, height} |
| `page_order` | Integer | Yes | Reading order position |

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

## Competitive Intelligence Deep Dive

### DocuPipe: AI-First OCR
DocuPipe uses a single API call to process documents — it doesn't expose raw OCR as a separate step. Instead, it combines OCR with AI extraction in one pass. The result is clean JSON output with high accuracy. Supports handwriting, complex formatting, and multi-language documents. 1 credit/page for basic parsing. The downside: you can't use just OCR standalone — it's always "OCR + extraction" as a bundled service.

### AWS Textract: 5 APIs, Granular Control
Textract offers five distinct APIs:
- **Detect Document Text** — Pure OCR ($0.0015/page)
- **Analyze Document** — Forms, tables, queries, signatures, custom queries ($0.015-$0.065/page)
- **Analyze ID** — Identity document extraction ($0.025/page)
- **Analyze Expense** — Invoice/receipt extraction ($0.01/page)
- **Analyze Lending** — Mortgage document processing ($0.07/page)

Every API includes OCR at no extra cost. Bounding box coordinates and confidence scores for every element. Custom Queries require training an adapter via AWS Console. Free tier: 1,000 pages/month for basic OCR.

### Rossum: Aurora Document AI
Rossum's proprietary Aurora engine is built in-house (no third-party LLM dependencies). Processes millions of annotated documents during training. Handles handwriting, complex layouts, and multi-language documents. Returns confidence scores and highlights uncertain extractions on the validation screen. Enterprise-grade: SOC 2, ISO 27001, HIPAA, TX-RAMP compliant.

### Adobe PDF Services: PDF-Optimized
PDF Extract API converts PDFs to structured data (JSON/Text). Free tier: 500 document transactions/month. Supports PDF/A conversion, accessibility auto-tagging, and document generation. Strong on PDF-specific operations but limited on image/document formats. The PDF Extract API is part of the broader Acrobat Services suite with 15+ PDF operations.

### Paperless-ngx: Tesseract OCR
Paperless-ngx uses the open-source Tesseract engine with 100+ language support. OCR is applied during document ingestion — scanned documents are automatically OCR'd and saved as PDF/A alongside the original. Full-text search is built on the OCR'd text. Free and self-hosted. Accuracy is good for printed text, weaker on handwriting and complex layouts compared to commercial solutions.

### Hyperscience: Vision-Language Model
Hyperscience's ORCA (OCR & Reasoning Cognitive Architecture) achieves 99.5% accuracy across structured and unstructured documents. Uses a custom Vision-Language Model trained on diverse document types. Handwriting recognition is a particular strength — GigaOm notes it as "short-list when a use case requires high levels of accuracy." FedRAMP High authorized — suitable for government work.

---

## Implementation Roadmap

### Phase 1: Basic OCR (3-4 weeks) — P0
1. Define OCR Result and Text Element entities
2. Integrate Tesseract for basic OCR
3. Implement document-to-text endpoint
4. Support PDF and image formats (PNG, JPG, TIFF)
5. Basic confidence scoring
6. Language detection (10+ languages)

### Phase 2: Layout & Table Analysis (3-4 weeks) — P0
1. Layout analysis for paragraphs, headers, footers
2. Table detection with cell boundaries
3. Key-value pair extraction from forms
4. Bounding box coordinates for all elements
5. Reading order preservation

### Phase 3: Advanced Features (4-6 weeks) — P1
1. Handwriting recognition model integration
2. Signature detection and extraction
3. Natural language query support (similar to Textract's Query API)
4. Custom query training (user-specified field extraction)
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

**The immediate priority: integrate Tesseract, define the OCR result schema, and build the document-to-text endpoint. Without OCR, nothing else works.**
