# Documents — Cross-Suite Document Implementation Audit

> **Date:** 2026-05-12
> **Scope:** All RERP suite README.md files under `openapi/{suite}/{service}/`
> **Objective:** Identify every README that discusses implementing document processing features which should instead reference or depend on the Documents suite as the single source of truth.

---

## Findings Summary

| # | Suite | Service | File | Severity | Document Feature Discussed | Action |
|---|-------|---------|------|----------|---------------------------|--------|
| 1 | accounting | — | `accounting/README.md` | **CRITICAL** | Slice 5: Document automation (upload, classification, OCR/extraction, review, approval, linkage) | Must reference documents suite |
| 2 | accounting | documents-extraction | `accounting/documents-extraction/README.md` | **CRITICAL** | Full document automation hub (intake, classification, extraction, review, linkage) | Must be removed or delegated to documents suite |
| 3 | ai | document | `ai/document/README.md` | **CRITICAL** | Document AI service (extraction, OCR, classification, invoice processing) | Must reference documents suite |
| 4 | accounting | edi | `accounting/edi/README.md` | **MEDIUM** | "AI Document Extraction" as EDI capability | Must reference documents suite |
| 5 | hr | core | `hr/core/README.md` | **LOW** | Employee document management (contracts, certificates, IDs) | May be separate concern — document STORAGE, not processing |
| 6 | website | cms | `website/cms/README.md` | **LOW** | Media library with documents | Generic file storage — not document processing |

---

## Detailed Findings

### 1. CRITICAL — `accounting/README.md` — Slice 5: "Document Automation"

**Location:** Lines 240-260 (Slice 5 section)

**What it says:**
```markdown
### Slice 5: Document Automation

Goal: reduce manual accounting entry and improve source-document traceability.

Scaffold and implement:

1. `documents-extraction`
2. `invoice`
3. `accounts-payable`
4. `accounts-receivable`
5. `edi`

Deliver:

- Accounting document upload, classification, OCR/extraction jobs, confidence scoring, review, approval, and linkage.
- Conversion of reviewed documents into invoices, vendor bills, bank statement support, or EDI attachments.

Visible value:

- Users can turn documents into accounting records with review and audit traceability.
- This is a strong differentiator once core posting and reconciliation already work.
```

**Problem:** This describes implementing the FULL document processing pipeline (upload, classification, OCR/extraction, confidence scoring, review, approval, linkage) as part of the accounting suite's own implementation plan. This is exactly what the Documents suite is supposed to provide.

**Correction required:** Replace this entire section with a reference to the Documents suite. The accounting suite should:
- Consume document processing APIs from the Documents suite (ingest, classify, extract, review)
- Link reviewed document data to accounting records (invoices, bills, statements)
- NOT implement document processing itself

**Fix approach:**
```markdown
### Slice 5: Document Automation

Goal: reduce manual accounting entry and improve source-document traceability.

Scaffold and implement:

1. `documents-extraction` (delegated to documents suite)
2. `invoice`
3. `accounts-payable`
4. `accounts-receivable`
5. `edi`

Deliver:

- Integrate with the Documents suite for document upload, classification, OCR/extraction, confidence scoring, and review workflows.
- Link reviewed document data to invoices, vendor bills, bank statement support, and EDI records via the Documents suite API.
- Conversion of reviewed document fields into accounting records.

Visible value:

- Users can turn documents into accounting records via the centralized Documents suite, with review and audit traceability.
- This is a strong differentiator once core posting and reconciliation already work.
```

---

### 2. CRITICAL — `accounting/documents-extraction/README.md` — Duplicate documents suite

**Location:** Full file (61 lines)

**What it says:**
```markdown
# Documents Extraction

## What It Is

The Documents Extraction service is your accounting document automation hub — the system that ingests source documents, classifies them, extracts structured accounting data, routes uncertain results for human review, and links approved documents to invoices, bills, bank statements, and EDI records.
```

**Problem:** This is a FULL DESCRIPTION of document automation — ingestion, classification, extraction, review, linkage. This is the DOCUMENTS SUITE, not an accounting-specific service. It duplicates the Documents suite's purpose under the accounting suite.

**Problem with accounting/documents-extraction in microservices/ layout:**
There's also `microservices/accounting/documents-extraction/` with `gen/` and `impl/` — this is a **duplicate microservice** that should be removed and consolidated under `microservices/documents/`.

**Correction required:** This service is a duplicate of the Documents suite. It should be:
1. **Removed** from the accounting suite
2. **Consolidated** into the Documents suite (`openapi/documents/` + `microservices/documents/`)
3. The accounting suite should **consume** this via API calls to the Documents suite

**Fix approach:**
```markdown
# [REMOVED — This service is now part of the Documents suite]

The Documents Extraction service was moved to the Documents suite to eliminate duplication.
Accounting consumes document processing via the Documents suite API.

See: [Documents Suite README](../documents/README.md)
```

**Action items:**
- Remove `microservices/accounting/documents-extraction/` (both gen/ and impl/)
- Remove `openapi/accounting/documents-extraction/README.md`
- Update accounting suite's `bff-suite-config.yaml` to remove this service
- Update accounting suite's `openapi_bff.yaml` to remove this service
- The accounting suite should instead call the Documents suite's `/documents/` API endpoints

---

### 3. CRITICAL — `ai/document/README.md` — Duplicate document AI service

**Location:** Full file (64 lines)

**What it says:**
```markdown
# Document AI & Intelligent Extraction

## What It Is

The Document AI service is your business's intelligent document processor — the system that uses machine learning to extract data from documents, process invoices, and classify content automatically.
```

**Problem:** This describes a FULL document AI service with extraction, OCR, classification, invoice processing, learning capabilities. This is a DUPLICATE of the Documents suite's purpose under the AI suite.

**Problem with AI/document in microservices/ layout:**
There's also `microservices/ai/document/` — this should not exist. The AI suite should NOT have its own document service.

**Correction required:** This service duplicates the Documents suite. It should be:
1. **Removed** from the AI suite
2. **Consolidated** into the Documents suite
3. The AI suite should **consume** document processing via the Documents suite API

**Fix approach:**
```markdown
# [REMOVED — Document AI is now part of the Documents suite]

The Document AI service was moved to the Documents suite to eliminate duplication.
The AI suite consumes document processing via the Documents suite API.

See: [Documents Suite README](../documents/README.md)
```

**Action items:**
- Remove `microservices/ai/document/` (both gen/ and impl/)
- Remove `openapi/ai/document/README.md`
- Update AI suite's `bff-suite-config.yaml` to remove this service
- Update AI suite's `openapi_bff.yaml` to remove this service
- The AI suite should instead call the Documents suite's `/documents/` API endpoints

---

### 4. MEDIUM — `accounting/edi/README.md` — "AI Document Extraction" as EDI capability

**Location:** Line 50

**What it says:**
```markdown
- **AI Document Extraction**: AI-powered extraction of data from paper invoices and documents
```

**Problem:** This lists "AI Document Extraction" as a key capability of the EDI service. Document extraction is the Documents suite's responsibility, not EDI's. The EDI service should reference the Documents suite for extraction.

**Correction required:** Replace "AI Document Extraction" with "Document Integration (via Documents suite)" and add a note about integration.

**Fix approach:**
```markdown
- **Document Integration**: Integration with the Documents suite for AI-powered extraction from paper invoices and documents
```

---

### 5. LOW — `hr/core/README.md` — Employee document management

**Location:** Lines 15, 30-31, 42, 50

**What it says:**
```markdown
- **Document Chaos**: Employee documents (contracts, certificates) stored in filing cabinets
### 📄 **Digital Document Management**
Store and manage employee documents digitally. Contracts, certificates, IDs—all searchable, all secure, all accessible.
- **Employee Documents**: Store and manage employee documents (contracts, certificates, IDs) with version control
```

**Analysis:** This describes **storing employee documents** (contracts, certificates, IDs) — which is document STORAGE, not document PROCESSING. The Documents suite handles document ingestion, extraction, classification, and analysis. Employee document storage is a different concern.

**Recommendation:** This is likely OK as-is, but should be clarified:
- If HR Core is just storing/retrieving documents (like a file store), it should use the Documents suite for storage
- If HR Core is doing document processing (OCR, extraction) on employee documents, it should reference the Documents suite

**No immediate correction needed** — but worth noting for future architecture decisions.

---

### 6. LOW — `website/cms/README.md` — Media library with documents

**Location:** Line 48

**What it says:**
```markdown
- **Media Library**: Manage media assets (images, videos, documents) with organization and search
```

**Analysis:** This is generic media/file storage in a CMS context. Not document processing. Likely OK as-is.

---

## Cross-Suite Document References (Minor)

| File | Mention | Assessment |
|------|---------|------------|
| `ai/README.md` | "Document AI service using machine learning for document extraction" | References `ai/document/` which is a duplicate — see #3 above |
| `accounting/README.md` | "electronic document/payment handoffs" | Refers to EDI, not document processing |
| `accounting/README.md` | "Accounting document upload, classification, OCR/extraction" | Slice 5 — see #1 above (CRITICAL) |

---

## Consolidated Action Plan

### P0 (Blockers)

| # | Action | File(s) | Effort |
|---|--------|---------|--------|
| 1 | Remove `microservices/accounting/documents-extraction/` and `openapi/accounting/documents-extraction/` | accounting suite | Medium — requires BFF config updates |
| 2 | Remove `microservices/ai/document/` and `openapi/ai/document/` | AI suite | Medium — requires BFF config updates |
| 3 | Consolidate documents-extraction into `microservices/documents/` | documents suite | High — requires new service implementation |
| 4 | Update `accounting/README.md` Slice 5 to reference documents suite | accounting suite | Low |
| 5 | Update `accounting/README.md` service inventory to remove documents-extraction | accounting suite | Low |

### P1 (Cleanup)

| # | Action | File(s) | Effort |
|---|--------|---------|--------|
| 6 | Update `accounting/edi/README.md` — replace "AI Document Extraction" with "Document Integration" | edi suite | Low |
| 7 | Update `ai/README.md` — remove reference to ai/document service | ai suite | Low |

### P2 (Review)

| # | Action | File(s) | Effort |
|---|--------|---------|--------|
| 8 | Review `hr/core/README.md` — clarify if document management is storage or processing | hr suite | Low |
| 9 | Review `website/cms/README.md` — clarify media library vs document processing | website suite | Low |

---

## Current State: Where Document Logic Currently Lives

```
accounting/documents-extraction/    ← DUPLICATE of Documents suite
  ├── gen/                          ← Should be in documents suite
  └── impl/                         ← Should be in documents suite

ai/document/                        ← DUPLICATE of Documents suite
  ├── gen/                          ← Should be in documents suite
  └── impl/                         ← Should be in documents suite

documents/                          ← THE REAL Documents suite (early stage)
  ├── gen/                          ← Only partial (core handlers only)
  └── impl/                         ← Empty — no implementation yet
```

**All document processing logic should flow through `documents/`.** The `accounting/documents-extraction/` and `ai/document/` services should be deleted, and their functionality merged into the Documents suite.

---

## Integration Points After Correction

Once the duplicates are removed and consolidated:

```
┌─────────────────────────────────────────────────────┐
│                    Documents Suite                    │
│  ┌─────────┐ ┌────────┐ ┌──────────┐ ┌───────────┐ │
│  │  Intake │ │   OCR  │ │ Extraction│ │ Classification│
│  └─────────┘ └────────┘ └──────────┘ └───────────┘ │
└──────────┬──────────────────────────────────────────┘
           │ API calls
┌──────────┴────┐ ┌────────────┐ ┌──────────┐ ┌──────────┐
│  Accounting   │ │    AI      │ │   HR     │ │   Website│
│  (invoice)    │ │ (extraction│ │(doc store│ │ (media   │
│               │ │  via docs) │ │  maybe)  │ │  library)│
└───────────────┘ └────────────┘ └──────────┘ └──────────┘
```

**Accounting suite** — consumes Documents suite for invoice/bill extraction
**AI suite** — consumes Documents suite for ML-assisted extraction (or delegates to Documents)
**HR suite** — may consume Documents suite for employee document storage (or keeps its own)
**Website CMS** — uses media library, not document processing
