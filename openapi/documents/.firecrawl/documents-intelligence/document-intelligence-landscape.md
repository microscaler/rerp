# Document Intelligence Competitive Intelligence

**Analysis Date:** 2026-05-11
**Scope:** Document processing, OCR, data extraction, document management platforms
**Used for:** RERP Documents suite competitive positioning

---

## Competitor Summary

### DocuPipe (docupipe.ai)
- **Pricing:** Credit-based ($99 Starter, $499 Premium, Enterprise custom)
- **Model:** 100 free credits/mo, 20% annual discount
- **Per-page cost:** Parse=1 credit, Standardize=2 credits, core workflow=3 credits/page
- **Strengths:** Natural language schema definition, all-in-one API, developer-friendly, excellent docs at readme.io
- **Weaknesses:** No standalone OCR, no document storage, no visual workflow, no self-hosted
- **Security:** SOC 2 Type II, ISO 27001, GDPR, HIPAA

### AWS Textract
- **Pricing:** Pay-per-page, tiered after 1M pages
- **Per-page costs:** DetectText=$0.0015, Forms=$0.05, Tables=$0.015, Queries=$0.015, Custom Queries=$0.025, Expense=$0.01, ID=$0.025, Lending=$0.07
- **Free tier:** 1,000 pages/mo basic OCR; 100 pages for forms/tables/queries
- **Strengths:** 5 separate APIs, custom queries, identity document extraction, deep AWS integration
- **Weaknesses:** AWS lock-in, operational complexity, no built-in search or storage, no webhook native support
- **Security:** SOC 1/2/3, ISO 27001, PCI DSS, HIPAA eligible, FedRAMP authorized

### Docparser (docparser.com)
- **Pricing:** Credit-based (1 credit = 1 doc up to 5 pages)
- **Plans:** Starter $39/mo, Pro $74/mo, Business $159/mo, Enterprise custom
- **Add-ons:** MFA $5/mo, Version Control $8/mo, Multi-Layout $25/mo, Setup $149/layout
- **Strengths:** No-code parser builder, 100+ integrations, smart AI parser, template access
- **Weaknesses:** Per-page costs at scale, no standalone OCR, limited enterprise features
- **Security:** Basic — no public certifications mentioned

### Rossum (rossum.ai)
- **Pricing:** Enterprise only, Starter starting at $18,000/year
- **Minimum:** 1-year contract required
- **Strengths:** Aurora Document AI (in-house ML), validation screen, master data matching, SAP/Coupa/Workday/Oracle integrations
- **Weaknesses:** Enterprise-only pricing, no self-hosted, no free tier beyond 14-day trial
- **Security:** ISO 27001, SOC 2, HIPAA, TX-RAMP, models built in-house (no third-party LLM)

### Hyperscience (hyperscience.com)
- **Pricing:** Enterprise only, custom
- **Accuracy:** 99.5% documented accuracy
- **Strengths:** ORCA Vision-Language Model, FedRAMP High authorized, Python-first API, GenAI enablement for LLM training data
- **Weaknesses:** Enterprise-only, no self-hosted, custom integration needed
- **Security:** FedRAMP High, SOC 2 Type II, ISO 27001 — highest security tier

### Adobe PDF Services (adobe.io)
- **Pricing:** Freemium — 500 free Document Transactions/month
- **Paid:** Volume/multi-product discounts, contact sales
- **Strengths:** 15+ PDF operations, PDF Extract API, Accessibility Auto-Tag, Document Generation, excellent SDKs
- **Weaknesses:** PDF-focused (not general document intelligence), per-transaction pricing, no webhook support
- **Security:** SOC 2, ISO 27001, ISO 27018

### Paperless-ngx (paperless-ngx.com)
- **Pricing:** Free, open-source (AGPL v3)
- **Strengths:** Self-hosted, Tesseract OCR (100+ languages), full-text search, ML auto-tagging, email ingestion, no per-page costs
- **Weaknesses:** No structured data extraction, no enterprise certifications, no cloud storage backends, user-managed backups
- **Security:** Data stays on user server (privacy-first by design)

### M-Files (m-files.com)
- **Pricing:** Per-user/month, custom enterprise licensing
- **Strengths:** Metadata-driven (not folder-based), Gartner Leader 2026 Magic Quadrant, Microsoft 365 integration, GenAI (Aino), workflow automation
- **Weaknesses:** Enterprise pricing, no self-hosted, M365-dependent
- **Security:** SOC 2, ISO 27001, ISO 27018, deep Azure AD integration

### Kofax/Tungsten Automation
- **Pricing:** Enterprise licensing, perpetual + subscription models
- **Strengths:** Legacy enterprise relationships, manufacturing/government focus, Power PDF line
- **Weaknesses:** Pages frequently return 404 or browser-incompatible, product lines fragmented, slow to adopt cloud-native approaches

---

## Key Pricing Insights

| Vendor | Entry Cost | Scale Cost Model | Best For |
|--------|-----------|------------------|----------|
| DocuPipe | 100 free credits/mo | Credits (3/page workflow) | SMB to mid-market |
| AWS Textract | 1,000 free pages/mo | $0.0015-$0.07/page | AWS-native orgs |
| Docparser | 14-day free trial | $39-$159/mo | Small teams |
| Rossum | 14-day trial | $18k+/yr minimum | Enterprise |
| Hyperscience | Custom demo | Enterprise only | Regulated industries |
| Adobe | 500 free transactions/mo | Contact sales | PDF operations |
| Paperless-ngx | Free | Free (self-hosted) | Privacy-focused |
| M-Files | 30-day trial | Per-user (custom) | Enterprise governance |

## Positioning Notes for RERP Documents

**RERP's winning angles:**
- Self-hosted = zero per-page/per-transaction costs (unlike DocuPipe, Textract, Docparser, Adobe)
- OpenAPI-first = machine-readable schemas, auto-generated SDKs (unlike any competitor)
- Rust-native = sub-millisecond API latency, handles 10k+ concurrent operations
- ERP-native = document processing feeds directly into accounting, procurement, inventory workflows

**RERP's gaps to close:**
- Zero implemented features vs. fully operational competitors
- No ML models for OCR/extraction/classification
- No compliance certifications (SOC 2, ISO 27001, HIPAA)
- No enterprise sales motion / customer base
