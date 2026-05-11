# Security & Compliance

> **Component:** Security controls, data encryption, compliance certifications, and audit logging
> **Priority:** P3 — Critical for enterprise adoption and regulated industries
> **DocuPipe Reference:** SOC 2 Type II, ISO 27001, GDPR, HIPAA compliant, AES-256 at rest, TLS 1.3 in transit
> **Hyperscience Reference:** FedRAMP High authorized, data masking/redaction, AI-in-the-Loop governance

---

## The Pitch

**Buyer Question:** *Can I trust my documents with your platform — with encryption, access controls, audit trails, and compliance certifications — or will I face regulatory penalties and data breaches?*

If the answer is no, you don't have a document platform — you have a liability. Security and compliance are not features; they are prerequisites. In regulated industries (healthcare, finance, government), a single compliance failure can shut down an organization. Security is the foundation that enables everything else. This component defines how data is protected, how access is controlled, how operations are audited, and how compliance is maintained.

---

## What This Component Does

Security & Compliance is the protection layer:

1. **Encryption at Rest** — AES-256 encryption for all stored documents and data
2. **Encryption in Transit** — TLS 1.3 for all API communication
3. **Access Control** — Role-based and attribute-based access control (RBAC/ABAC)
4. **Audit Logging** — Complete audit trail of all document operations
5. **Compliance Certifications** — SOC 2, ISO 27001, HIPAA, GDPR, FedRAMP
6. **Data Residency** — Control over where data is stored geographically
7. **Multi-Factor Authentication** — MFA for all user accounts
8. **Data Retention** — Configurable retention and deletion policies
9. **Right to Be Forgotten** — GDPR-compliant data deletion

---

## Entity Model

### Audit Log Entity

The single source of truth for all security operations. Every action on every document is logged here.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `user_id` | UUID | No | Acting user (nullable for system/API actions) |
| `action` | String (128) | Yes | Action performed (e.g., document.uploaded, extraction.completed) |
| `resource_type` | String (64) | Yes | Resource type (Document, Extraction, Workflow) |
| `resource_id` | String (255) | Yes | Resource identifier |
| `ip_address` | String (45) | Yes | Client IP address (IPv4 or IPv6) |
| `user_agent` | String (500) | No | Client user agent string |
| `metadata` | JSONB | No | Additional context (old values, new values, changes) |
| `severity` | Enum: [INFO, WARNING, ERROR, CRITICAL] | No | Audit severity |
| `created_at` | DateTime | Yes | Timestamp (auto-set, immutable) |

### Access Control Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `user_id` | UUID | No | User (one of user_id or team_id) |
| `team_id` | UUID | No | Team (one of user_id or team_id) |
| `resource_type` | String (64) | Yes | Resource type (Document, Storage, Schema, etc.) |
| `resource_id` | UUID | Yes | Resource identifier |
| `permission` | Enum: [READ, WRITE, ADMIN, DELETE] | Yes | Permission level |
| `granted_by` | UUID | No | Who granted access (auditable) |
| `expires_at` | DateTime | No | Access expiration |
| `created_at` | DateTime | Yes | Creation timestamp |

### Compliance Framework Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `framework` | String (64) | Yes | Compliance framework (SOC2, ISO27001, HIPAA, GDPR, FedRAMP) |
| `status` | Enum: [PENDING, IN_PROGRESS, COMPLIANT, NON_COMPLIANT] | Yes | Compliance status |
| `last_audit` | DateTime | No | Last audit date |
| `next_audit` | DateTime | No | Next scheduled audit |
| `evidence` | JSONB | No | Compliance evidence (checklist results) |
| `created_at` | DateTime | Yes | Creation timestamp |
| `updated_at` | DateTime | Yes | Last update |

### Data Redaction Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | UUID | Yes | Source document |
| `pattern_type` | String (64) | Yes | Pattern type (SSN, EMAIL, CREDIT_CARD, PHI) |
| `pattern_regex` | String (255) | Yes | Regex pattern to match |
| `occurrences` | Integer | Yes | Number of matches found |
| `redacted_at` | DateTime | Yes | When redaction applied |
| `redaction_method` | Enum: [BLACKOUT, REPLACE, HASH, TOKENIZE] | Yes | How PII was redacted |

---

## Entity Relationships

```
Audit Log (central security record)
  ├── Document (many-to-one)         ← via resource_id (when resource_type=Document)
  ├── Extraction (many-to-one)       ← via resource_id (when resource_type=Extraction)
  └── User (many-to-one)             ← via user_id (acting user)

Access Control
  ├── User (many-to-one)             ← via user_id
  ├── Team (many-to-one)             ← via team_id
  └── Document (many-to-one)         ← via resource_id (when resource_type=Document)

Compliance Framework
  └── Audit Log (one-to-many)        ← linked by framework audits

Data Redaction
  └── Document (many-to-one)         ← via document_id
```

---

## Hyperscience Technical Patterns to Follow

### Pattern 1: Data Redaction and Masking

Hyperscience provides built-in data redaction for sensitive information across documents. This is critical for regulated industries (healthcare, finance, government) where PII and PHI must be protected. Redaction methods include blackout (visual removal), replace (substitution), hash (cryptographic hash), and tokenize (replace with token that maps back to original).

**Recommendation: RERP should implement data redaction as a configurable step in the processing pipeline.** When a document is ingested, scan for sensitive patterns (SSN, email, credit card, PHI). Apply redaction based on user-defined rules. Log all redactions in the audit trail. This ensures compliance with GDPR, HIPAA, and other regulations.

### Pattern 2: AI-in-the-Loop Governance

Hyperscience's AI-in-the-Loop architecture includes governance at every stage. Every AI decision is logged, every correction is tracked, and every model change is versioned. This creates a complete audit trail that can be reviewed by compliance officers.

**Recommendation: RERP should implement governance logging at every processing stage.** Every OCR result, extraction, classification, and workflow decision should be logged in the audit trail with the model version, confidence score, and user corrections. This creates an auditable chain of custody for every document.

---

## Competitive Intelligence Deep Dive

### DocuPipe: Strong Security Posture
DocuPipe holds SOC 2 Type II and ISO 27001 certifications. GDPR and HIPAA compliant. AES-256 encryption at rest, TLS 1.3 in transit. Trust center available at docupipe.ai/security. No data residency options mentioned. No self-hosted option — security model is cloud-only.

**Key strengths:** SOC 2 Type II, ISO 27001, HIPAA, GDPR compliant
**Key weaknesses:** Cloud-only, no data residency, no self-hosted

### AWS Textract: Enterprise-Grade Security
Textract inherits AWS's security posture: SOC 1, SOC 2, SOC 3, ISO 27001, ISO 27017, ISO 27018, PCI DSS, HIPAA eligible. Encryption at rest (AWS KMS) and in transit (TLS 1.2+). Data residency options through AWS Region selection. FedRAMP authorized. The security model is comprehensive but complex — you must configure encryption, IAM policies, and VPC endpoints yourself.

**Key strengths:** Comprehensive certifications, KMS encryption, FedRAMP
**Key weaknesses:** Complex configuration, AWS lock-in, per-request costs

### Rossum: Multi-Certification Enterprise
Rossum holds ISO 27001, SOC 2, HIPAA, and TX-RAMP certifications. Models built completely in-house (no third-party data leaks). Data protection processes only user-defined data points. Email security enforced via DMARC checks. Compliance is a core selling point — enterprise buyers require these certifications.

**Key strengths:** ISO 27001, SOC 2, HIPAA, TX-RAMP, in-house models
**Key weaknesses:** Enterprise-only, no self-hosted option

### Hyperscience: FedRAMP High
Hyperscience is FedRAMP High authorized — the highest level of cloud security certification, suitable for government work with sensitive data. SOC 2 Type II compliant. ISO 27001 certified. Built-in data masking/redaction. AI-in-the-Loop governance with complete audit trails. Supports AWS, Google, Azure, on-premises, and air-gapped deployments.

**Key strengths:** FedRAMP High, redaction/masking, multi-deployment
**Key weaknesses:** Enterprise-only, no self-hosted option

### Paperless-ngx: Privacy-First Open Source
Paperless-ngx stores all data locally on your server — never transmitted or shared. GDPR compliant by design (data stays on your infrastructure). No cloud dependencies means no third-party data exposure. The open-source model allows security audits by anyone. Community-driven security with regular vulnerability patches.

**Key strengths:** Complete data sovereignty, open-source security audits, GDPR by design
**Key weaknesses:** Self-managed security, no certifications, no enterprise features

---

## Competitive Positioning

### Where RERP Wins
- **Self-hosted, full data sovereignty** — Unlike cloud-only solutions (DocuPipe, Rossum, Hyperscience), RERP gives you complete control over your security infrastructure
- **Open-source, auditable by anyone** — Unlike proprietary solutions, RERP's security can be independently audited by security teams
- **Compliance-ready architecture** — Unlike Paperless-ngx (no certifications), RERP is built with SOC 2, ISO 27001, HIPAA, and GDPR compliance as design requirements

### Where RERP Lags
- **No security certifications** — Not yet implemented
- **No audit logging** — Not yet implemented
- **No data redaction** — Not yet implemented

---

## Implementation Roadmap

### Phase 1: Basic Security (3-4 weeks) — P3
1. Define `Audit Log` entity with immutable timestamps
2. Implement audit logging middleware (intercepts all CRUD operations)
3. AES-256 encryption for stored documents
4. TLS 1.3 for API communication
5. Basic access control (user-level permissions: read/write)
6. IP address logging on all API requests

### Phase 2: Advanced Security (4-6 weeks) — P3
1. Role-based access control (RBAC) with roles: admin, editor, viewer
2. Attribute-based access control (ABAC) — permissions based on document type, team, etc.
3. Multi-factor authentication (MFA) for all user accounts
4. API key rotation and revocation endpoints
5. IP allowlisting for API access
6. Audit log export (CSV, PDF) for compliance review

### Phase 3: Compliance Framework (4-6 weeks) — P4
1. Define `Compliance Framework` entity with framework-specific checklists
2. Implement compliance evidence collection (automated checks)
3. Compliance reporting (SOC 2, ISO 27001, HIPAA templates)
4. Data retention policies with automated archival/deletion
5. Right to be forgotten implementation (GDPR data deletion)
6. Data portability (GDPR export)

### Phase 4: Enterprise Security (3-4 weeks) — P4
1. Data residency configuration (select region for storage)
2. SSO/SAML integration for enterprise authentication
3. Advanced audit log analytics (trend analysis, anomaly detection)
4. Security incident response workflows
5. Compliance certification preparation (SOC 2 Type II audit support)
6. Data redaction for PII/PHI patterns (SSN, email, credit card)

---

## Key Takeaway for Buyers

RERP Documents' security pitch is **self-hosted, open-source, and compliance-ready**. Unlike cloud-only solutions (DocuPipe, Rossum, Hyperscience) where security is vendor-dependent, RERP gives you complete control over your security infrastructure. Unlike Textract (complex AWS security configuration), RERP provides out-of-the-box security with audit logging and access control.

The Rust-native security model provides memory-safe encryption and access control with zero risk of buffer overflows or memory corruption vulnerabilities. And because security is defined in OpenAPI, every client gets type-safe SDKs, automatic validation, and complete API documentation for security operations.

**The immediate priority: implement audit logging, define the audit log entity, and build the logging middleware. Security is the foundation — without it, nothing else matters.**
