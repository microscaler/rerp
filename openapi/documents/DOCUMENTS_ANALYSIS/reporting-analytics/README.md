# Reporting & Analytics

> **Component:** Document processing metrics, extraction quality, workflow performance, and business intelligence
> **Priority:** P4 — Valuable for management justification but not for first buyers
> **DocuPipe Reference:** Credit consumption tracking, pages processed, API call counts, cost metrics
> **Rossum Reference:** Workflow reporting — automation rates, team performance, processing times, exception rates, SLA compliance

---

## The Pitch

**Buyer Question:** *Can I measure the value of document intelligence — accuracy rates, processing times, cost savings, ROI — with dashboards and reports that justify the investment?*

If the answer is no, you have a document processing tool, not a business intelligence platform. The ROI of document intelligence is invisible without metrics. Management needs to see accuracy rates, processing volumes, time savings, and cost reductions to justify continued investment. Reporting is the bridge between technical performance and business value. This component defines how metrics are collected, how reports are generated, and how insights are presented to decision-makers.

---

## What This Component Does

Reporting & Analytics is the intelligence layer on top of the document pipeline:

1. **Processing Dashboards** — Real-time metrics on document processing volumes, speeds, and accuracy
2. **Extraction Quality Reports** — Accuracy rates, confidence distributions, exception rates
3. **Workflow Analytics** — Throughput, bottlenecks, approval times, SLA compliance
4. **Cost Analytics** — Processing costs per document type, per team, per department
5. **Trend Analysis** — Document volume trends, extraction accuracy trends, seasonal patterns
6. **Export & Integration** — Reports exportable to CSV, PDF, or integrated with BI tools
7. **Custom Dashboards** — User-configurable dashboard layouts and metrics
8. **Scheduled Reports** — Automated report generation and distribution

---

## Entity Model

### Metric Entity

The raw data point. Every measurement is stored here with dimensions for slicing.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `metric_name` | String (128) | Yes | Metric name (e.g., pages_processed, extraction_accuracy) |
| `metric_value` | Float | Yes | Metric value |
| `dimension_type` | String (64) | No | Dimension (document_type, team, department) |
| `dimension_id` | UUID | No | Dimension identifier |
| `timestamp` | DateTime | Yes | Metric timestamp (minute/hour/day granularity) |
| `source` | String (64) | Yes | Metric source system (ocr, extraction, workflow) |
| `tags` | String[] | No | Additional tags for grouping |

### Report Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (255) | Yes | Report name |
| `template` | JSONB | Yes | Report template definition (metrics, filters, charts) |
| `schedule` | String (64) | No | Cron schedule (empty = manual only) |
| `recipients` | UUID[] | No | Report recipients |
| `format` | Enum: [PDF, CSV, HTML] | No | Output format |
| `is_active` | Boolean | No | Report activation (default: true) |
| `created_at` | DateTime | Yes | Creation timestamp |
| `last_generated_at` | DateTime | No | Last generation timestamp |

### Dashboard Widget Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `dashboard_id` | UUID | Yes | Parent dashboard |
| `name` | String (255) | Yes | Widget name |
| `widget_type` | Enum: [COUNTER, LINE_CHART, BAR_CHART, TABLE, GAUGE] | Yes | Widget type |
| `metric` | String (128) | Yes | Connected metric |
| `config` | JSONB | Yes | Widget configuration (filters, colors, thresholds) |
| `position_x` | Integer | No | Grid position X |
| `position_y` | Integer | No | Grid position Y |
| `width` | Integer | No | Widget width (1-12 columns) |

### Dashboard Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (255) | Yes | Dashboard name |
| `widgets` | JSONB | Yes | Widget definitions (array of widget IDs) |
| `owner_id` | UUID | Yes | Dashboard owner |
| `is_default` | Boolean | No | Default dashboard for users |
| `is_public` | Boolean | No | Shareable with team |
| `created_at` | DateTime | Yes | Creation timestamp |

---

## Entity Relationships

```
Metric (central data source)
  ├── Dashboard Widget (many-to-one)    ← via metric name
  └── Report (many-to-one)              ← via metrics in template

Report
  └── Metric (many-to-one)              ← via metrics in template

Dashboard
  └── Dashboard Widget (one-to-many)    ← via dashboard_id

Dashboard Widget
  ├── Dashboard (many-to-one)           ← via dashboard_id
  └── Metric (many-to-one)              ← via metric name
```

---

## Rossum Technical Patterns to Follow

### Pattern 1: Workflow Automation Analytics

Rossum provides comprehensive workflow reporting that tracks:
- **Automation rates** — Percentage of documents processed without human intervention
- **Team performance** — Processing times, correction rates, approvals per team member
- **Processing times** — End-to-end time from ingestion to final approval
- **Exception rates** — Percentage of documents requiring manual review
- **SLA compliance** — Percentage of documents processed within SLA timeframes

This reporting is designed for enterprise management — proving ROI to stakeholders. The key insight is that document processing metrics should be presented in business terms (time saved, cost reduced, accuracy improved), not technical terms (API calls, page counts).

**Recommendation: RERP should present metrics in business terms.** Instead of "pages processed," show "documents processed." Instead of "API calls," show "time saved." The dashboard should answer: how many invoices were processed? How many were auto-approved? What's the average processing time? What's the error rate? How much time/money was saved?

### Pattern 2: Confidence Score Distributions

Rossum tracks confidence scores across all extractions. This reveals:
- **High-confidence extractions** (≥0.95) — auto-approved
- **Medium-confidence extractions** (0.7-0.95) — reviewed by junior staff
- **Low-confidence extractions** (<0.7) — reviewed by senior staff or rejected

The distribution of confidence scores directly correlates with automation rates and operational costs. By tracking this over time, organizations can identify which document types are improving and which need more attention.

**Recommendation: RERP should implement confidence score distribution analytics.** Track the distribution of confidence scores over time, grouped by document type, schema, and user. This enables targeted improvements — if a document type has consistently low confidence scores, the extraction schema needs refinement.

---

## Competitive Intelligence Deep Dive

### DocuPipe: Basic Usage Metrics
DocuPipe provides basic usage metrics: pages processed, credits consumed, API call counts. No extraction quality reports or workflow analytics. Cost metrics are straightforward (credits × rate) but limited to credit consumption. No trend analysis or custom dashboards. The focus is on operational metrics, not business intelligence.

**Key weakness:** Only credit consumption tracking, no quality or performance metrics.

### AWS Textract: CloudWatch Integration
Textract integrates with AWS CloudWatch for monitoring and AWS QuickSight for analytics. You can track API calls, page counts, error rates, and latency through CloudWatch. QuickSight provides dashboards and trend analysis. The advantage is deep AWS integration with unlimited customization. The disadvantage is operational complexity — you're building analytics on AWS primitives.

**Key strengths:** CloudWatch integration, QuickSight dashboards, unlimited customization
**Key weaknesses:** Operational complexity, AWS lock-in, operational metrics only

### Rossum: Enterprise Workflow Analytics
Rossum provides comprehensive workflow reporting: automation rates, team performance, processing times, exception rates. The validation screen includes metrics on review times and correction patterns. Custom business logic can generate tailored reports. The analytics are designed for enterprise management — proving ROI to stakeholders.

**Key strengths:** Workflow analytics, team performance tracking, ROI reporting
**Key weaknesses:** Enterprise-only, no self-hosted option

### Paperless-ngx: Basic Dashboard Statistics
Paperless-ngx provides basic dashboard statistics: total documents, storage used, OCR results, processing times. Customizable dashboard with saved views. No extraction quality metrics (no extraction pipeline). The analytics are designed for personal or small-team use, not enterprise reporting. Free and self-hosted.

**Key strengths:** Free, self-hosted, basic statistics
**Key weaknesses:** No extraction quality metrics, no ROI reporting

### M-Files: Enterprise BI Integration
M-Files integrates with Microsoft Power BI and other BI tools. Deep Microsoft 365 integration means documents are searchable and analyzable in Power BI, Excel, and other M365 tools. Custom reporting with metadata-driven dimensions. The analytics are designed for enterprise business intelligence — connecting document data to broader business metrics.

**Key strengths:** Power BI integration, M365 analytics, metadata-driven dimensions
**Key weaknesses:** Enterprise pricing, Microsoft lock-in

---

## Competitive Positioning

### Where RERP Wins
- **Self-hosted, no analytics licensing** — Unlike Rossum (included in $18k+/yr) or M-Files (enterprise pricing), RERP provides full analytics at zero cost
- **Business-term metrics** — Unlike DocuPipe (credit consumption) or Textract (API calls), RERP presents metrics in business terms (time saved, accuracy, ROI)
- **OpenAPI-defined metrics** — Every metric is defined in OpenAPI specs, enabling automatic SDK generation and BI tool integration

### Where RERP Lags
- **No metrics collection** — Not yet implemented
- **No dashboards** — Not yet implemented
- **No ROI reporting** — Not yet implemented

---

## Implementation Roadmap

### Phase 1: Basic Metrics (2-3 weeks) — P4
1. Define `Metric` entity with dimension support
2. Implement processing volume metrics (documents processed, pages processed)
3. Basic dashboard with key metrics (counter widgets)
4. Document type distribution charts (bar charts)
5. Processing time statistics (average, median, p95)

### Phase 2: Quality Analytics (3-4 weeks) — P4
1. Extraction accuracy metrics (confidence score distributions)
2. Exception rate tracking (documents requiring manual review)
3. Human review statistics (review times, correction rates)
4. Quality trend analysis (accuracy over time, by document type)
5. Confidence score distribution charts

### Phase 3: Business Analytics (4-6 weeks) — P4
1. Cost analytics (processing cost per document type, team, department)
2. ROI calculation and reporting (time saved, cost reduced, accuracy improved)
3. Workflow bottleneck analysis (throughput by stage, approval times)
4. SLA compliance reporting (documents processed within SLA timeframes)
5. Custom dashboard builder (drag-and-drop widget editor)

### Phase 4: Advanced BI (3-4 weeks) — P4
1. Scheduled report generation (cron-based, PDF/CSV/HTML output)
2. Report export and email distribution
3. Power BI / Tableau integration (OData feed for BI tools)
4. Predictive analytics (volume forecasting, error prediction)
5. Executive summary generation (automated narrative reports)

---

## Key Takeaway for Buyers

RERP Documents' reporting pitch is **open-source, self-hosted, and ERP-integrated**. Unlike cloud solutions (DocuPipe, Rossum) where analytics are limited or vendor-dependent, RERP provides comprehensive reporting with zero per-report costs. Unlike Textract (AWS CloudWatch, operational focus), RERP's analytics are business-focused — connecting document processing metrics to ERP business outcomes.

The Rust-native analytics engine processes millions of metric records in real-time with sub-millisecond query latency. And because reporting is defined in OpenAPI, every client gets type-safe SDKs, automatic validation, and complete API documentation for analytics operations.

**The immediate priority: implement basic processing metrics, define the metric entity, and build the dashboard endpoint. Reporting transforms technical performance into business value.**
