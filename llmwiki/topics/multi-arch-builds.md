# Multi-Arch Builds

> Docker multi-architecture build process for RERP microservices.

**Status:** partially-verified

## Overview

RERP uses multi-arch container builds for deployment across different platforms.

## Docker Structure

- `docker/base/` — Base image (Rust toolchain, dependencies)
- `docker/build/` — Build stage (cargo build)
- `docker/microservices/` — Per-service Dockerfiles
- `docker/website/` — Frontend website build

## Helm

- `helm/rerp-microservice/` — Helm chart template for deploying microservices
- Supports configurable service parameters

## K8s

- `k8s/data/` — Data service configs
- `k8s/microservices/` — Microservice K8s manifests

## Code Anchors
- Dockerfiles: `docker/*/`
- Helm charts: `helm/rerp-microservice/`
- K8s manifests: `k8s/`
- Design doc: `docs/CONTAINER_RELEASE_DESIGN_PROPOSAL.md`
- Multi-arch doc: `docs/MULTIARCH_BUILD.md`
