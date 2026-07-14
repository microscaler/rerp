# Sesame RLS Contract v1

`install.sql` is the application-vendored Sesame/Lifeguard RLS protocol. Its
canonical source is `seasame-idam/sql/rls/v1/install.sql`; RERP keeps a copy so
a self-hosted database can be bootstrapped without a runtime dependency on the
Sesame repository.

Lifeguard calls `public.rls_set_session` after `BEGIN`. The function validates
the complete identity shape and writes transaction-local PostgreSQL settings.
RERP policies compare every tenant-owned row with
`public.sesame_current_tenant_id()`.

The install intentionally revokes function execution from `PUBLIC`.
Deployment must grant the runtime role only the functions it needs. JWT
validation remains in BRRTRouter/Sesame; no token or caller-supplied tenant
value is accepted by this SQL contract.
