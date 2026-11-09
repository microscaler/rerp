# CRM-PLT-01: CRM platform resources on GCP (gcp-infrastructure)

**Repository:** gcp-infrastructure (Crossplane), not RERP · **Build step:** 0 · **Owner:** supervisor (not the local model) · **Points:** 8

## Goal

The resources of `openapi/crm/docs/13-implementation-plan.md` §2.1 exist in pp through Crossplane: the Cloud SQL
instance and databases, one Postgres user per service, Pub/Sub topics `crm.<service>.v1` with subscriptions and
dead-letter topics per `openapi/crm/docs/10-events.md`, service accounts with Workload Identity bindings, Secret Manager
entries, the bucket, and the tracking host.

## Why not the local model

It changes the GCP organisation through the gcp-infrastructure Crossplane code, which needs review by someone who can
see the platform state. The local model may draft the manifests in a branch for review.
