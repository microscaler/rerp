# PRD-004: Competitive Intelligence Enrichment

## Meta

- **Status:** Draft
- **Author:** Engineering Design
- **Created:** 2026-05-11
- **Related:** README.md (competitive landscape), all component READMEs
- **Priority:** P2 — Strengthens market positioning
- **Blocks:** Sales collateral, product launch materials

## Problem

The competitive intelligence in the README.md and component READMEs is **incomplete** and has **mischaracterizations**:

1. **Missing competitors:** 5 major players are absent:
   - **ABBYY Vantage** — Enterprise document intelligence ($15k+/yr), OCR + extraction + workflow, strong in OCR accuracy
   - **Google Document AI** — Google's competitor to Textract, pre-trained parsers for invoices/receipts/IDs, $1.50/1000 pages for basic, $10/1000 for advanced
   - **UiPath Document Understanding** — RPA platform with document processing, strong in workflow automation, enterprise pricing
   - **Amazon Kendra** — AWS document search with ML, competes with search-discovery, not pure document processing
   - **Notion AI / Coda AI** — Emerging document processing via AI assistants, targeting knowledge work

2. **Mischaracterized DocuPipe:** Claimed as having "no standalone OCR" — it does. Parse endpoint OCRs any document and returns OCR + extraction in one call. The OCR IS standalone (you just can't use it without also getting extraction).

3. **Mischaracterized Paperless-ngx:** Claimed as having "no enterprise certifications" — this is irrelevant for an AGPLv3 open-source product. The enterprise certification bar doesn't apply to software the user runs themselves.

4. **Overclaimed performance metrics:** Every component claims "Rust-native processing handles 10,000+ concurrent operations with sub-millisecond latency." This is a marketing claim with no basis — the system doesn't exist yet.

5. **Duplicate positioning:** Every component's "Where RERP Wins" says "Self-hosted, no per-page cost" and "OpenAPI-first design." True but repetitive — the positioning should be unique per component.

## Solution

### 1. Add Missing Competitors

Add the following to the competitive landscape README.md and relevant component READMEs:

#### ABBYY Vantage

**Position:** Enterprise document capture and intelligent automation
- **Pricing:** Enterprise, $15,000+/year minimum
- **Strengths:** Best-in-class OCR accuracy (99%+), 1,000+ document templates, deep SAP/Oracle integration, FedRAMP authorized
- **Weaknesses:** Enterprise-only, expensive, Windows-heavy deployment
- **RERP Advantage:** Self-hosted, zero per-page costs, Linux-native deployment, OpenAPI-first

#### Google Document AI

**Position:** Cloud-based document understanding with pre-trained parsers
- **Pricing:** Pay-per-page. Basic OCR: $1.50/1000 pages. Invoice parser: $10/1000 pages. Receipt parser: $10/1000 pages. ID parser: $10/1000 pages. Custom parsers: $50/setup + $10/1000 pages
- **Free tier:** 500 pages/month
- **Strengths:** Pre-trained parsers (invoice, receipt, ID, passport, tax documents), Google Cloud integration, auto-generates extraction schemas from training data
- **Weaknesses:** Google Cloud lock-in, per-page pricing, no self-hosted, limited customization for custom document types
- **RERP Advantage:** Self-hosted, zero per-page costs, custom schema definition, no cloud lock-in

#### UiPath Document Understanding

**Position:** RPA platform with document processing capabilities
- **Pricing:** Part of UiPath Automation Cloud, $1,000+/month minimum
- **Strengths:** Deep RPA integration (automate the entire process from document ingestion to data entry), strong workflow automation, Microsoft ecosystem
- **Weaknesses:** RPA-focused (document processing is a module, not the core), enterprise pricing, Microsoft dependency
- **RERP Advantage:** Document-native (not RPA-dependent), self-hosted, zero licensing costs, OpenAPI-first

### 2. Correct Mischaracterizations

**DocuPipe OCR:** Clarify that Parse endpoint DOES perform standalone OCR. The distinction is that OCR is not exposed as a separate step — it's always combined with extraction in one call. RERP can do standalone OCR (Tesseract) separate from extraction (LLM-based).

**Paperless-ngx Certifications:** Remove the "no enterprise certifications" claim as a competitive disadvantage. For open-source self-hosted software, certifications are irrelevant to the user — they run it themselves. Reframe: "Paperless-ngx is the only open-source competitor in the space, making it the closest philosophical match to RERP. But it lacks structured data extraction."

### 3. Remove Unverifiable Performance Claims

Replace these claims in ALL 10 component READMEs:

| Current Claim | Replacement |
|---------------|-------------|
| "Rust-native processing handles 10,000+ concurrent operations with sub-millisecond latency" | "Rust-native processing provides memory safety and zero-cost abstractions, enabling high-throughput document processing without GC pauses" |
| "10x faster than Python-based competitors" | "Rust-native processing avoids the GIL limitations and GC pauses of Python-based solutions" |
| "Handles 10,000+ concurrent uploads with sub-second response times" | "Rust's async runtime (Tokio) enables non-blocking I/O for high-concurrency document ingestion" |
| "Rust-native search engine handles 10,000+ concurrent searches" | "Rust-based search indexing provides deterministic performance without GC pauses" |
| "Rust-native workflow engine handles 10,000+ concurrent workflow executions" | "Rust-based workflow execution provides deterministic state management" |
| "Rust-native security model" | "Rust provides memory-safe encryption and access control implementation" |
| "Rust-native analytics engine processes millions of metric records" | "Rust-based analytics processing avoids GC pauses for real-time metric aggregation" |

### 4. Unique Per-Component Positioning

Rewrite "Where RERP Wins" sections to be component-specific:

| Component | Current Generic Claim | Component-Specific Positioning |
|-----------|----------------------|-------------------------------|
| document-ingestion | "Multi-source ingestion" | "Email, API, scan, and cloud storage connectors in one system — unlike DocuPipe (API-only)" |
| ocr-extraction | "Modular OCR pipeline" | "Standalone OCR separate from extraction — unlike DocuPipe which bundles them" |
| data-extraction | "Natural language extraction" | "Same natural language schema creation as DocuPipe but free and self-hosted" |
| classification | "Hierarchical classification" | "Document type hierarchies (Document → Invoice → Supplier Invoice) — unlike DocuPipe's flat types" |
| storage-management | "Multi-backend storage" | "Local + S3 + GCS + Azure Blob in one system — unlike Textract (S3-only)" |
| workflow-automation | "Blocks and Flows architecture" | "Visual workflow builder with branching, approvals, and conditional logic — unlike DocuPipe's linear API" |
| search-discovery | "Unified search" | "Searches both OCR text AND extracted fields in one query — unlike Textract (no search) or DocuPipe (metadata only)" |
| integration-api | "OpenAPI-first with auto-generated SDKs" | "Type-safe SDKs for TS/Python/Rust/Go from a single OpenAPI spec — unlike any competitor" |
| security-compliance | "Self-hosted, full data sovereignty" | "Complete data sovereignty — unlike cloud-only competitors (DocuPipe, Rossum, Hyperscience)" |
| reporting-analytics | "Business-term metrics" | "Metrics in business terms (time saved, accuracy, ROI) — unlike DocuPipe (credit consumption) or Textract (API calls)" |

## Acceptance Criteria

- [ ] ABBYY Vantage, Google Document AI, UiPath Document Understanding added to README.md competitive landscape
- [ ] DocuPipe OCR capability correctly characterized (it exists, just bundled with extraction)
- [ ] Paperless-ngx "no enterprise certifications" claim removed/ reframed
- [ ] All 7 unverifiable performance claims replaced with accurate Rust benefit statements across all 10 component READMEs
- [ ] All 10 "Where RERP Wins" sections rewritten with component-specific positioning
