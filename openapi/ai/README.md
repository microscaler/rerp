# AI Services

## Overview

AI capabilities for intelligent processing across RERP modules.

## Services

### Core
- **Path**: `ai/core/`
- **Description**: Core AI service providing AI capabilities across modules
- **Documentation**: [Core README](./core/README.md)
- **API Spec**: [Core OpenAPI](./core/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/ai` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The ai services services work together to provide complete functionality:

*Integration patterns specific to ai services will be documented as services are implemented.*

---

> **Note:** Document processing (OCR, extraction, classification) has been moved to the [Documents suite](../documents/). The AI suite provides the ML inference and intelligence layer that other suites consume.
