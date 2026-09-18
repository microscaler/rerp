# RERP Architecture Decision Register

This register is the authoritative inventory of RERP ADRs. See
[`DOCUMENTATION_GOVERNANCE.md`](../DOCUMENTATION_GOVERNANCE.md) for lifecycle,
authority, and supersession rules.

**Next ADR number:** `003`

## Status vocabulary

- `PROPOSED`: under review and non-normative;
- `ACCEPTED`: current normative decision;
- `SUPERSEDED`: replaced by a named later ADR; and
- `REJECTED`: considered and not adopted.

## Register

| ADR | Status | Scope | Decision |
|---|---|---|---|
| [001](./001-accounting-runtime-boundary.md) | Accepted | `accounting.runtime-boundary` | First invoice-to-GL slice remains atomic within the invoice implementation runtime |
| [002](./002-document-generation-ownership.md) | Accepted | `documents.generated-rendition-ownership` | Documents owns generic generated renditions; source suites own business facts |

## Creating an ADR

1. Copy [`000-ADR-template.md`](./000-ADR-template.md) to the next numbered file.
2. Add it here as `PROPOSED` and register it in `../authority.json`; every ADR
   is registered, including proposed, rejected, and superseded records.
3. Use one stable dot-separated scope.
4. Record alternatives and consequences before acceptance.
5. On acceptance, update the status here and in `authority.json`.
6. To change an accepted decision, create a new ADR and use reciprocal
   `Supersedes` and `Superseded by` metadata. Never rewrite the old outcome.
