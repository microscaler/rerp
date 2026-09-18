# Website & E-commerce

## Overview

Website builder, content management, and e-commerce storefront.

## Services

### Builder
- **Path**: `website/builder/`
- **Description**: Website builder service providing drag-and-drop page builder
- **Documentation**: [Builder README](./builder/README.md)
- **API Spec**: [Builder OpenAPI](./builder/openapi.yaml)

### Ecommerce
- **Path**: `website/ecommerce/`
- **Description**: E-commerce service managing online store and shopping cart
- **Documentation**: [Ecommerce README](./ecommerce/README.md)
- **API Spec**: [Ecommerce OpenAPI](./ecommerce/openapi.yaml)

### Cms
- **Path**: `website/cms/`
- **Description**: Content management service for creating and managing website content
- **Documentation**: [Cms README](./cms/README.md)
- **API Spec**: [Cms OpenAPI](./cms/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/website` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The website & e-commerce services work together to provide complete functionality:

*Integration patterns specific to website & e-commerce will be documented as services are implemented.*
