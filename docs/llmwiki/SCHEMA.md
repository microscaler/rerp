# RERP LLM Wiki Schema

## Purpose

This wiki is a persistent knowledge layer for RERP agents. It stores reconciled, operational knowledge that should survive chat boundaries: code conventions, tooling contracts, known drift, and decisions that are easy for future agents to accidentally undo.

## Source of Truth Order

RERP distinguishes intended behaviour from delivered behaviour. Start at
[`docs/README.md`](../README.md) for the current human authority map.

For **normative truth**—what the product and architecture should do—use:

1. Accepted ADRs.
2. Approved modes of operation and PRDs.
3. Active suite/service designs.
4. Authoritative OpenAPI contracts and acceptance criteria.

For **delivered truth**—what the system does now—use:

1. Deployed runtime behaviour and persisted schema.
2. User-owned implementation and configuration under `tooling/`,
   `microservices/`, `Tiltfile`, Helm, and CI.
3. OpenAPI generator inputs and generated artifacts.
4. Tests and dated verification records.

When the two views disagree, preserve the normative authority and record
implementation drift. Code is evidence of delivery, not an implicit repeal of
an accepted decision. A design is evidence of intent, not proof of delivery.

This wiki is derived synthesis in both views. If it contradicts an authority or
current implementation evidence, update the wiki; it cannot override either.

## Layout

```text
docs/llmwiki/
├── SCHEMA.md
├── README.md
├── index.md
├── log.md
├── docs-catalog.md
├── topics/
├── entities/
└── reconciliation/
```

Keep `topics/`, `entities/`, and `reconciliation/` flat. Prefer updating an existing page over creating another near-duplicate.

## Page Conventions

Substantive pages start with:

```text
# <Title>

- **Status**: `verified` | `partially-verified` | `unverified`
- **Source docs**: repo-relative paths
- **Code anchors**: repo-relative paths
- **Last updated**: YYYY-MM-DD
```

Then include:

- **What it is**: short explanation.
- **Where it lives / how it works**: file anchors and key commands.
- **Gotchas / drift**: explicit failure modes and migration state.
- **Cross-references**: related wiki/source pages.

## Operations

### Ingest

When work changes code or authoritative docs:

1. Identify which wiki pages are affected.
2. Update or create only the relevant pages.
3. Update `index.md` for new pages.
4. Append a `log.md` entry: `## [YYYY-MM-DD] ingest | <short>`.
5. Mark contradictions with `> **Drift:**` or `> **Open:**` instead of burying them.

### Query

When answering architecture/tooling questions:

1. Read `index.md`.
2. Read relevant topic/entity pages.
3. Verify uncertain claims against code.
4. File durable findings back into the wiki.

### Session Close

At the end of meaningful work, update touched pages and append to `log.md`. This is part of keeping RERP recoverable across agent crashes and context compaction.
