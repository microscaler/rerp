# core/ — Canonical Document Storage

**Path:** `documents/core/`
**API:** `/api/v1/documents/core/`

`core/` is the **single source of truth** for all documents in RERP. Every document — regardless of intake channel, processing stage, or origin — is stored here. Other services don't store documents; they read from `core/`, process them, and write results back to `core/` as metadata.

## What `core/` Stores

- **Raw files** — The actual document (PDF, image, DOCX, etc.)
- **Metadata** — uploader, source channel (email/camera/webhook/drag-drop), tags, folder path, created timestamp
- **Versions** — Full version history with diff and rollback support
- **Processing results** — Extraction results, classification results, analysis results (written by other services)
- **Folder hierarchy** — Tree-structured organization with parent-child relationships
- **Search index** — Full-text search across all document content and metadata

## Key Capabilities

| Capability | Description |
|---|---|
| **Universal storage** | Accepts any file type (PDF, image, DOCX, XLSX, TXT, CSV, EML, MSG) |
| **Version control** | Every upload creates a version; full history with diff and rollback |
| **Folder hierarchy** | Tree-structured organization with parent-child relationships |
| **Full-text search** | Search across document content and metadata |
| **Metadata tagging** | System tags (auto-generated from processing) + user tags |
| **Download/preview** | Download original file, view OCR text, download processed files |
| **Version comparison** | Diff between any two versions |
| **Bulk operations** | Bulk download, bulk delete, bulk metadata update |
| **RLS policies** | Row-level security per-organization, per-user access |

## API Surface

```
POST   /documents                  # Upload/create document
GET    /documents                  # List documents (paginated, filterable)
GET    /documents/{id}             # Get document details + metadata
PUT    /documents/{id}             # Update document metadata/tags
DELETE /documents/{id}             # Delete document
POST   /documents/{id}/versions    # Create new version
GET    /documents/{id}/versions    # List all versions
GET    /documents/{id}/versions/{vid}  # Get specific version
PUT    /documents/{id}/versions/{vid}  # Update version metadata
DELETE /documents/{id}/versions/{vid}   # Delete version (keep others)
GET    /folders                    # List folder tree
POST   /folders                    # Create folder
GET    /folders/{id}               # Get folder details
PUT    /folders/{id}               # Update folder
DELETE /folders/{id}               # Delete folder
GET    /documents/search           # Full-text search
GET    /documents/{id}/download    # Download original file
GET    /documents/{id}/ocr         # Download OCR text
GET    /documents/{id}/status      # Get processing status
```
