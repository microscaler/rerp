# Inventory Management

## Overview

Complete inventory management from stock tracking to warehouse operations and logistics.

## Services

### Core
- **Path**: `inventory/core/`
- **Description**: Inventory management service tracking stock levels and movements
- **Documentation**: [Core README](./core/README.md)
- **API Spec**: [Core OpenAPI](./core/openapi.yaml)

### Warehouse
- **Path**: `inventory/warehouse/`
- **Description**: Warehouse operations service managing multi-warehouse setups
- **Documentation**: [Warehouse README](./warehouse/README.md)
- **API Spec**: [Warehouse OpenAPI](./warehouse/openapi.yaml)

### Logistics
- **Path**: `inventory/logistics/`
- **Description**: Logistics and shipping service integrating with carriers
- **Documentation**: [Logistics README](./logistics/README.md)
- **API Spec**: [Logistics OpenAPI](./logistics/openapi.yaml)

### Dropshipping
- **Path**: `inventory/dropshipping/`
- **Description**: Dropshipping service managing vendor dropship orders
- **Documentation**: [Dropshipping README](./dropshipping/README.md)
- **API Spec**: [Dropshipping OpenAPI](./dropshipping/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/inventory` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The inventory management services work together to provide complete functionality:

*Integration patterns specific to inventory management will be documented as services are implemented.*
