# PriceWhisperer Microservice Helm Chart

A reusable Helm chart for deploying PriceWhisperer microservices (Marketing, IDAM, AMD, Billing, FTE, BFF).

## Usage

### Deploy a specific microservice

```bash
# Deploy Marketing service
helm install marketing ./helm/pricewhisperer-microservice -f ./helm/pricewhisperer-microservice/values/marketing.yaml

# Deploy IDAM service
helm install idam ./helm/pricewhisperer-microservice -f ./helm/pricewhisperer-microservice/values/idam.yaml

# Deploy AMD service
helm install amd ./helm/pricewhisperer-microservice -f ./helm/pricewhisperer-microservice/values/amd.yaml

# Deploy Billing service
helm install billing ./helm/pricewhisperer-microservice -f ./helm/pricewhisperer-microservice/values/billing.yaml

# Deploy FTE service
helm install fte ./helm/pricewhisperer-microservice -f ./helm/pricewhisperer-microservice/values/fte.yaml

# Deploy BFF service
helm install bff ./helm/pricewhisperer-microservice -f ./helm/pricewhisperer-microservice/values/bff.yaml
```

### Upgrade a service

```bash
helm upgrade marketing ./helm/pricewhisperer-microservice -f ./helm/pricewhisperer-microservice/values/marketing.yaml
```

### Uninstall a service

```bash
helm uninstall marketing
```

## Values Files

Each microservice has its own values file in `values/`:
- `marketing.yaml` - Marketing Service (port 8004)
- `idam.yaml` - Identity & Access Management Service (port 8002)
- `amd.yaml` - Account Management Domain Service (port 8003)
- `billing.yaml` - Billing Service (port 8005)
- `fte.yaml` - FTE Testing Service (port 8001)
- `bff.yaml` - Backend for Frontend Service (port 8000)

## Customization

Override any value from the base `values.yaml` by:
1. Editing the service-specific values file
2. Using `--set` flags: `helm install marketing ./helm/pricewhisperer-microservice -f ./helm/pricewhisperer-microservice/values/marketing.yaml --set deployment.replicas=3`

## Configuration

The chart creates:
- **ConfigMap**: Application configuration (security, database, redis, observability)
- **Deployment**: Service deployment with health checks and resource limits
- **Service**: Kubernetes service (NodePort by default for local dev)

## Ports

- Marketing: 8004 (NodePort: 30804)
- IDAM: 8002 (NodePort: 30802)
- AMD: 8003 (NodePort: 30803)
- Billing: 8005 (NodePort: 30805)
- FTE: 8001 (NodePort: 30801)
- BFF: 8000 (NodePort: 30800)

## Health Checks

All services expose `/health` endpoint for:
- Liveness probe (checks if service is alive)
- Readiness probe (checks if service is ready to accept traffic)

## Observability

All services are configured with:
- Prometheus metrics scraping (enabled by default)
- OpenTelemetry tracing (OTLP endpoint: `http://otel-collector:4317`)
- Structured JSON logging for Loki ingestion

