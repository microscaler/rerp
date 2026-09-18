# RERP Documentation Governance

- **Status**: ACTIVE
- **Authority**: Normative
- **Owner**: RERP maintainers
- **Scope**: repository.documentation-governance
- **Last reviewed**: 2026-07-15
- **Supersedes**: None
- **Superseded by**: None

## Purpose

RERP retains design history without allowing obsolete documents to masquerade
as current product intent. This policy defines document classes, lifecycle
states, authority, supersession, external-source handling, and automated
validation.

## Authority is not implementation evidence

RERP maintains two complementary views:

- **Normative truth** defines intended behaviour through accepted ADRs,
  approved operating models and PRDs, active designs, public contracts, and
  acceptance criteria.
- **Delivered truth** describes current behaviour through the deployed runtime,
  persistence schema, implementation, generated contracts, tests, and dated
  verification reports.

Conflicts are explicit drift. The implementation is evidence of what happens;
it does not silently supersede an accepted decision. A design is evidence of
intent; it does not prove delivery.

## Controlled lifecycle states

### ADRs

| Status | Meaning |
|---|---|
| `PROPOSED` | Under discussion and not normative |
| `ACCEPTED` | Current normative decision |
| `SUPERSEDED` | Replaced by a named later ADR; retained as history |
| `REJECTED` | Considered and deliberately not adopted |

### PRDs, modes of operation, and designs

| Status | Meaning |
|---|---|
| `DRAFT` | Incomplete and non-normative |
| `IN_REVIEW` | Review candidate and non-normative |
| `APPROVED` | Current normative product/design authority |
| `IMPLEMENTING` | Approved and actively being delivered |
| `DELIVERED` | Approved acceptance criteria verified against delivery |
| `SUPERSEDED` | Replaced by a named authority |
| `ABANDONED` | Work deliberately stopped and non-normative |

### Roadmaps, analyses, audits, and status reports

| Status | Meaning |
|---|---|
| `PROPOSED` | Working sequence awaiting approval |
| `ACTIVE` | Current execution plan or maintained reference |
| `CURRENT_ANALYSIS` | Dated analysis still considered relevant |
| `HISTORICAL_SNAPSHOT` | Preserved evidence about a prior state |
| `SUPERSEDED` | Replaced by a named document |
| `ABANDONED` | No longer pursued |

Age does not automatically invalidate an accepted decision. `Last reviewed`
supports review scheduling, not silent expiry.

## Required metadata

Every new ADR and every document registered as current authority carries:

```markdown
- **Status**: APPROVED
- **Authority**: Normative
- **Owner**: Owning team or role
- **Scope**: stable.dot.separated.scope
- **Last reviewed**: YYYY-MM-DD
- **Supersedes**: None or ADR/document identifier
- **Superseded by**: None or ADR/document identifier
```

ADRs also carry their decision date and decision owners. External authority
records additionally carry the repository, path, and reviewed revision.

Allowed authority values are:

- `Normative`: approved definition of intended behaviour;
- `Working`: active planning or review input that has not been approved; and
- `Informative`: research, history, or implementation evidence.

## ADR policy

ADRs capture decisions, not whole requirements documents. Once accepted, the
decision, context, alternatives, and consequences are immutable. Typographical
repairs, broken links, and explicit clarification may be added without changing
the outcome.

A reversal or material change requires a new ADR:

1. Allocate the next ID from `docs/adrs/README.md`.
2. Set the new ADR's `Supersedes` field.
3. Set the old ADR to `SUPERSEDED` and its `Superseded by` field.
4. Add reciprocal links in both documents and the ADR register.
5. Update `docs/authority.json` so only the new ADR is current for the scope.

Rejected alternatives stay in the ADR. They explain why a superficially easy
solution must not be repeatedly reintroduced.

## Retiring other documents

Do not silently delete, rename, or move a referenced document merely because it
is stale. Add a banner at its beginning:

```markdown
> **Status: SUPERSEDED**
>
> Retained as historical context. Current authority:
> [replacement title](replacement-path). Superseded on YYYY-MM-DD.
```

The replacement reciprocally identifies what it supersedes. A historical
analysis should use `HISTORICAL_SNAPSHOT` and retain its `as-of` date. It does
not need a replacement unless callers could reasonably treat it as current.

Files may move to an archive only after inbound links are corrected and the
authority index no longer depends on their original paths. Git history alone
is not the documentation archive.

## External product authorities

Dogfood systems such as Hauliage own their commercial operating models. RERP
must not copy those models and then allow the copy to drift.

An RERP document deriving requirements from an external product records:

- repository and document path;
- reviewed commit, tag, or approved document version;
- review date;
- relevant decisions and assumptions;
- the RERP ADRs or PRDs derived from it.

A moving `main` branch link is useful navigation but is not a reproducible
review reference. During joint drafting, mark the dependency as working and
refresh the revision before approval.

## Current-authority registry

`docs/authority.json` is the machine-readable registry. It is deliberately
small and contains only documents that define current authority or active
working gates. It is not a catalog of every Markdown file.

Each entry has a stable ID, scope, path, class, status, authority, owner,
review date, and reciprocal supersession identifiers. More than one historical
entry may share a scope, but only one current normative entry may do so.

`docs/README.md` is the human-readable view. The ADR register at
`docs/adrs/README.md` lists every ADR, including rejected and superseded records.

## Automated checks

`python tooling/scripts/check_doc_governance.py` validates:

- registry schema and required fields;
- allowed statuses and authority values;
- repository-relative targets and ISO review dates;
- unique entry IDs;
- no competing current normative documents for one scope;
- reciprocal supersession references;
- registration and metadata of every non-template ADR; and
- ADR register coverage.

The validator is intentionally incremental. It does not declare the existing
legacy document tree current merely because a file exists, and it does not
require a disruptive mass rewrite before new governance can be enforced.

## Pull-request obligations

Reviewers must ask:

1. Does this change alter an accepted decision or active requirement?
2. Does it create drift between normative and delivered truth?
3. Does a replaced document carry a reciprocal supersession link?
4. Does a newly accepted concept need an ADR or authority-registry entry?
5. Are suite/service READMEs and OpenAPI sources aligned?
6. Was the affected LLM wiki synthesis updated after the authority changed?
