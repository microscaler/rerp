# routes/ — Routing Rules Engine

**Path:** `documents/routes/`
**API:** `/api/v1/documents/routes/`

Defines where classified and extracted documents should be routed within RERP. Pure configuration — does not store or process documents.

## Routing Rule Fields

```yaml
id: string (auto-generated)
name: string (human-readable)
document_type: string (classified document type)
source: list (email, camera, webhook, upload)
target_module: string (RERP module name)
target_endpoint: string (API endpoint to create/update record)
schema_id: string (extraction schema to use)
auto_approve: boolean
auto_approve_threshold: float (confidence score, 0.0-1.0)
notify: list (user roles or specific user IDs)
fallback_route: string (what to do if extraction confidence < threshold)
retry_count: int (max retries on API call failure)
retry_delay: int (seconds between retries)
```

## API Surface

```
POST   /routes                        # Create routing rule
GET    /routes                        # List all routing rules
GET    /routes/{id}                   # Get routing rule details
PUT    /routes/{id}                   # Update routing rule
DELETE /routes/{id}                   # Delete routing rule
POST   /routes/{id}/test              # Test routing rule with sample document
GET    /routes/by-document-type       # Get routes for a document type
POST   /routes/import                 # Bulk import routing rules from YAML/JSON
GET    /routes/export                 # Export all routing rules
```
