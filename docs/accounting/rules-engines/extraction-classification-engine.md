# Extraction Classification Engine

Status: scaffold dossier, implementation deferred

Owner service: `documents-extraction`

Runtime gate: `documents-extraction` is contract-only until its Rust crates, Helm values, Dockerfile, workspace registration, and generated docs exist.

## Purpose

The extraction classification engine classifies accounting documents, extracts structured fields, assigns confidence, routes review, and links approved results to invoices, bills, bank statements, or EDI documents.

## Ownership

Owns:

- accounting document classification
- extraction jobs and results
- confidence and review state
- approval of extracted fields
- linkage to accounting records

Does not own:

- invoice posting
- AP approval
- bank statement reconciliation
- EDI transport
- raw storage infrastructure

## Initial Contract Anchors

- `/accounting-documents`
- `/accounting-documents/{id}/classify`
- `/extraction-jobs`
- `/extraction-results`
- `/documents/{id}/approve-extraction`
- `/documents/{id}/link-invoice`
- `/documents/{id}/link-bank-statement`

## Design Questions Before Implementation

- What confidence thresholds require human review?
- How are corrections versioned and fed back into model/rule tuning?
- How are duplicate documents detected?
- How is source file integrity preserved?

## Required BDD Slices

- Classify a document and explain the classification confidence.
- Extract invoice fields and route low-confidence fields to review.
- Approve corrected extraction results.
- Link an approved document to an invoice or bank statement.
