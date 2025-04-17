# Workflow Automation

## Overview

Rule-based workflow automation across all modules.

## Services

### Core
- **Path**: `automation/core/`
- **Description**: Workflow automation service providing rule-based automation
- **Documentation**: [Core README](./core/README.md)
- **API Spec**: [Core OpenAPI](./core/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/automation` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The workflow automation services work together to provide complete functionality:

*Integration patterns specific to workflow automation will be documented as services are implemented.*
