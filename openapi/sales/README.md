# Sales Services

## Overview

Complete sales management from quotations to orders, subscriptions, and loyalty programs.

## Services

### Core
- **Path**: `sales/core/`
- **Description**: Unified sales service orchestrating quotations, orders, and sales workflows
- **Documentation**: [Core README](./core/README.md)
- **API Spec**: [Core OpenAPI](./core/openapi.yaml)

### Quotation
- **Path**: `sales/quotation/`
- **Description**: Quotation management service handling quote creation and approval workflows
- **Documentation**: [Quotation README](./quotation/README.md)
- **API Spec**: [Quotation OpenAPI](./quotation/openapi.yaml)

### Order
- **Path**: `sales/order/`
- **Description**: Sales order management service processing orders and fulfillment
- **Documentation**: [Order README](./order/README.md)
- **API Spec**: [Order OpenAPI](./order/openapi.yaml)

### Subscription
- **Path**: `sales/subscription/`
- **Description**: Subscription management service handling recurring billing and renewals
- **Documentation**: [Subscription README](./subscription/README.md)
- **API Spec**: [Subscription OpenAPI](./subscription/openapi.yaml)

### Loyalty
- **Path**: `sales/loyalty/`
- **Description**: Customer loyalty program service managing points and rewards
- **Documentation**: [Loyalty README](./loyalty/README.md)
- **API Spec**: [Loyalty OpenAPI](./loyalty/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/sales` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The sales services services work together to provide complete functionality:

1. **Sales Process**:
   - `quotation/` creates quotes
   - `order/` converts quotes to orders
   - `subscription/` manages recurring sales
   - `loyalty/` tracks customer rewards

2. **Order Fulfillment**:
   - `order/` processes sales orders
   - Integrates with `inventory/` for stock management
   - Integrates with `accounting/` for invoicing
