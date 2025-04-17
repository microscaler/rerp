# AI Services

## Overview

AI capabilities and document processing.

## Services

### Core
- **Path**: `ai/core/`
- **Description**: Core AI service providing AI capabilities across modules
- **Documentation**: [Core README](./core/README.md)
- **API Spec**: [Core OpenAPI](./core/openapi.yaml)

### Document
- **Path**: `ai/document/`
- **Description**: Document AI service using machine learning for document extraction
- **Documentation**: [Document README](./document/README.md)
- **API Spec**: [Document OpenAPI](./document/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/ai` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The ai services services work together to provide complete functionality:

*Integration patterns specific to ai services will be documented as services are implemented.*
