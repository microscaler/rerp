# Analytics & BI

## Overview

Business intelligence, dashboards, and reporting tools.

## Services

### Dashboards
- **Path**: `analytics/dashboards/`
- **Description**: Analytics service providing customizable dashboards and KPI tracking
- **Documentation**: [Dashboards README](./dashboards/README.md)
- **API Spec**: [Dashboards OpenAPI](./dashboards/openapi.yaml)

### Reporting
- **Path**: `analytics/reporting/`
- **Description**: Reporting service generating standard and custom reports
- **Documentation**: [Reporting README](./reporting/README.md)
- **API Spec**: [Reporting OpenAPI](./reporting/openapi.yaml)

### Bi
- **Path**: `analytics/bi/`
- **Description**: Business Intelligence service providing advanced analytics and data warehousing
- **Documentation**: [Bi README](./bi/README.md)
- **API Spec**: [Bi OpenAPI](./bi/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/analytics` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The analytics & bi services work together to provide complete functionality:

*Integration patterns specific to analytics & bi will be documented as services are implemented.*
