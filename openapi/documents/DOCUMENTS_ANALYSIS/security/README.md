# Security & Compliance

> **Component:** Encryption at rest and in transit, role-based access control, audit logging, data retention policies, GDPR, HIPAA, FedRAMP, PII detection and redaction, and compliance reporting
> **Priority:** P3 — Critical for regulated industries (healthcare, finance, government)
> **DocuPipe Reference:** DocuPipe HIPAA compliance, ABBYY SOC2/TSP compliance, Azure Document Intelligence compliance, AWS Textract security

---

## The Pitch

**Buyer Question:** *Can I process sensitive documents — medical records, financial statements, legal contracts — with the assurance that data never leaves my infrastructure and every action is tracked for audit?*

In regulated industries, document processing isn't just about accuracy — it's about compliance. A buyer needs to know: *Where does my data go when it leaves my system? Who can see it? What happens if it's breached?* This component covers the security and compliance layer that protects documents throughout their lifecycle — from ingestion through processing, storage, review, and deletion.

---

## What This Component Does

1. **Encryption at Rest** — AES-256 encryption for all stored documents and metadata
2. **Encryption in Transit** — TLS 1.3 for all data in transit (API, storage, integrations)
3. **Role-Based Access Control (RBAC)** — Fine-grained permissions per document, per type, per user
4. **Audit Logging** — Immutable audit trail for every document action (view, extract, modify, delete)
5. **Data Retention Policies** — Automatic archival and deletion based on configured rules
6. **GDPR Compliance** — Right to be forgotten, data portability, consent management
7. **HIPAA Compliance** — PHI protection, breach notification, access controls
8. **FedRAMP Compliance** — Government-grade security controls
9. **PII Detection and Redaction** — Auto-detect and redact personally identifiable information
10. **Compliance Reporting** — Generate compliance reports for auditors
11. **Document Watermarks** — Add watermarks to sensitive documents for traceability
12. **Access Token Management** — Short-lived tokens for document access with expiration

---

## Entity Model

### SecurityPolicy Entity

Defines a security policy applied to documents:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Policy name (e.g., "HIPAA Document Protection") |
| `description` | Text | No | Policy description |
| `policy_type` | Enum: [GDPR, HIPAA, FEDRAMP, CUSTOM, INTERNAL] | Yes | Compliance framework |
| `encryption_required` | Boolean | Yes | Require encryption at rest |
| `encryption_algorithm` | String (32) | No | Encryption algorithm (AES-256-GCM) |
| `ttl_hours` | Integer | No | Time-to-live for document access tokens (0 = unlimited) |
| `require_audit_log` | Boolean | Yes | Require audit logging for all access |
| `allow_downloads` | Boolean | Yes | Allow document downloads |
| `allow_sharing` | Boolean | Yes | Allow document sharing |
| `require_mfa` | Boolean | No | Require MFA for document access |
| `is_active` | Boolean | Yes | Enable/disable policy |
| `created_at` | DateTime | Yes | Creation timestamp |

### AccessControlEntry Entity

Individual access control rules:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | Foreign Key: DocumentStore | Yes | Document this applies to |
| `entity_type` | Enum: [USER, ROLE, GROUP, TEAM] | Yes | Type of entity being granted access |
| `entity_id` | UUID | Yes | ID of the entity |
| `permission` | Enum: [READ, WRITE, DELETE, SHARE, ADMIN] | Yes | Permission level |
| `granted_by` | Foreign Key: User | Yes | Who granted this permission |
| `granted_at` | DateTime | Yes | When permission was granted |
| `expires_at` | DateTime | No | When permission expires (NULL = permanent) |

### AuditLog Entity

Immutable audit trail for all document actions:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | Foreign Key: DocumentStore | No | Related document (nullable for system events) |
| `action` | String (64) | Yes | Action performed (VIEW, EXTRACT, MODIFY, DELETE, SHARE, DOWNLOAD, EXPORT) |
| `actor_id` | Foreign Key: User | Yes | Who performed the action |
| `actor_type` | Enum: [USER, SYSTEM, API_KEY] | Yes | Type of actor |
| `source_ip` | String (45) | Yes | Source IP address |
| `user_agent` | String (512) | No | User agent string |
| `details` | JSON | No | Additional context (field names modified, etc.) |
| `timestamp` | DateTime | Yes | When action occurred |
| `compliance_frame` | Enum: [GDPR, HIPAA, FEDRAMP, NONE] | No | Applicable compliance framework |
| `is_tamper_proof` | Boolean | Yes | Cryptographic integrity flag |

### DataRetentionPolicy Entity

Defines automatic retention and deletion rules:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Policy name (e.g., "HIPAA Records 6 Years") |
| `description` | Text | No | Policy description |
| `scope` | Enum: [ALL, BY_TYPE, BY_CATEGORY, BY_USER] | Yes | Policy scope |
| `document_type_filter` | String (64) | No | Document type to apply to (NULL = all) |
| `retention_days` | Integer | Yes | Days to retain after processing |
| `action_on_expiry` | Enum: [DELETE, ARCHIVE, REDACT, ANONYMIZE] | Yes | Action when retention expires |
| `compliance_framework` | Enum: [GDPR, HIPAA, FEDRAMP, NONE] | Yes | Applicable framework |
| `is_active` | Boolean | Yes | Enable/disable policy |
| `created_at` | DateTime | Yes | Creation timestamp |

### ComplianceReport Entity

Generated compliance reports for auditors:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `report_type` | Enum: [AUDIT_TRAIL, ACCESS_LOG, DATA_SUBJECT_REQUEST, BREACH_REPORT, RETENTION_REPORT] | Yes | Report type |
| `compliance_framework` | Enum: [GDPR, HIPAA, FEDRAMP, NONE] | Yes | Applicable framework |
| `period_start` | DateTime | Yes | Report period start |
| `period_end` | DateTime | Yes | Report period end |
| `status` | Enum: [DRAFT, GENERATED, REVIEWED, ARCHIVED] | Yes | Report status |
| `generated_by` | Foreign Key: User | Yes | Who generated the report |
| `generated_at` | DateTime | Yes | Generation timestamp |
| `file_path` | String (1024) | No | Path to generated report file |
| `summary` | Text | No | Report summary |

### EncryptionKey Entity

Encryption key management:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Key name (e.g., "documents-primary") |
| `algorithm` | String (32) | Yes | Encryption algorithm (AES-256-GCM) |
| `key_length` | Integer | Yes | Key length in bits |
| `key_store` | Enum: [LOCAL, AWS_KMS, AZURE_KEY_VAULT, HASHICORP_VAULT] | Yes | Where the key is stored |
| `key_reference` | String (1024) | Yes | Reference to the key in the key store |
| `is_active` | Boolean | Yes | Is this the active key? |
| `created_at` | DateTime | Yes | Key creation timestamp |
| `rotated_at` | DateTime | No | Last rotation timestamp |
| `expires_at` | DateTime | No | When key expires |

### PIIField Entity

PII field definitions for detection and redaction:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `field_name` | String (128) | Yes | Field name to scan (e.g., "email", "ssn", "phone") |
| `pii_type` | Enum: [EMAIL, PHONE, SSN, CREDIT_CARD, IP_ADDRESS, NAME, ADDRESS, MEDICAL_RECORD, DRIVERS_LICENSE, PASSPORT] | Yes | Type of PII |
| `detection_pattern` | String (512) | Yes | Regex pattern for detection |
| `redaction_method` | Enum: [MASK, HASH, REMOVE, TOKENIZE] | Yes | How to redact detected PII |
| `redaction_template` | String (255) | No | Redaction template (e.g., "***-**-XXXX" for SSN) |
| `is_active` | Boolean | Yes | Enable/disable this PII type |
| `created_at` | DateTime | Yes | Creation timestamp |

---

## Entity Relationships

```
SecurityPolicy (applied to documents)
  ├── AccessControlEntry (via document_id filter)      ← access rules per document
  ├── AuditLog (via document_id + compliance_frame)    ← audit trails per policy
  └── DataRetentionPolicy (via compliance_framework)   ← retention rules per policy

AccessControlEntry
  ├── DocumentStore (via document_id)                   ← document being protected
  ├── User/Role/Group (via entity_type + entity_id)     ← entity with access
  ├── User (via granted_by)                             ← who granted access
  └── User (via actor_id in AuditLog when action taken) ← audit trail

AuditLog
  ├── DocumentStore (via document_id)                   ← document accessed
  └── User (via actor_id)                               ← who performed action

DataRetentionPolicy
  ├── DocumentStore (via policy filter)                 ← documents subject to policy
  └── ComplianceReport (via compliance_framework)       ← reports generated from policy

ComplianceReport
  ├── AuditLog (via period range + compliance_frame)    ← source data for report
  └── User (via generated_by)                           ← who generated report

EncryptionKey
  └── DocumentStore (via key_id filter)                 ← documents encrypted with this key

PIIField
  └── AuditLog (via detection in document content)      ← when PII was detected
```

---

## Required API Endpoints

### Security Policies

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/security/policies` | List all security policies |
| `GET` | `/security/policies/{id}` | Get policy detail |
| `POST` | `/security/policies` | Create security policy |
| `PATCH` | `/security/policies/{id}` | Update security policy |
| `DELETE` | `/security/policies/{id}` | Delete security policy |
| `POST` | `/security/policies/{id}/apply` | Apply policy to documents |

### Access Control

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/security/access/{document_id}` | List access control entries for a document |
| `POST` | `/security/access/{document_id}` | Grant access to a document |
| `DELETE` | `/security/access/{document_id}/{entry_id}` | Revoke access |
| `GET` | `/security/access/user/{user_id}` | List all documents a user can access |
| `POST` | `/security/verify/{document_id}` | Verify user has access to a document |

### Audit Logging

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/security/audit` | Search audit log |
| `GET` | `/security/audit/{id}` | Get specific audit entry |
| `GET` | `/security/audit/document/{document_id}` | Audit trail for a document |
| `GET` | `/security/audit/user/{user_id}` | Audit trail for a user |
| `GET` | `/security/audit/export` | Export audit log (CSV) |
| `POST` | `/security/audit/tamper-check` | Verify audit log integrity |

### Data Retention

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/security/retention/policies` | List all retention policies |
| `POST` | `/security/retention/policies` | Create retention policy |
| `PATCH` | `/security/retention/policies/{id}` | Update retention policy |
| `DELETE` | `/security/retention/policies/{id}` | Delete retention policy |
| `POST` | `/security/retention/enforce` | Run retention enforcement |
| `GET` | `/security/retention/compliance` | Retention compliance status |

### GDPR Compliance

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/security/gdpr/right-to-be-forgotten` | Delete all data for a data subject |
| `POST` | `/security/gdpr/data-portability` | Export all data for a data subject |
| `POST` | `/security/gdpr/consent` | Record or withdraw consent |
| `GET` | `/security/gdpr/consent/audit` | Consent audit trail |

### PII Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/security/pii/fields` | List all PII field definitions |
| `POST` | `/security/pii/fields` | Create PII field definition |
| `PATCH` | `/security/pii/fields/{id}` | Update PII field definition |
| `DELETE` | `/security/pii/fields/{id}` | Delete PII field definition |
| `POST` | `/security/pii/scan/{document_id}` | Scan document for PII |
| `POST` | `/security/pii/redact/{document_id}` | Redact PII from document |

### Encryption Keys

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/security/keys` | List all encryption keys |
| `POST` | `/security/keys` | Register encryption key |
| `PATCH` | `/security/keys/{id}` | Update encryption key |
| `POST` | `/security/keys/{id}/rotate` | Rotate encryption key |
| `POST` | `/security/keys/{id}/deactivate` | Deactivate encryption key |

### Compliance Reporting

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/security/reports` | List all compliance reports |
| `POST` | `/security/reports/generate` | Generate compliance report |
| `GET` | `/security/reports/{id}` | Get report detail |
| `POST` | `/security/reports/{id}/download` | Download report file |
| `GET` | `/security/reports/audit-trail` | Full audit trail export |

---

## Competitive Positioning

### Where RERP Wins

- **Self-hosted, no data egress** — Unlike every competitor, data NEVER leaves your infrastructure. No HIPAA concerns about sending medical records to AWS, Google, or DocuPipe.
- **OpenAPI-defined security policies** — Every policy, permission, and rule is machine-readable. Automated compliance checking is possible.
- **Rust-level audit log performance** — Writing and querying 1 million audit entries in Rust is instantaneous.
- **Zero marginal cost at scale** — 1 million audit log entries cost the same as 10K.
- **Full control over encryption keys** — BYOK (Bring Your Own Key) to any KMS (AWS KMS, Azure Key Vault, HashiCorp Vault).

### Where RERP Lags

- **No security policies deployed** — No RBAC, no audit logging, no encryption at rest.
- **No GDPR compliance** — No right-to-be-forgotten or data portability endpoints.
- **No HIPAA compliance** — No PHI protection, no breach notification.
- **No PII detection** — No auto-detection and redaction of personally identifiable information.
- **No encryption key management** — No key rotation, no KMS integration.

---

## Competitive Intelligence Deep Dive

### DocuPipe — HIPAA-Compliant Cloud Processing

DocuPipe positions itself as a HIPAA-compliant document intelligence platform. All data is encrypted in transit (TLS 1.3) and at rest (AES-256). DocuPipe offers customizable data retention policies and provides a data processing agreement (DPA) for HIPAA-covered entities. The key advantage is ease of use — no infrastructure to manage. The key disadvantage is that data DOES leave your infrastructure and resides in DocuPipe's cloud. For healthcare organizations that can't send PHI outside their network, this is a deal-breaker. Pricing: $0.001/request (freemium available).

### ABBYY FlexiCapture — Enterprise-Grade Compliance

ABBYY is SOC 2 Type 1 certified and meets TSP Section 100 & AICPA Trust Services Criteria. It supports HIPAA, GDPR, and FedRAMP compliance. Key features: AES-256 encryption at rest and in transit, role-based access control, immutable audit logging, configurable data retention, and encrypted operator exchange. ABBYY supports on-premises deployment (data never leaves your infrastructure) as well as cloud (Azure) and SDK. The key differentiator is proven enterprise compliance at scale — 10,000+ enterprise customers. Cost: part of $100K+ license.

### Azure Document Intelligence — Microsoft Compliance

Azure Document Intelligence inherits Azure's compliance certifications: HIPAA, GDPR, FedRAMP, ISO 27001, SOC 1/2/3, and more. Data is encrypted at rest (AES-256) and in transit (TLS 1.3). Azure's Key Vault provides key management with automatic rotation. The key advantage is tight Azure ecosystem integration — Key Vault, Azure Monitor, Azure Policy. However, data is processed in Azure data centers, which may not be acceptable for some regulated industries. Pricing: $0.01-$0.03/page.

### AWS Textract — AWS Security Infrastructure

AWS Textract inherits AWS's security infrastructure: encryption at rest (S3 default encryption), encryption in transit (TLS 1.3), KMS for key management, CloudTrail for audit logging, and AWS IAM for access control. AWS is HIPAA-eligible (but requires a BAA) and supports GDPR, FedRAMP, SOC 1/2/3. The key advantage is deep AWS integration — Textract outputs can be stored directly in S3 with encryption, and audit trails go directly to CloudTrail. However, data is processed in AWS data centers. Pricing: $0.0006/page.

---

## Implementation Roadmap

### Phase 1: Audit Logging Foundation (2-3 weeks) — P3

1. Define `AuditLog` entity: id, document_id, action, actor_id, actor_type, source_ip, details, timestamp, compliance_frame
2. Implement audit log writing at every document action (ingest, extract, view, modify, delete)
3. Implement audit log search endpoint (GET /security/audit)
4. Implement document-level audit trail endpoint
5. Add tamper-proof integrity verification (hash chain for audit entries)
6. Implement CSV audit log export

### Phase 2: RBAC & Access Control (2-3 weeks) — P3

1. Define `AccessControlEntry` entity
2. Implement access control entry CRUD endpoints
3. Implement permission verification endpoint (POST /security/verify)
4. Implement document access listing (by user and by document)
5. Add role-based permission groups (admin, viewer, editor, reviewer)

### Phase 3: Data Retention (2-3 weeks) — P3

1. Define `DataRetentionPolicy` entity
2. Implement retention policy CRUD endpoints
3. Implement retention enforcement cron job (DELETE/ARCHIVE/REDACT/ANONYMIZE)
4. Implement retention compliance status endpoint
5. Add automatic anonymization for GDPR right-to-be-forgotten

### Phase 4: GDPR & PII Compliance (3-4 weeks) — P3

1. Implement GDPR right-to-be-forgotten endpoint (delete all data for a data subject)
2. Implement GDPR data portability endpoint (export all data for a data subject)
3. Define `PIIField` entity with detection patterns
4. Implement PII detection endpoint (POST /security/pii/scan)
5. Implement PII redaction endpoint (POST /security/pii/redact)
6. Add consent management (record and withdraw consent)

### Phase 5: Encryption & Compliance Reporting (4-6 weeks) — P3

1. Define `SecurityPolicy` and `EncryptionKey` entities
2. Implement encryption key management (register, rotate, deactivate)
3. Implement AES-256 encryption at rest for documents and metadata
4. Implement TLS 1.3 enforcement for all API and storage endpoints
5. Define `ComplianceReport` entity
6. Implement automated compliance report generation (audit trail, access log, breach report)
7. Add document watermarking for traceability

---

## Key Takeaway for Buyers

Security and compliance is the foundation of trust in document processing. A buyer needs to know: *Can I process sensitive documents with the assurance that data never leaves my infrastructure and every action is tracked for audit?* RERP's advantage is self-hosted deployment with zero data egress — your medical records stay in your data center, your financial statements never touch AWS or Google, and every document action is logged with tamper-proof integrity. The immediate priority: define the AuditLog entity, implement audit writing at every document action, and build the audit log search endpoint. Everything else builds on this foundation.
