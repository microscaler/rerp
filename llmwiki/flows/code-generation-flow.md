# Code Generation Flow

> How `openapi.yaml` → `brrtrouter-gen` → generated Rust crate works in RERP.

**Status:** partially-verified

## Overview

```
openapi/{suite}/{name}/openapi.yaml
    → brrtrouter-gen
    → microservices/{suite}/{name}/gen/ (generated Rust crate)
    → microservices/{suite}/{name}/impl/ (business logic, authored by developers)
```

## Two-Crate Model

Each service has exactly two crates:
- **Generated crate** (`gen/`): Auto-generated from OpenAPI. Contains types, routes, handler traits, serializers. **Never edit directly.**
- **Implementation crate** (`impl/`): Business logic. Contains the handler implementations, DB operations, server setup. This is where developers write code.

## Generation Command

```bash
brrtrouter-gen --spec openapi/{suite}/{name}/openapi.yaml --output microservices/{suite}/{name}/gen
```

## Key Conventions

- The generated crate defines the service's API surface (paths, methods, request/response types).
- The impl crate implements the `Service` trait or handler functions defined in the gen crate.
- Changes to the API surface must go in `openapi.yaml` first, then regenerate.
- Changes to business logic go directly in `impl/src/`.

## Suite BFF Generation

Each suite has one BFF that aggregates its services:
```bash
bff-generator generate-spec --config openapi/{suite}/bff-suite-config.yaml --output openapi/{suite}/openapi_bff.yaml
```

The BFF config (`bff-suite-config.yaml`) declares `bff_service_name` and lists all microservices in the suite.
