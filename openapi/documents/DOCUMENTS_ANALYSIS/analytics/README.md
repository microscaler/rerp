# Analytics & Reporting

> **Component:** Processing volume dashboards, extraction accuracy tracking, processing time analytics, error rate monitoring, throughput monitoring, and SLA compliance tracking
> **Priority:** P2 — Monitor processing performance and document processing KPIs
> **ABBYY Reference:** ABBYY analytics dashboard, UiPath analytics, Google Document AI metrics, Kofax analytics

---

## The Pitch

**Buyer Question:** *Can I see at a glance how many documents are being processed, how accurate the extractions are, how fast everything runs, and where the bottlenecks are?*

A document processing system without analytics is a black box. A buyer needs real-time visibility into processing performance, extraction accuracy, error rates, and throughput — both for operational monitoring and for demonstrating ROI to management. This component covers all analytics surfaces: processing dashboards, accuracy tracking, time-to-process metrics, error rate monitoring, cost per document, SLA compliance, and custom report builder.

---

## What This Component Does

1. **Processing Volume Dashboard** — Real-time count of documents processed, pending, and failed
2. **Extraction Accuracy Tracking** — Track accuracy by document type, extraction model, and time period
3. **Processing Time Analytics** — Average time to process, time in each pipeline stage, time-to-review
4. **Error Rate Monitoring** — Error rates by stage, error type, and document type
5. **Document Type Distribution** — Breakdown of documents by type and sub-type
6. **Cost Per Document Analysis** — Track processing cost per document type, per period
7. **Throughput Monitoring** — Documents processed per hour, per minute, per day
8. **SLA Compliance Tracking** — Track time-to-process and time-to-review against SLAs
9. **Processing Trend Analysis** — Historical trends in volume, accuracy, and errors
10. **Custom Report Builder** — Build ad-hoc reports with filters and groupings

---

## Entity Model

### ProcessingMetric Entity

A single metric record (aggregated daily/hourly):

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `metric_type` | Enum: [VOLUME, ACCURACY, LATENCY, ERROR_RATE, COST, THROUGHPUT] | Yes | Type of metric |
| `metric_name` | String (128) | Yes | Metric name (e.g., "Invoice Extraction Accuracy") |
| `period_start` | DateTime | Yes | Period start timestamp |
| `period_end` | DateTime | Yes | Period end timestamp |
| `value` | Float | Yes | Metric value |
| `unit` | String (32) | No | Unit of measurement (%, seconds, count, USD) |
| `document_type` | String (64) | No | Document type filter (NULL = all) |
| `connector_name` | String (64) | No | Connector filter (NULL = all) |
| `granularity` | Enum: [HOURLY, DAILY, WEEKLY, MONTHLY] | Yes | Aggregation granularity |

### ProcessingReport Entity

A saved report configuration:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Report name |
| `description` | Text | No | Report description |
| `metric_types` | JSON | Yes | Which metrics to include |
| `filters` | JSON | No | Filter configuration (document types, date ranges) |
| `group_by` | JSON | No | Grouping configuration |
| `created_by` | Foreign Key: User | Yes | Who created the report |
| `is_active` | Boolean | Yes | Enable/disable report |
| `created_at` | DateTime | Yes | Creation timestamp |

### ErrorLog Entity

Detailed error records for processing:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | Foreign Key: DocumentStore | No | Related document |
| `stage` | Enum: [INGESTION, OCR, CLASSIFICATION, EXTRACTION, STORAGE, REVIEW, INTEGRATION] | Yes | Stage where error occurred |
| `error_type` | Enum: [CONNECTION_ERROR, TIMEOUT, PARSING_ERROR, VALIDATION_ERROR, AUTH_ERROR, UNKNOWN] | Yes | Error category |
| `error_message` | Text | Yes | Human-readable error |
| `error_code` | String (64) | No | Error code |
| `document_type` | String (64) | No | Document type at time of error |
| `occurred_at` | DateTime | Yes | When error occurred |
| `resolved_at` | DateTime | No | When error was resolved |
| `resolution` | Text | No | How error was resolved |
| `is_reproducible` | Boolean | No | Is the error reproducible? |

### PerformanceDashboard Entity

Pre-built dashboard widget configuration:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Dashboard name |
| `widget_type` | Enum: [COUNTER, LINE_CHART, BAR_CHART, PIE_CHART, TABLE, GAUGE] | Yes | Visualization type |
| `metric_type` | String (64) | Yes | Which metric to display |
| `time_range` | Enum: [1H, 24H, 7D, 30D, 90D, 1Y, CUSTOM] | Yes | Default time range |
| `filters` | JSON | No | Default filters |
| `position_x` | Integer | No | Dashboard position X |
| `position_y` | Integer | No | Dashboard position Y |
| `position_w` | Integer | No | Dashboard width |
| `position_h` | Integer | No | Dashboard height |

### SLACompliance Entity

SLA tracking for processing and review:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | Foreign Key: DocumentStore | No | Related document |
| `sla_name` | String (128) | Yes | SLA name |
| `sla_target_hours` | Float | Yes | Target hours |
| `actual_hours` | Float | Yes | Actual hours taken |
| `status` | Enum: [MET, BREACHED, PENDING] | Yes | Whether SLA was met |
| `document_type` | String (64) | No | Document type |
| `started_at` | DateTime | Yes | When processing started |
| `completed_at` | DateTime | No | When processing completed |

### ProcessingTrend Entity

Historical trend analysis:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `metric_type` | Enum: [VOLUME, ACCURACY, ERROR_RATE, LATENCY] | Yes | Metric being trended |
| `period` | String (16) | Yes | Period identifier (e.g., "2026-05", "2026-W20") |
| `value` | Float | Yes | Metric value for this period |
| `previous_period_value` | Float | No | Previous period value (for comparison) |
| `change_percentage` | Float | Computed | Percentage change from previous |

---

## Entity Relationships

```
ProcessingMetric (aggregated metric data)
  ├── [DocumentFilter] (via document_type)              ← filter by document type
  └── [ConnectorFilter] (via connector_name)           ← filter by connector

ProcessingReport
  ├── [Metrics] (via metric_types)                     ← which metrics to include
  └── User (via created_by)                            ← who created the report

ErrorLog
  ├── DocumentStore (via document_id)                  ← related document
  └── [StageFilter] (via stage)                        ← which stage failed

PerformanceDashboard
  └── [Metrics] (via metric_type)                      ← which metric to display

SLACompliance
  ├── DocumentStore (via document_id)                  ← related document
  └── [SLA Filter] (via sla_name)                      ← which SLA

ProcessingTrend
  └── [Metric Filter] (via metric_type)                ← which metric trending
```

---

## Required API Endpoints

### Dashboard Metrics

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/analytics/dashboard/overview` | Overview dashboard (all key metrics) |
| `GET` | `/analytics/dashboard/volume` | Processing volume over time |
| `GET` | `/analytics/dashboard/accuracy` | Extraction accuracy by document type |
| `GET` | `/analytics/dashboard/latency` | Processing time by stage |
| `GET` | `/analytics/dashboard/errors` | Error rates and distribution |
| `GET` | `/analytics/dashboard/throughput` | Documents per hour/minute |
| `GET` | `/analytics/dashboard/sla` | SLA compliance rates |

### Cost Analysis

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/analytics/cost/total` | Total processing cost by period |
| `GET` | `/analytics/cost/by-type` | Cost by document type |
| `GET` | `/analytics/cost/by-connector` | Cost by connector |
| `GET` | `/analytics/cost/per-document` | Average cost per document |
| `GET` | `/analytics/cost/trend` | Cost trend over time |

### Error Analytics

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/analytics/errors/by-stage` | Error rates by processing stage |
| `GET` | `/analytics/errors/by-type` | Error distribution by error type |
| `GET` | `/analytics/errors/by-document` | Errors by document type |
| `GET` | `/analytics/errors/trend` | Error rate trend over time |
| `GET` | `/analytics/errors/unresolved` | Unresolved errors |

### Processing Analytics

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/analytics/processing/by-type` | Processing volume by document type |
| `GET` | `/analytics/processing/by-stage` | Time in each processing stage |
| `GET` | `/analytics/processing/trend` | Processing volume trend |
| `GET` | `/analytics/processing/bottlenecks` | Processing bottlenecks |

### SLA Compliance

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/analytics/sla/compliance` | Overall SLA compliance rate |
| `GET` | `/analytics/sla/by-type` | SLA compliance by document type |
| `GET` | `/analytics/sla/breaches` | SLA breach details |
| `GET` | `/analytics/sla/trend` | SLA compliance trend |

### Custom Reports

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/analytics/reports` | List all saved reports |
| `POST` | `/analytics/reports` | Create custom report |
| `PATCH` | `/analytics/reports/{id}` | Update report |
| `DELETE` | `/analytics/reports/{id}` | Delete report |
| `POST` | `/analytics/reports/{id}/run` | Run a saved report |
| `GET` | `/analytics/reports/{id}/export` | Export report (CSV/JSON) |

---

## Competitive Positioning

### Where RERP Wins

- **Self-hosted analytics** — No per-dashboard licensing. Unlimited dashboards on your infrastructure.
- **OpenAPI-defined report schemas** — Every metric, filter, and grouping is machine-readable. BI tools can query analytics data directly via API.
- **Rust-level aggregation speed** — Aggregating 1 million document processing records in Rust is instantaneous. Python-based analytics (ABBYY) can be slow at scale.
- **Zero marginal cost at scale** — Analyzing 1M processing events costs the same as 10K.

### Where RERP Lags

- **No analytics surface** — No dashboards, no metrics, no reporting.
- **No accuracy tracking** — No extraction accuracy by document type or time period.
- **No cost analysis** — No cost per document tracking.
- **No SLA compliance** — No time-to-process or time-to-review SLA tracking.
- **No error analytics** — No error rate monitoring or trend analysis.

---

## Competitive Intelligence Deep Dive

### ABBYY Analytics Dashboard

ABBYY FlexiCapture includes built-in analytics dashboards showing processing volume, accuracy rates, throughput, and error distribution. Metrics can be filtered by document type, time period, and operator. The key differentiator is enterprise-grade reporting with configurable dashboards. However, analytics are limited to ABBYY's built-in views — no custom API for external BI tools. Cost: part of $100K+ enterprise license.

### UiPath Analytics

UiPath provides comprehensive analytics for Document Understanding: processing volume, accuracy by document type, processing time, error rates, throughput, and human review metrics. UiPath's Analytics Dashboard is built on top of the UiPath Platform's data layer. The key differentiator is integration with UiPath Orchestrator for end-to-end automation analytics. For UiPath users, this provides a single pane of glass for all automation metrics.

### Google Document AI Metrics

Google Document AI provides built-in metrics in the Cloud Console: pages processed, error rates, API latency, and billing. The key advantage is tight integration with Google Cloud's monitoring ecosystem (Cloud Monitoring, BigQuery). Metrics can be exported to BigQuery for custom analysis. However, the built-in dashboard is basic — no document-type-specific accuracy tracking or error distribution.

### Azure Document Intelligence Diagnostics

Azure provides diagnostic logs and metrics in Azure Monitor: request volume, latency, error rates, and billing. The key advantage is integration with Azure Monitor and Log Analytics. Metrics can be queried via KQL (Kusto Query Language) for custom analysis. However, the built-in dashboard is minimal — no document-type-specific analytics or SLA tracking.

---

## Implementation Roadmap

### Phase 1: Core Metrics Collection (2-3 weeks) — P2

1. Define `ProcessingMetric` entity with metric types (VOLUME, ACCURACY, LATENCY, ERROR_RATE, COST, THROUGHPUT)
2. Implement metric collection at each pipeline stage (ingest → ocr → classify → extract → store)
3. Implement overview dashboard endpoint (GET /analytics/dashboard/overview)
4. Implement processing volume over time endpoint
5. Implement daily aggregation cron job

### Phase 2: Error & Accuracy Analytics (2-3 weeks) — P2

1. Define `ErrorLog` entity with stage and error_type classification
2. Implement error tracking at each pipeline stage
3. Implement error rate by stage endpoint
4. Implement error rate by document type endpoint
5. Implement extraction accuracy tracking (comparing extracted vs. reviewed values)
6. Implement error trend analysis endpoint

### Phase 3: Cost & SLA Analytics (2-3 weeks) — P2

1. Define `SLACompliance` entity for processing and review SLAs
2. Implement SLA tracking (time-to-process, time-to-review)
3. Implement SLA compliance endpoint
4. Implement cost per document calculation (based on processing pipeline steps)
5. Implement cost analysis endpoints (by type, by connector, trend)

### Phase 4: Custom Reports & Dashboards (3-4 weeks) — P2

1. Define `ProcessingReport` entity for saved reports
2. Implement custom report builder endpoint (POST /analytics/reports)
3. Implement report execution engine (aggregate metrics with filters and groupings)
4. Implement CSV/JSON report export
5. Define `PerformanceDashboard` entity for pre-built dashboards
6. Implement dashboard widget endpoints

### Phase 5: Advanced Analytics (3-4 weeks) — P2

1. Implement `ProcessingTrend` entity for historical trend analysis
2. Implement trend comparison (current vs. previous period)
3. Implement bottleneck analysis (where processing slows down)
4. Add anomaly detection (unusual spikes in errors or latency)
5. Implement automated alerting (email/webhook on SLA breach or error threshold)

---

## Key Takeaway for Buyers

Analytics are where document processing demonstrates ROI. A buyer needs to know: *Can I see at a glance how many documents are being processed, how accurate the extractions are, and where the bottlenecks are?* RERP's advantage is self-hosted analytics with zero per-dashboard licensing, OpenAPI-defined report schemas that BI tools can query directly, and Rust-level aggregation speed that outperforms Python-based alternatives. The immediate priority: define the ProcessingMetric entity, implement metric collection at each pipeline stage, and build the overview dashboard. Everything else builds on this foundation.
