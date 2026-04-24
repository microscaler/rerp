# Sibling Repos and Wikis

> How RERP relates to sibling repos: Hauliage, Lifeguard, BRRTRouter.

**Status:** partially-verified

## Sibling Repos

| Repo | Role | Wiki Location |
|------|------|---------------|
| `../brrtrouter/` | HTTP router framework | `../brrtrouter/llmwiki/` |
| `../lifeguard/` | ORM + migrations | `../lifeguard/docs/llmwiki/` |
| `../hauliage/` | Primary HTTP consumer + BFF | `../hauliage/docs/llmwiki/` |

## Responsibility Split

- **BRRTRouter**: HTTP routing, request dispatch, OpenAPI-based codegen
- **Lifeguard**: ORM entity definitions, migration generation, DB access
- **Hauliage**: BFF implementation, service composition, frontend proxy (legacy)
- **RERP**: Microservice business logic, OpenAPI specs, orchestration

## Cross-Repo Concepts

- RERP services use BRRTRouter for HTTP routing
- RERP entities use Lifeguard for ORM
- RERP BFF (planned) would use concepts from Hauliage's BFF pattern

## Wiki Navigation

From RERP wiki, link to sibling wikis:
- BRRTRouter: `../../brrtrouter/llmwiki/`
- Lifeguard: `../../lifeguard/docs/llmwiki/`
- Hauliage: `../../hauliage/docs/llmwiki/`
