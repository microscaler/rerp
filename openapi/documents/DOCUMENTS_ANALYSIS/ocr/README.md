# OCR & Text Recognition

> **Component:** Optical Character Recognition engine — convert scanned documents, images, and PDFs into machine-readable text with structure awareness
> **Priority:** P0 — Core capability; the base layer upon which classification and extraction depend
> **Reference Competitors:** Tesseract, ABBYY FineReader (ICR), AWS Textract OCR, Google Document AI Layout Parser, Azure Read API, PaddleOCR

---

## The Pitch

**Buyer Question:** *Can I take a scanned PDF, a photograph of a receipt, a handwritten form, or a multi-page contract — and reliably get back structured, searchable text with tables, signatures, and regions identified?*

OCR is the gateway from physical (or image-based) documents to the digital world. Get this wrong, and every downstream capability — classification, extraction, analysis — fails. The question isn't whether you need OCR; it's whether you need it self-hosted (no data egress, no per-page fees, no vendor lock-in) or dependent on a cloud API that charges per page and sends your documents to their servers. RERP's OCR engine supports both: self-hosted Tesseract/PaddleOCR for cost-sensitive workloads, and cloud OCR APIs as fallback for edge cases.

---

## What This Component Does

OCR & Text Recognition transforms visual document content into machine-readable structured text. It is the first processing step after ingestion:

1. **Printed Text OCR** — Convert printed text from scanned documents, PDFs, and images using Tesseract 5.x (LSTM neural network) or PaddleOCR for superior CJK (Chinese/Japanese/Korean) recognition
2. **Handwritten Text Recognition (ICR)** — Handwritten text support via custom models (Tesseract HTR, PaddleOCR hand writing model, or integration with cloud APIs for high-accuracy handwriting recognition)
3. **Image Preprocessing** — Automatic deskew (correct rotation), deskew detection, noise removal (denoise, despeckle), binarization (Otsu thresholding), contrast enhancement, and page orientation detection before OCR
4. **Multi-Format Input** — Accept PDF (scanned or native), PNG, JPEG, TIFF (single and multi-page), BMP, WebP, and GIF. PDFs can be either image-based (scanned) or text-based (pass-through)
5. **Table Structure Detection** — Identify and reconstruct table boundaries, cell structure, headers, and row/column spans. Output includes cell coordinates and content mapping
6. **Signature Detection** — Detect signature regions in documents with bounding boxes and confidence scores. Signatures can be flagged for separate review or stored as images for downstream processing
7. **Multi-Language Support** — Tesseract supports 100+ languages. RERP configuration allows per-document or per-source language specification. Auto-detection via language identification models for unassigned documents
8. **Layout Analysis** — Detect and tag text regions (paragraphs, headers, footers, margins), column layout (single vs. multi-column), and reading order for accurate text reconstruction

---

## Entity Model

### OCRJob

Represents a single OCR processing request.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `ingest_job_id` | UUID | Yes | Foreign key to DocumentIngestJob; the document being processed |
| `status` | Enum: [QUEUED, PREPROCESSING, PROCESSING, COMPLETED, FAILED, PARTIAL] | Yes | Processing lifecycle state |
| `ocr_engine` | Enum: [TESSERACT, PADDLE_OCR, AWS_TEXTRACT, GOOGLE_DOC_AI, AZURE_READ] | Yes | Engine used for processing |
| `language_code` | String (16) | Yes | BCP-47 language code (e.g., en, fr, de, ja, zh-Hans) |
| `page_count` | Int32 | Yes | Total pages to process |
| `pages_processed` | Int32 | No | Pages successfully processed |
| `image_preprocessing` | JSONB | No | Applied preprocessing steps (deskew, denoise, threshold) |
| `confidence_avg` | Float (0-100) | No | Average OCR confidence across all pages |
| `confidence_min` | Float (0-100) | No | Minimum confidence on any page (for quality gating) |
| `processing_time_ms` | Float | No | Total processing time in milliseconds |
| `error_details` | JSONB | No | Error message and stack trace if failed |
| `cloud_cost_usd` | Float | No | Cost if using cloud API (0.0 for self-hosted) |
| `created_at` | DateTime | Yes | Auto-set on creation |
| `updated_at` | DateTime | Yes | Auto-updated on status change |
| `completed_at` | DateTime | No | When all pages processed or job failed |

### OCRResult

Stores the output of a completed OCR job — one result per page.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `ocr_job_id` | UUID | Yes | Parent OCR job |
| `page_number` | Int32 | Yes | Page number (1-indexed) |
| `full_text` | Text | Yes | Complete OCR text for the page |
| `confidence_avg` | Float (0-100) | No | Average confidence for this page |
| `image_url` | String (1024) | No | URL/path to the preprocessed page image |
| `image_base64` | Text | No | Base64-encoded page image (optional) |
| `created_at` | DateTime | Yes | Auto-set on creation |

### OCRError

Captures per-page or per-region OCR failures.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `ocr_job_id` | UUID | Yes | Parent OCR job |
| `page_number` | Int32 | No | Affected page number (nullable for job-level errors) |
| `error_code` | String (64) | Yes | Machine-readable error code |
| `error_message` | String (1024) | Yes | Human-readable error description |
| `error_context` | JSONB | No | Additional context (image dimensions, preprocessing applied) |
| `retry_count` | Int32 | No | Number of retry attempts |
| `created_at` | DateTime | Yes | Auto-set on creation |

### TextRegion

Identified text blocks within an OCR result page.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `ocr_result_id` | UUID | Yes | Parent OCR result (page) |
| `region_type` | Enum: [PARAGRAPH, HEADER, FOOTER, TITLE, LABEL, VALUE, MARGIN] | Yes | Semantic type of text region |
| `bounding_box` | JSONB | Yes | `{x, y, width, height}` in normalized coordinates (0-1) |
| `text_content` | Text | Yes | Text content within the region |
| `confidence` | Float (0-100) | No | Confidence for this region |
| `line_count` | Int32 | No | Number of lines in the region |
| `reading_order` | Int32 | No | Position in reading order sequence |

### TableRegion

Identified table structures within an OCR result page.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `ocr_result_id` | UUID | Yes | Parent OCR result (page) |
| `bounding_box` | JSONB | Yes | `{x, y, width, height}` normalized |
| `row_count` | Int32 | Yes | Number of rows detected |
| `column_count` | Int32 | Yes | Number of columns detected |
| `has_header` | Boolean | No | Whether first row is a header |
| `cell_data` | JSONB | Yes | 2D array of cell contents with merged cell info |
| `confidence` | Float (0-100) | No | Confidence in table structure |
| `reading_order` | Int32 | No | Position in reading order |

### SignatureRegion

Detected signature areas on a document page.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `ocr_result_id` | UUID | Yes | Parent OCR result (page) |
| `bounding_box` | JSONB | Yes | `{x, y, width, height}` normalized |
| `signature_type` | Enum: [HANDWRITTEN, DIGITAL_CERTIFICATE, STAMP, INITIALS, UNKNOWN] | Yes | Detected signature type |
| `confidence` | Float (0-100) | No | Confidence that a signature exists in this region |
| `is_signed` | Boolean | No | Whether the document is signed in this region |
| `created_at` | DateTime | Yes | Auto-set on creation |

---

## Entity Relationships

```
OCRJob
  ├── DocumentIngestJob (via ingest_job_id)          ← source document
  ├── OCRResult × N (one per page)                    ← per-page output
  ├── OCRError × N                                    ← per-page/job errors
  └── ClassificationModel (via routing)               ← dispatched after OCR

OCRResult
  ├── TextRegion × N                                  ← identified text blocks
  ├── TableRegion × N                                 ← identified table structures
  └── SignatureRegion × N                             ← identified signatures

OCRError
  └── OCRJob                                          ← error source

DocumentIngestJob
  └── OCRJob × N (1:1 relationship per job)           ← one OCR job per ingest job
```

---

## Required API Endpoints

### OCRJob Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/ocr/jobs` | List OCR jobs with status, engine, language filters |
| `GET` | `/ocr/jobs/{id}` | Get OCR job detail with per-page results |
| `POST` | `/ocr/jobs` | Submit a document for OCR processing |
| `DELETE` | `/ocr/jobs/{id}` | Cancel an OCR job (if not yet processing) |
| `GET` | `/ocr/jobs/{id}/status` | Poll job status (QUEUED → PROCESSING → COMPLETED/FAILED) |
| `POST` | `/ocr/jobs/{id}/retry` | Retry failed pages with different engine/language |

### OCR Results

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/ocr/jobs/{id}/pages` | List all page results for an OCR job |
| `GET` | `/ocr/jobs/{id}/pages/{page_num}` | Get full text + regions for a specific page |
| `GET` | `/ocr/jobs/{id}/pages/{page_num}/text` | Get raw text for a page |
| `GET` | `/ocr/jobs/{id}/pages/{page_num}/image` | Get preprocessed page image |
| `GET` | `/ocr/jobs/{id}/pages/{page_num}/tables` | Get detected table structures |
| `GET` | `/ocr/jobs/{id}/pages/{page_num}/signatures` | Get detected signature regions |

### OCR Engine Configuration

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/ocr/config` | Get current OCR engine configuration |
| `PATCH` | `/ocr/config` | Update default engine, language, preprocessing options |
| `GET` | `/ocr/engines` | List available engines (Tesseract, PaddleOCR, cloud APIs) |
| `POST` | `/ocr/config/test` | Test OCR engine with sample document |

### Batch OCR

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/ocr/jobs/batch` | Submit multiple documents for parallel OCR |
| `GET` | `/ocr/jobs/batch/{batch_id}` | Get batch OCR processing status |

---

## Competitive Positioning

### Where RERP Wins

- **Self-hosted with cloud fallback** — RERP runs Tesseract/PaddleOCR locally (free, no data egress) and falls back to cloud APIs only when needed. AWS Textract, Google Document AI, and Azure Read API are cloud-only — you must send your documents to their servers.
- **No per-page pricing** — Tesseract and PaddleOCR are free and open-source. AWS Textract charges $1.50/1,000 pages, Google Document AI charges $1.50/1,000 pages, Azure Read API charges $1-2/1,000 pages. For a company processing 1M pages/year: **$1,500-2,000/year in OCR fees** avoided.
- **Unified layout analysis** — Table detection, signature detection, and text region detection are built-in. Competitors often require separate API calls for layout analysis (Azure Read API separates text detection from table extraction).
- **Multi-engine orchestration** — RERP can try Tesseract first, then PaddleOCR for CJK, then fall back to a cloud API if confidence is below threshold. Competitors lock you into a single engine.
- **No vendor lock-in** — Switch engines, languages, or preprocessors without migrating data. Cloud APIs are tied to their ecosystem.

### Where RERP Lags

- **Handwriting accuracy** — ABBYY ICR has ~95% accuracy on printed handwriting, significantly above Tesseract's ~75%. RERP's ICR will start at Tesseract/PaddleOCR levels until custom models are trained.
- **Complex form recognition** — Google Document AI and Azure Form Recognizer excel at identifying form field types (name, date, amount) automatically. RERP must build form field detection from scratch.
- **Cloud API performance** — AWS Textract and Azure Read API have optimized GPU inference that processes pages in < 1 second. Self-hosted Tesseract on CPU takes 2-5 seconds per page.
- **Document AI processors** — Google Document AI and Azure Document Intelligence include pre-trained models for invoices, receipts, IDs, and passports. RERP starts with raw OCR only.
- **Enterprise support** — ABBYY and Google offer 24/7 support SLAs. RERP relies on community support (or paid support contracts if offered).

---

## Competitive Intelligence Deep Dive

### Tesseract (Google) — The Open-Source Workhorse

Tesseract 5.x uses LSTM neural networks and supports 100+ languages out of the box. It's the most widely deployed OCR engine (PowerPoint, Google Drive, Amazon Kindle, Apple's Live Text). Accuracy on clean printed text: 95-98%. On scanned documents: 85-92%. On handwritten text: 60-75%. Key limitation: it's a pure text recognizer — no table detection, no form field detection, no semantic region classification. RERP must build these on top. Tesseract is free and self-hosted. **Competitor pricing: $0.**

### ABBYY FineReader / ICR

ABBYY is the enterprise OCR gold standard. FineReader Desktop costs ~$200 one-time; ABBYY Cloud OCR costs $0.002-0.004 per page. FineReader's ICR (Intelligent Character Recognition) achieves 95%+ on printed handwriting and 98%+ on printed text. It includes built-in table reconstruction, form recognition, and export to Word/Excel/PowerPoint with layout preservation. ABBYY Vantage (enterprise platform) costs $5,000-50,000+ per server. **Competitor pricing: $0.002-0.004/page (cloud), $5,000-50,000/server (on-prem Vantage).**

### AWS Textract

Textract is AWS's serverless OCR with table and form extraction. It processes PDFs and images, detecting printed text, handwritten text, tables, and form key-value pairs. Pricing: $1.50/1,000 pages for text extraction, $3.00/1,000 pages for tables/forms. Processing time: < 1 second per page. **Competitor pricing: $1.50/1,000 pages ($1,500 for 1M pages).** Advantage: serverless, no infrastructure to manage. Disadvantage: documents leave your environment.

### Google Document AI

Google Document AI includes Layout Parser (text + structure), OCR (text), Invoice Parser, Receipt Parser, and ID Document Parser. Pricing: $1.50/1,000 pages for Layout Parser, $10-250/page for specialized parsers (ID, invoice, receipt). **Competitor pricing: $1.50-250/page depending on processor type.** Advantage: pre-trained specialized parsers. Disadvantage: expensive for specialized documents, vendor lock-in.

### Azure Read API (Azure AI Document Intelligence)

Azure Read API extracts printed and handwritten text with layout information. Pricing: $1-2/1,000 pages for basic read, $5/1,000 pages for prebuilt document models. **Competitor pricing: $1-5/1,000 pages.** Advantage: tight Office 365 integration. Disadvantage: requires Azure subscription.

### PaddleOCR

Baidu's PaddleOCR is a lightweight, high-performance OCR framework with excellent CJK (Chinese/Japanese/Korean) recognition — significantly better than Tesseract for East Asian languages. It includes text detection, text recognition, and PP-OCRv4 models. Accuracy on Chinese text: 97%+ (vs. Tesseract ~80%). Self-hosted and free. **Competitor pricing: $0.** Advantage: CJK superiority, lightweight deployment. Disadvantage: smaller community than Tesseract.

---

## Implementation Roadmap

### Phase 1: Core OCR Engine (Weeks 1-4) — P0

1. Define `OCRJob`, `OCRResult`, `OCRError` entities in OpenAPI spec
2. Integrate Tesseract 5.x OCR engine via Rust bindings (tesseract-rs or subprocess)
3. Implement single-page and multi-page OCR processing
4. Implement basic image preprocessing pipeline (deskew, binarization via OpenCV)
5. Configure language selection (default: English, configurable per job)
6. Implement result storage: full text + bounding boxes per line/word
7. Generate Rust server stubs from OpenAPI spec

### Phase 2: Structure Detection & Preprocessing (Weeks 5-8) — P0

1. Implement table structure detection (horizontal/vertical line detection, cell boundary inference)
2. Implement signature detection (contour analysis + template matching)
3. Add image preprocessing: denoise, despeckle, contrast enhancement, deskew
4. Add multi-column layout detection and reading order reconstruction
5. Implement confidence scoring per page and per word
6. Add page-level error handling and retry logic
7. Implement JSON output format for TextRegion, TableRegion, SignatureRegion

### Phase 3: Engine Expansion (Weeks 9-12) — P1

1. Add PaddleOCR engine integration for CJK language support
2. Add AWS Textract cloud API integration as fallback
3. Add Google Document AI Layout Parser as fallback
4. Add Azure Read API as fallback
5. Implement engine selection strategy (preferred → fallback → last-resort)
6. Implement confidence threshold gating (auto-fallback when confidence < threshold)
7. Add handwriting recognition via Tesseract HTR or cloud fallback

### Phase 4: Advanced Features (Weeks 13-16) — P1

1. Implement custom language model training pipeline
2. Add document-specific preprocessing profiles (receipts, forms, contracts, IDs)
3. Implement parallel page processing for multi-page documents
4. Add OCR result caching (hash-based — skip re-OCR for identical pages)
5. Implement quality score dashboard (avg confidence, failure rate by engine/language)
6. Add integration tests against known document benchmarks

---

## Key Takeaway for Buyers

RERP's OCR engine gives you the best of both worlds: **self-hosted, free OCR via Tesseract/PaddleOCR** for cost-sensitive, compliance-sensitive workloads, with **cloud API fallback** for edge cases where ABBYY-level accuracy is needed. No per-page fees, no data egress, no vendor lock-in.

The competitive landscape for OCR is split: cloud APIs (AWS Textract, Google Document AI, Azure Read API) are convenient but expensive ($1,500-2,500/year for 1M pages) and require data to leave your environment. On-premise solutions (ABBYY Vantage, Kofax) are powerful but expensive ($50,000-300,000 TCO over 3 years) and rigid.

RERP sits in the middle: open-source engines with cloud fallback, self-hosted by default but flexible enough to use cloud APIs when they're the better tool for the job. For a buyer processing 500K-1M pages/year, that's $750-2,500 in annual savings compared to cloud-only solutions — plus the strategic advantage of never having to send sensitive documents to a third party.

**For buyers: If your documents contain PII, PHI, financial data, or trade secrets, self-hosted OCR isn't a nice-to-have — it's a compliance requirement. RERP delivers enterprise-grade OCR without the enterprise price tag or the data compromise.**
