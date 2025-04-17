# Product Management

## Overview

Product catalog, pricing, and tax calculation services.

## Services

### Catalog
- **Path**: `product/catalog/`
- **Description**: Comprehensive product catalog managing SKUs, variants, attributes, and hierarchies
- **Documentation**: [Catalog README](./catalog/README.md)
- **API Spec**: [Catalog OpenAPI](./catalog/openapi.yaml)

### Pricing
- **Path**: `product/pricing/`
- **Description**: Dynamic pricing engine supporting multiple price lists and volume discounts
- **Documentation**: [Pricing README](./pricing/README.md)
- **API Spec**: [Pricing OpenAPI](./pricing/openapi.yaml)

### Tax
- **Path**: `product/tax/`
- **Description**: Tax calculation service handling complex tax rules and country-specific logic
- **Documentation**: [Tax README](./tax/README.md)
- **API Spec**: [Tax OpenAPI](./tax/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/product` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The product management services work together to provide complete functionality:

*Integration patterns specific to product management will be documented as services are implemented.*
