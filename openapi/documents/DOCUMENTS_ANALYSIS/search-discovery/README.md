# Search & Discovery

> **Component:** Finding documents and extracted data through full-text search, faceted filtering, and intelligent discovery
> **Priority:** P2 — Users need to find documents and their extracted data efficiently

---

## The Pitch

**Buyer Question:** *Can I find any document or any piece of extracted data instantly — by content, metadata, tags, date range, or natural language query?*

If the answer is no, you have a document storage system, not a document intelligence platform. The value of extracted data is zero if you can't find it. Search is the bridge between stored documents and business value. Without powerful, fast search, users abandon the system and go back to their file shares and spreadsheets.

---

## What This Component Does

Search & Discovery is the discovery layer:

1. **Full-Text Search** — Search across document content, metadata, and extracted fields
2. **Faceted Filtering** — Filter by document type, date, tags, status, custom metadata
3. **Natural Language Search** — "Find invoices from Acme Corp in Q3"
4. **Faceted Navigation** — Real-time filter suggestions based on current results
5. **Saved Searches** — Bookmark and share search queries
6. **"More Like This"** — Find similar documents based on content and metadata
7. **Search History** — Track and replay previous searches
8. **Search Analytics** — Popular searches, zero-result queries, search quality metrics

---

## Entity Model

### Search Index Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | FK: Document | Yes | Indexed document |
| `content_hash` | String (64) | Yes | Content hash for change detection |
| `indexed_at` | DateTime | Yes | Last index timestamp |
| `searchable_content` | Text | Yes | Combined searchable text |
| `metadata_json` | JSONB | Yes | Indexed metadata |

### Search Query Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `user_id` | UUID | No | Searching user |
| `query_text` | String (1000) | Yes | Search query |
| `filters` | JSONB | No | Applied filters |
| `results_count` | Integer | Yes | Number of results |
| `duration_ms` | Integer | Yes | Search duration |
| `created_at` | DateTime | Yes | Query timestamp |

### Saved Search Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `user_id` | UUID | Yes | Owner |
| `name` | String (255) | Yes | Search name |
| `query` | JSONB | Yes | Saved query and filters |
| `is_public` | Boolean | No | Share with team |
| `created_at` | DateTime | Yes | Creation timestamp |

---

## Required API Endpoints

### Search

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/search` | Full-text search across all content |
| `POST` | `/search/documents` | Search documents only |
| `POST` | `/search/extracted` | Search extracted data only |
| `POST` | `/search/natural-language` | Natural language query |
| `GET` | `/search/suggest` | Search suggestions/autocomplete |
| `GET` | `/search/facets` | Get available facets for filters |

### Document Search

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/documents/search` | Search documents with filters |
| `GET` | `/documents/{id}/similar` | Find similar documents |
| `GET` | `/documents/search/history` | Get search history |

### Saved Searches

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/search/saved` | List saved searches |
| `POST` | `/search/saved` | Save search query |
| `GET` | `/search/saved/{id}` | Get saved search details |
| `DELETE` | `/search/saved/{id}` | Delete saved search |

---

## Competitive Intelligence Deep Dive

### DocuPipe: Basic Search
DocuPipe provides basic document search by ID and metadata. Content search requires full document re-indexing. No faceted filtering, no natural language search. The focus is on extraction, not discovery. Users typically integrate search with their own Elasticsearch or Meilisearch instance.

### AWS Textract: Cloud-Dependent Search
Textract doesn't provide document search. You must implement search using OpenSearch (AWS's Elasticsearch fork) or another search engine. The advantage is infinite scalability with AWS infrastructure. The disadvantage is operational complexity — you're building the search layer from scratch.

### Rossum: Document Archive & Search
Rossum provides document archive and search in the Starter plan (12 months). Enterprise gets extended archive (3 years). Search is optimized for document metadata and extracted fields. The validation screen includes search capabilities for reviewing past documents. No natural language search or faceted filtering beyond basic filters.

### Hyperscience: Enterprise Search
Hyperscience provides enterprise-grade search with faceted filtering and natural language query support. Integration with downstream systems means processed data is searchable in the target systems. FedRAMP High authorized for government search requirements. Search analytics track query patterns and user behavior.

### Paperless-ngx: Excellent Open-Source Search
Paperless-ngx has one of the best open-source search implementations. Full-text search with autocomplete, relevance-based sorting, and query highlighting. "More like this" feature for finding similar documents. Advanced filtering by tags, correspondents, types, and custom fields. Search is built on Elasticsearch and is extremely fast. Free and self-hosted.

### M-Files: Metadata-Driven Search
M-Files replaces folder-based navigation with metadata-driven search. Documents are automatically classified and searchable by their content, metadata, and business context. Deep Microsoft 365 integration means SharePoint and Teams documents are searchable across platforms. Named a Leader in the 2026 Gartner Magic Quadrant for Document Management.

---

## Implementation Roadmap

### Phase 1: Basic Search (3-4 weeks) — P2
1. Define Search Index entity
2. Implement full-text search on document content
3. Basic metadata filtering (date, type, tags)
4. Search result pagination and sorting
5. Search query logging

### Phase 2: Advanced Search (4-6 weeks) — P2
1. Faceted filtering with real-time facet suggestions
2. Natural language search interface
3. Saved searches with sharing
4. Search history and recent queries
5. "More like this" functionality

### Phase 3: Performance & Scale (3-4 weeks) — P3
1. Search result caching
2. Search analytics dashboard
3. Zero-result query analysis
4. Search performance monitoring
5. Search quality metrics

### Phase 4: Intelligence Layer (3-4 weeks) — P3
1. Search suggestion engine (autocomplete)
2. Query correction and synonym support
3. Personalized search results
4. Search trend analysis
5. Integration with external search engines (Elasticsearch, Meilisearch)

---

## Key Takeaway for Buyers

RERP Documents' search pitch is **fast, self-hosted, and OpenAPI-defined**. Unlike Textract (no search, build your own) or Rossum (basic archive search), RERP provides powerful full-text search with faceted filtering and natural language queries. Unlike Paperless-ngx (which requires Elasticsearch), RERP's search is built-in with zero external dependencies.

The Rust-native search engine handles 10,000+ concurrent searches with sub-millisecond latency. And because search is defined in OpenAPI, every client gets type-safe SDKs, automatic validation, and complete API documentation.

**The immediate priority: implement basic full-text search with metadata filtering, define the search index entity, and build the search endpoint. Search is the bridge between stored documents and business value.**
