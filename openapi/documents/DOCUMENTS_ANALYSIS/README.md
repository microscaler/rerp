# Document Intelligence Competitive Analysis

> **Date:** 2026-05-11
> **Purpose:** Pitch-level competitive analysis for buyer decision-making
> **Scope:** RERP Documents vs. DocuPipe, AWS Textract, Docparser, Rossum, Hyperscience, Adobe PDF Services, Paperless-ngx, M-Files, Kofax

---

## Overview

This analysis examines document intelligence capabilities across **10 functional components**, comparing RERP Documents against the competitive landscape from a buyer's perspective. Each component is documented as a pitch — the question a buyer asks and the answer their options provide.

The document intelligence market spans three layers:

1. **Document Ingestion & OCR** — Getting documents into the system (scanning, upload, email ingestion)
2. **Data Extraction & Intelligence** — Converting unstructured documents into structured data
3. **Storage, Search & Workflow** — Managing, finding, and acting on document data

The competitors evaluated:

| Vendor | Market Position | Best For | Pricing Model |
|--------|----------------|----------|---------------|
| **DocuPipe** | AI-Extraction Champion | High-volume structured document extraction | Credit-based ($99–$499/mo) |
| **AWS Textract** | Cloud Infrastructure | AWS-native orgs, custom ML solutions | Pay-per-page ($0.0015–$0.07/page) |
| **Docparser** | No-Code Parsing | Small teams, invoice/PO automation | Credit-based ($39–$159/mo) |
| **Rossum** | Enterprise IDP | Large-scale invoice & AP automation | Enterprise only (~$18k+/yr) |
| **Hyperscience** | Enterprise AI Platform | Regulated industries, high accuracy needs | Enterprise only (custom) |
| **Adobe PDF Services** | PDF Manipulation | PDF-specific operations, content generation | Freemium ($500 free transactions/mo) |
| **Paperless-ngx** | Open-Source DMS | Privacy-focused, self-hosted orgs | Free (open source) |
| **M-Files** | Metadata-Driven DMS | Enterprise document governance | Per-user/month (custom) |
| **Kofax** | Legacy Document Automation | Manufacturing, government, heavy enterprise | Enterprise licensing |
| **RERP Documents** | Open-Source, API-First | Dev-driven orgs, data sovereignty, ERP integration | Self-hosted (free) / Hosted (TBD) |

---

## Component Directory

| # | Component | Directory | Status |
|---|-----------|-----------|--------|
| 1 | Document Ingestion | [document-ingestion/README.md](document-ingestion/README.md) | Planned |
| 2 | OCR & Text Extraction | [ocr-extraction/README.md](ocr-extraction/README.md) | Planned |
| 3 | Data Extraction & Standardization | [data-extraction/README.md](data-extraction/README.md) | Planned |
| 4 | Document Classification | [classification/README.md](classification/README.md) | Planned |
| 5 | Storage & Management | [storage-management/README.md](storage-management/README.md) | Planned |
| 6 | Workflow Automation | [workflow-automation/README.md](workflow-automation/README.md) | Planned |
| 7 | Search & Discovery | [search-discovery/README.md](search-discovery/README.md) | Planned |
| 8 | Integration & API | [integration-api/README.md](integration-api/README.md) | Planned |
| 9 | Security & Compliance | [security-compliance/README.md](security-compliance/README.md) | Planned |
| 10 | Reporting & Analytics | [reporting-analytics/README.md](reporting-analytics/README.md) | Planned |

---

## Head-to-Head Capability Summary

| Capability Area | RERP | DocuPipe | AWS Textract | Docparser | Rossum | Hyperscience | Adobe | Paperless-ngx | M-Files | Kofax |
|----------------|------|----------|--------------|-----------|--------|--------------|-------|---------------|---------|-------|
| Document Ingestion | ●○○ | ●●○ | ●●○ | ●●○ | ●●● | ●●● | ●●○ | ●●● | ●●● | ●●● |
| OCR & Text Extraction | ●○○ | ●●● | ●●● | ●●○ | ●●● | ●●● | ●●○ | ●●○ | ●●○ | ●●○ |
| Data Extraction | ●○○ | ●●● | ●●● | ●●○ | ●●● | ●●● | ●○○ | ●○○ | ●○○ | ●●○ |
| Classification | ●○○ | ●●○ | ●●○ | ●○○ | ●●● | ●●● | ●○○ | ●●○ | ●●○ | ●●● |
| Storage Management | ●○○ | ●○○ | ●●○ | ●○○ | ●●○ | ●●○ | ●●○ | ●●● | ●●● | ●●● |
| Workflow Automation | ●○○ | ●●○ | ●○○ | ●●○ | ●●● | ●●● | ●○○ | ●●○ | ●●● | ●●● |
| Search & Discovery | ●○○ | ●●○ | ●●○ | ●●○ | ●●○ | ●●○ | ●●○ | ●●● | ●●● | ●●● |
| Integration API | ●●● | ●●● | ●●● | ●●● | ●●● | ●●● | ●●● | ●●○ | ●●○ | ●●○ |
| Security & Compliance | ●○○ | ●●● | ●●● | ●●○ | ●●● | ●●● | ●●● | ●●○ | ●●● | ●●○ |
| Reporting & Analytics | ●○○ | ●●○ | ●●○ | ●●○ | ●●● | ●●● | ●○○ | ●●○ | ●●● | ●●● |

**Legend:** ●●● = Full feature parity, ●●○ = Partial coverage, ●○○ = Planned / not yet implemented, ●○○ = Minimal / not addressed

---

## RERP Documents' Strategic Position

### Strengths
1. **OpenAPI-first architecture** — Every entity, endpoint, and schema is machine-readable. Enables automatic SDK generation, API contracts, and tooling. No other document platform exposes its data model this cleanly.
2. **Rust-based performance** — Axum + async I/O delivers sub-millisecond API latency. Bulk operations on 100,000+ documents complete in seconds.
3. **Self-hosted, no vendor lock-in** — No per-page pricing, no rate limits, no data egress fees. Full control over infrastructure and data.
4. **ERP-native integration** — Deep integration with RERP's existing microservices (accounting, procurement, inventory) eliminates the need for point-to-point connectors.
5. **Two-crate codegen model** — Separation of generated (from OpenAPI) and implementation (business logic) enables safe regeneration.

### Weaknesses (Current)
1. **Zero implemented features** — No ingestion, OCR, extraction, storage, or search implemented.
2. **No ML models** — No pre-trained models for document classification, data extraction, or OCR.
3. **No document storage** — No object storage integration, no versioning, no retention policies.
4. **No workflow engine** — No visual automation surface, no human-in-the-loop validation.
5. **No compliance certifications** — No SOC 2, ISO 27001, HIPAA, or FedRAMP credentials.

### Threats
1. **AWS Textract's infrastructure moat** — Once orgs build their extraction pipelines on AWS, migration cost is prohibitive.
2. **Rossum's enterprise relationships** — Long-term contracts with Fortune 500 companies create switching costs.
3. **DocuPipe's developer experience** — Clean API, generous free tier, and excellent documentation lower adoption friction.

### Opportunities
1. **SMB/mid-market cost sensitivity** — Organizations tired of per-page pricing at scale (e.g., 2.4M pages/month with Textract = $7,000+/mo).
2. **Developer-first organizations** — Teams that value API contracts over drag-and-drop builders.
3. **Regulated industries** — Healthcare, finance, government — where self-hosting and data sovereignty are required.
4. **AI-native document processing** — RERP's Rust infrastructure is ideal for embedding ML inference at API scale.
5. **ERP integration** — Document processing that natively feeds into accounting, procurement, and inventory workflows.

---

## Implementation Priority Matrix

| Priority | Component | Effort | Impact | Rationale |
|----------|-----------|--------|--------|-----------|
| **P0** | Document Ingestion | Low | Critical | Foundation — nothing works without document intake |
| **P0** | OCR & Text Extraction | Medium | Critical | Core document intelligence feature |
| **P1** | Data Extraction & Standardization | Medium | High | Structured output is the primary value proposition |
| **P1** | Classification | Medium | High | Routing documents to correct processing pipelines |
| **P1** | Storage & Management | Medium | High | Documents need versioning, retention, and lifecycle management |
| **P2** | Search & Discovery | High | High | Users need to find documents and their extracted data |
| **P2** | Workflow Automation | High | High | Human-in-the-loop validation is essential for production |
| **P3** | Integration & API | Medium | High | Must have for dev-first positioning |
| **P3** | Security & Compliance | High | Medium | Important for enterprise but not for first buyers |
| **P4** | Reporting & Analytics | High | Medium | Valuable but can be phased; basic counts first |

---

## Quick Links

- [Current OpenAPI Spec](../openapi.yaml) — RERP Documents gateway specification
- [Core Service Spec](../core/openapi.yaml) — Core entities sub-spec
- [DocuPipe Analysis](../documents/.firecrawl/documents-intelligence/docupipe-full.md) — Full scraped data from DocuPipe
- [AWS Textract Features](../documents/.firecrawl/documents-intelligence/aws-textract-full.md) — Full scraped data from AWS Textract
- [Docparser Pricing](../documents/.firecrawl/documents-intelligence/docparser-pricing.md) — Full scraped pricing data
- [DocuPipe Prior Analysis](../crm/CRM_ANALYSIS/../../competitive-saas-analysis/references/docupipe-analysis.md) — Previous competitive intelligence
