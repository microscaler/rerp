# analyze/ — LLM-Based Document Analysis

**Path:** `documents/analyze/`
**API:** `/api/v1/documents/analyze/`

Free-text Q&A about documents, powered by LLM with document context injection. Writes analysis results to `core/` metadata.

## API Surface

```
POST   /documents/{id}/analyze       # Ask question about document
GET    /documents/{id}/analysis      # Get analysis results
POST   /documents/analyze-bulk       # Ask question about multiple documents
```
