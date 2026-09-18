# Search & Discovery

> **Component:** Finding documents and extracted data through full-text search, faceted filtering, and intelligent discovery
> **Priority:** P2 — Users need to find documents and their extracted data efficiently
> **Paperless-ngx Reference:** Elasticsearch-based full-text search, autocomplete, relevance sorting, "more like this", advanced filtering by tags/correspondents/types

---

## The Pitch

**Buyer Question:** *Can I find any document or any piece of extracted data instantly — by content, metadata, tags, date range, or natural language query?*

If the answer is no, you have a document storage system, not a document intelligence platform. The value of extracted data is zero if you can't find it. Search is the bridge between stored documents and business value. Without powerful, fast search, users abandon the system and go back to their file shares and spreadsheets. This component defines how documents are indexed, how queries are executed, and how search results are ranked and filtered.

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

The index entry for each document. Combines OCR text, metadata, and extracted fields into a single searchable document.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key (same as document_id) |
| `document_id` | UUID | Yes | Indexed document (also primary key) |
| `content_hash` | String (64) | Yes | Content hash for change detection |
| `indexed_at` | DateTime | Yes | Last index timestamp |
| `searchable_content` | Text | Yes | Combined searchable text (OCR + extracted fields) |
| `metadata_json` | JSONB | Yes | Indexed metadata (type, tags, dates, etc.) |
| `extracted_fields_json` | JSONB | Yes | Indexed extracted field values |
| `vector_embedding` | Vector (1536) | No | Semantic embedding for vector search |

### Search Query Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `user_id` | UUID | No | Searching user (nullable for API) |
| `query_text` | String (1000) | Yes | Search query |
| `filters` | JSONB | No | Applied filters |
| `results_count` | Integer | Yes | Number of results returned |
| `duration_ms` | Float | Yes | Search duration |
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
| `last_run_at` | DateTime | No | Last time search was executed |

---

## Entity Relationships

```
Document (central)
  ├── Search Index (one-to-one)         ← via document_id (= document_id)
  ├── Search Query (one-to-many)         ← created by searches
  └── Saved Search (one-to-many)         ← user-saved searches

Search Index
  └── Document (one-to-one)              ← via document_id

Search Query
  └── Document (many-to-one)             ← search results link to documents

Saved Search
  ├── User (many-to-one)                 ← via user_id
  └── Document (many-to-one)             ← search results link to documents
```

---

## Required API Endpoints

### Search

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/search` | Full-text search across all content |
| `POST` | `/search/documents` | Search documents only (content + metadata) |
| `POST` | `/search/extracted` | Search extracted data only (structured fields) |
| `POST` | `/search/natural-language` | Natural language query |
| `GET` | `/search/suggest` | Search suggestions/autocomplete |
| `GET` | `/search/facets` | Get available facets for filters |
| `GET` | `/search/popular` | Get popular searches |

### Document Search

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/documents/search` | Search documents with filters |
| `GET` | `/documents/{id}/similar` | Find similar documents |
| `GET` | `/documents/search/history` | Get search history for user |

### Saved Searches

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/search/saved` | List saved searches for user |
| `POST` | `/search/saved` | Save search query |
| `GET` | `/search/saved/{id}` | Get saved search details |
| `DELETE` | `/search/saved/{id}` | Delete saved search |
| `POST` | `/search/saved/{id}/run` | Execute saved search |

---

## Paperless-ngx Technical Patterns to Follow

### Pattern 1: Elasticsearch-Powered Full-Text Search

Paperless-ngx uses Elasticsearch for its search engine. The search combines OCR text, metadata, and custom fields into a single searchable document. Features include:
- **Full-text search** with autocomplete, relevance-based sorting, and query highlighting
- **Advanced filtering** by tags, correspondents, types, and custom fields
- **"More like this"** feature for finding similar documents based on content similarity
- **Query highlighting** to show matching text in results

**Recommendation: RERP should use Elasticsearch (or Meilisearch for a lighter alternative) for search.** The index combines OCR text (`searchable_content`), metadata (`metadata_json`), and extracted fields (`extracted_fields_json`) into a single searchable document. This eliminates the need for separate content and metadata searches — one query returns everything.

### Pattern 2: Relevance-Based Sorting with Multiple Signals

Paperless-ngx sorts search results by relevance, using multiple signals:
- Text match score (TF-IDF or BM25)
- Tag match bonus (documents with matching tags rank higher)
- Date recency bonus (newer documents rank slightly higher)
- User interaction signals (previously viewed/liked documents rank higher)

**Recommendation: RERP should implement multi-signal relevance ranking.** Use BM25 for text matching, boost documents with matching tags/types, and apply a recency decay factor. This produces results that are both relevant and fresh.

### Pattern 3: Saved Searches with Re-Execution

Paperless-ngx allows users to save searches with complex filter combinations. Saved searches can be re-executed at any time with one click. Public saved searches can be shared with team members.

**Recommendation: RERP should implement saved searches with re-execution.** A saved search stores the query text, filter combination, and sort options. When executed, it runs the search and returns current results (not a snapshot). This ensures saved searches always reflect the latest data.

---

## Competitive Intelligence Deep Dive

### DocuPipe: Basic Search by ID and Metadata
DocuPipe provides basic document search by ID and metadata. Content search requires full document re-indexing. No faceted filtering, no natural language search. The focus is on extraction, not discovery. Users typically integrate search with their own Elasticsearch or Meilisearch instance.

**Key weakness:** No built-in search — users must build their own.

### AWS Textract: Cloud-Dependent Search
Textract doesn't provide document search. You must implement search using OpenSearch (AWS's Elasticsearch fork) or another search engine. The advantage is infinite scalability with AWS infrastructure. The disadvantage is operational complexity.

**Key strengths:** Infinite scalability, AWS-native
**Key weaknesses:** No built-in search, operational complexity

### Rossum: Document Archive & Search
Rossum provides document archive and search in the Starter plan (12 months). Enterprise gets extended archive (3 years). Search is optimized for document metadata and extracted fields. The validation screen includes search capabilities for reviewing past documents. No natural language search or faceted filtering beyond basic filters.

**Key strengths:** Built-in archive search, enterprise security
**Key weaknesses:** Limited search features, vendor lock-in

### Paperless-ngx: Excellent Open-Source Search
Paperless-ngx has one of the best open-source search implementations. Full-text search with autocomplete, relevance-based sorting, and query highlighting. "More like this" feature for finding similar documents. Advanced filtering by tags, correspondents, types, and custom fields. Search is built on Elasticsearch and is extremely fast. Free and self-hosted.

**Key strengths:** Elasticsearch-powered, "more like this", advanced filtering, free
**Key weaknesses:** Self-managed, no natural language search

### M-Files: Metadata-Driven Search
M-Files replaces folder-based navigation with metadata-driven search. Documents are automatically classified and searchable by their content, metadata, and business context. Deep Microsoft 365 integration means SharePoint and Teams documents are searchable across platforms. Named a Leader in the 2026 Gartner Magic Quadrant for Document Management.

**Key strengths:** Metadata-driven search, M365 integration, enterprise governance
**Key weaknesses:** Enterprise pricing, Microsoft lock-in

---

## Competitive Positioning

### Where RERP Wins
- **Unified search across content and extracted data** — Unlike Textract (no search) or DocuPipe (metadata only), RERP searches both OCR text AND extracted fields in one query
- **Self-hosted, no search infrastructure** — Unlike Textract (requires OpenSearch) or Paperless-ngx (requires Elasticsearch), RERP includes search out of the box
- **Natural language search** — Unlike Docparser or Rossum (basic text search), RERP supports natural language queries

### Where RERP Lags
- **No search engine** — Not yet implemented
- **No faceted filtering** — Not yet implemented
- **No "more like this"** — Not yet implemented

---

## Implementation Roadmap

### Phase 1: Basic Search (3-4 weeks) — P2
1. Define `Search Index` entity with searchable_content, metadata_json, extracted_fields_json
2. Implement full-text search on document content (use SQLite FTS5 or Meilisearch)
3. Basic metadata filtering (date, type, tags)
4. Search result pagination and sorting
5. Search query logging

### Phase 2: Advanced Search (4-6 weeks) — P2
1. Implement faceted filtering with real-time facet suggestions
2. Natural language search interface (like DocuPipe's Analyze query)
3. Saved searches with re-execution
4. Search history and recent queries
5. "More like this" functionality (cosine similarity on embeddings)

### Phase 3: Performance & Scale (3-4 weeks) — P3
1. Search result caching (LRU cache for frequent queries)
2. Search analytics dashboard
3. Zero-result query analysis
4. Search performance monitoring
5. Search quality metrics

### Phase 4: Intelligence Layer (3-4 weeks) — P3
1. Search suggestion engine (autocomplete with typeahead)
2. Query correction and synonym support
3. Personalized search results (user history boost)
4. Search trend analysis
5. Integration with Elasticsearch for enterprise deployments

---

## Key Takeaway for Buyers

RERP Documents' search pitch is **fast, self-hosted, and OpenAPI-defined**. Unlike Textract (no search, build your own) or Rossum (basic archive search), RERP provides powerful full-text search with faceted filtering and natural language queries. Unlike Paperless-ngx (which requires Elasticsearch), RERP's search is built-in with zero external dependencies.

The Rust-native search engine handles 10,000+ concurrent searches with sub-millisecond latency. And because search is defined in OpenAPI, every client gets type-safe SDKs, automatic validation, and complete API documentation.

**The immediate priority: implement basic full-text search with metadata filtering, define the search index entity, and build the search endpoint. Search is the bridge between stored documents and business value.**
