-- Vendored Sesame RLS contract v1.
-- Canonical source: seasame-idam/sql/rls/v1/install.sql
--
-- Applications validate the JWT before calling this function. No token, key,
-- or unvalidated request value is accepted by the database. Lifeguard calls
-- public.rls_set_session once inside the transaction that owns the work.

CREATE OR REPLACE FUNCTION public.sesame_rls_contract_version()
RETURNS integer
LANGUAGE sql
IMMUTABLE
PARALLEL SAFE
SET search_path = pg_catalog
AS 'SELECT 1';

CREATE OR REPLACE FUNCTION public.rls_set_session(
    p_tenant_id text,
    p_subject_id uuid,
    p_organization_id uuid,
    p_session_id text,
    p_roles jsonb,
    p_permissions jsonb,
    p_user_type text,
    p_org_type text
)
RETURNS void
LANGUAGE plpgsql
SECURITY INVOKER
SET search_path = pg_catalog
AS $$
BEGIN
    IF p_tenant_id IS NULL
       OR p_subject_id IS NULL
       OR p_organization_id IS NULL
       OR NULLIF(pg_catalog.btrim(p_session_id), '') IS NULL THEN
        RAISE EXCEPTION 'incomplete Sesame RLS identity context'
            USING ERRCODE = '22023';
    END IF;

    IF pg_catalog.jsonb_typeof(p_roles) IS DISTINCT FROM 'array'
       OR pg_catalog.jsonb_typeof(p_permissions) IS DISTINCT FROM 'array' THEN
        RAISE EXCEPTION 'Sesame RLS roles and permissions must be JSON arrays'
            USING ERRCODE = '22023';
    END IF;

    PERFORM pg_catalog.set_config('sesame.tenant_id', p_tenant_id, true);
    PERFORM pg_catalog.set_config('sesame.subject_id', p_subject_id::text, true);
    PERFORM pg_catalog.set_config('sesame.organization_id', p_organization_id::text, true);
    PERFORM pg_catalog.set_config('sesame.session_id', p_session_id, true);
    PERFORM pg_catalog.set_config('sesame.roles', p_roles::text, true);
    PERFORM pg_catalog.set_config('sesame.permissions', p_permissions::text, true);
    PERFORM pg_catalog.set_config('sesame.user_type', COALESCE(p_user_type, ''), true);
    PERFORM pg_catalog.set_config('sesame.org_type', COALESCE(p_org_type, ''), true);
END;
$$;

CREATE OR REPLACE FUNCTION public.sesame_current_tenant_id()
RETURNS text
LANGUAGE sql
STABLE
PARALLEL SAFE
SET search_path = pg_catalog
AS $$
    SELECT NULLIF(pg_catalog.current_setting('sesame.tenant_id', true), '')
$$;

CREATE OR REPLACE FUNCTION public.sesame_current_subject_id()
RETURNS uuid
LANGUAGE sql
STABLE
PARALLEL SAFE
SET search_path = pg_catalog
AS $$
    SELECT NULLIF(pg_catalog.current_setting('sesame.subject_id', true), '')::uuid
$$;

CREATE OR REPLACE FUNCTION public.sesame_current_organization_id()
RETURNS uuid
LANGUAGE sql
STABLE
PARALLEL SAFE
SET search_path = pg_catalog
AS $$
    SELECT NULLIF(pg_catalog.current_setting('sesame.organization_id', true), '')::uuid
$$;

CREATE OR REPLACE FUNCTION public.sesame_current_session_id()
RETURNS text
LANGUAGE sql
STABLE
PARALLEL SAFE
SET search_path = pg_catalog
AS $$
    SELECT NULLIF(pg_catalog.current_setting('sesame.session_id', true), '')
$$;

CREATE OR REPLACE FUNCTION public.sesame_current_roles()
RETURNS jsonb
LANGUAGE sql
STABLE
PARALLEL SAFE
SET search_path = pg_catalog
AS $$
    SELECT NULLIF(pg_catalog.current_setting('sesame.roles', true), '')::jsonb
$$;

CREATE OR REPLACE FUNCTION public.sesame_current_permissions()
RETURNS jsonb
LANGUAGE sql
STABLE
PARALLEL SAFE
SET search_path = pg_catalog
AS $$
    SELECT NULLIF(pg_catalog.current_setting('sesame.permissions', true), '')::jsonb
$$;

CREATE OR REPLACE FUNCTION public.sesame_current_user_type()
RETURNS text
LANGUAGE sql
STABLE
PARALLEL SAFE
SET search_path = pg_catalog
AS $$
    SELECT NULLIF(pg_catalog.current_setting('sesame.user_type', true), '')
$$;

CREATE OR REPLACE FUNCTION public.sesame_current_org_type()
RETURNS text
LANGUAGE sql
STABLE
PARALLEL SAFE
SET search_path = pg_catalog
AS $$
    SELECT NULLIF(pg_catalog.current_setting('sesame.org_type', true), '')
$$;

CREATE OR REPLACE FUNCTION public.sesame_has_role(p_role text)
RETURNS boolean
LANGUAGE sql
STABLE
PARALLEL SAFE
SET search_path = pg_catalog
AS $$
    SELECT COALESCE(public.sesame_current_roles() ? p_role, false)
$$;

CREATE OR REPLACE FUNCTION public.sesame_has_permission(p_permission text)
RETURNS boolean
LANGUAGE sql
STABLE
PARALLEL SAFE
SET search_path = pg_catalog
AS $$
    SELECT COALESCE(public.sesame_current_permissions() ? p_permission, false)
$$;

COMMENT ON FUNCTION public.rls_set_session(text, uuid, uuid, text, jsonb, jsonb, text, text)
IS 'Sesame RLS v1: inject validated identity context using transaction-local GUCs';

REVOKE ALL ON FUNCTION public.sesame_rls_contract_version() FROM PUBLIC;
REVOKE ALL ON FUNCTION public.rls_set_session(text, uuid, uuid, text, jsonb, jsonb, text, text) FROM PUBLIC;
REVOKE ALL ON FUNCTION public.sesame_current_tenant_id() FROM PUBLIC;
REVOKE ALL ON FUNCTION public.sesame_current_subject_id() FROM PUBLIC;
REVOKE ALL ON FUNCTION public.sesame_current_organization_id() FROM PUBLIC;
REVOKE ALL ON FUNCTION public.sesame_current_session_id() FROM PUBLIC;
REVOKE ALL ON FUNCTION public.sesame_current_roles() FROM PUBLIC;
REVOKE ALL ON FUNCTION public.sesame_current_permissions() FROM PUBLIC;
REVOKE ALL ON FUNCTION public.sesame_current_user_type() FROM PUBLIC;
REVOKE ALL ON FUNCTION public.sesame_current_org_type() FROM PUBLIC;
REVOKE ALL ON FUNCTION public.sesame_has_role(text) FROM PUBLIC;
REVOKE ALL ON FUNCTION public.sesame_has_permission(text) FROM PUBLIC;
