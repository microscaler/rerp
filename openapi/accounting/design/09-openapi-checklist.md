# OpenAPI Design Checklist

> Part of RERP Accounting Suite Design
> See [main DESIGN.md](../DESIGN.md) for complete reference

---

## Pre-Implementation Checklist

Before starting work on any accounting microservice, ensure:

- [ ] OpenAPI spec written and reviewed
- [ ] All schemas defined with proper types
- [ ] Security scheme configured (`bearerAuth`)
- [ ] Parameters defined (`CompanyId`, `Page`, `Limit`, etc.)
- [ ] Error responses standardized (`400`, `401`, `403`, `409`)
- [ ] Pagination implemented (`PaginatedResponse` pattern)
- [ ] Implementation flags set (`x-brrtrouter-impl: true` on mutations)
- [ ] `brrtrouter-gen lint` passes
- [ ] Cross-service dependencies documented

## Design Principles

1. **Single Responsibility**: Each service owns one domain concept
2. **OpenAPI-First**: Spec before implementation
3. **Double-Entry**: All financial transactions balance
4. **Audit Trail**: All mutations logged
5. **Multi-Tenant**: Data isolated by company/tenant
6. **Idempotent**: Repeated requests produce same result

---

*End of design documentation.*
