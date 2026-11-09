#!/usr/bin/env python3
"""Checks the CRM service contracts against the conventions in openapi/crm/docs/09-api-conventions.md.

    python3 openapi/crm/tools/check_specs.py            # check; exit 1 on any finding
    python3 openapi/crm/tools/check_specs.py --docs     # also rewrite the generated docs (10-events.md, 11-permissions.md)

What it checks, per service (audience, messaging, automation, sales, acquisition, insights, workspace, edge, intake):
- every operation: unique snake_case operationId, summary, description, x-required-role, x-brrtrouter-impl;
- workspace scoping: X-Workspace-Id on every operation except the edge's and those marked x-no-workspace;
- the edge is public (security: []) and nothing else is;
- errors use the shared ErrorResponse responses; side-effecting verbs take Idempotency-Key;
- shapes brrtrouter-gen turns into compiling Rust: no allOf, no self-referencing schema, no body field that repeats a
  parameter; list responses are <Item>Page;
- money is a decimal string; user references are Sesame user ids (strings, no uuid format);
- every JSON request and 2xx response has an example that validates against its schema (JSON Schema 2020-12, formats);
- events: each x-events entry is a message the service's asyncapi.yaml publishes, and every published message names an
  operation or is published by a worker; AsyncAPI data refs resolve.
"""
import argparse
import json
import re
import sys
from pathlib import Path

import yaml

try:
    import jsonschema
    from referencing import Registry, Resource
    from referencing.jsonschema import DRAFT202012
except ImportError:  # pragma: no cover
    sys.exit('check_specs needs jsonschema>=4.18 (pip install jsonschema)')

ROOT = Path(__file__).resolve().parents[1]
SERVICES = ['audience', 'messaging', 'automation', 'sales', 'acquisition', 'insights', 'workspace', 'edge', 'intake']
ROLES = {'owner', 'admin', 'marketer', 'sales', 'read-only'}
IDEM_VERBS = {'schedule', 'send', 'test-send', 'sync', 'import', 'export', 'convert', 'approve', 'apply-recommendation',
              'publish', 'undo', 'recalculate', 'import-spend', 'run', 'fire', 'instantiate', 'merge', 'bulk', 'duplicate',
              'resend', 'regenerate', 'activate', 'retry', 'reconnect', 'submit', 'cancel'}
MONEY = re.compile(r'^(amount|.*revenue.*|.*_value|value|spend|daily_budget|cac|cpc|cpl|ltv_90d|quota.*|avg_deal_size|'
                   r'average_deal_size|weighted_pipeline)$')
MONEY_NOT = re.compile(r'^quota_attainment$')
NOT_MONEY = {('LeaderboardEntry', 'value'), ('PeriodCount', 'value'), ('Point', 'value')}
USER_REF = re.compile(r'^(owner_id|user_id|assignee_id|created_by|updated_by|approved_by|requested_by|sender_user_id|'
                      r'from_user_id|decided_by)$')
# Deliberate exceptions, each with its reason
ALLOW = {
    ('nonstd_error', 'messaging', 'POST /campaigns/{id}/schedule'): 'returns the readiness report (CampaignValidation) on 400',
    ('nonstd_error', 'automation', 'PUT /workflows/{id}/steps'): 'returns the graph report (StepGraphValidation) on 400',
}


class Checker:
    def __init__(self):
        self.findings = []

    def f(self, kind, svc, where, msg):
        if (kind, svc, where) in ALLOW:
            return
        self.findings.append(f'{svc}: {kind}: {where}: {msg}')

    # ------------------------------------------------------------------ openapi
    def openapi(self, svc, doc):
        comp = doc.get('components', {})
        schemas = comp.get('schemas', {})
        params = comp.get('parameters', {})
        reg = Registry().with_resource('urn:doc', Resource.from_contents(doc, default_specification=DRAFT202012))
        fmt = jsonschema.Draft202012Validator.FORMAT_CHECKER
        opids = set()

        def resolve(p):
            return params.get(p['$ref'].split('/')[-1], {}) if '$ref' in p else p

        def validate(schema, value):
            wrapped = json.loads(json.dumps(schema).replace('"#/components/', '"urn:doc#/components/'))
            v = jsonschema.Draft202012Validator(wrapped, registry=reg, format_checker=fmt)
            return [f'{"/".join(map(str, e.absolute_path)) or "(root)"}: {e.message[:140]}' for e in v.iter_errors(value)][:3]

        for path, item in doc.get('paths', {}).items():
            for m, op in item.items():
                if m == 'parameters':
                    continue
                w = f'{m.upper()} {path}'
                oid = op.get('operationId', '')
                if not re.fullmatch(r'[a-z][a-z0-9_]*', oid):
                    self.f('operation_id', svc, w, f'{oid!r} is not snake_case')
                if oid in opids:
                    self.f('operation_id', svc, w, f'{oid} repeated')
                opids.add(oid)
                for k in ('summary', 'description'):
                    if not op.get(k):
                        self.f('docs', svc, w, f'no {k}')
                if op.get('x-brrtrouter-impl') is not True:
                    self.f('impl_flag', svc, w, 'x-brrtrouter-impl: true missing')
                roles = op.get('x-required-role')
                if svc != 'edge' and (not roles or set(roles) - ROLES):
                    self.f('roles', svc, w, f'x-required-role {roles!r}')
                ps = [resolve(p) for p in item.get('parameters', []) + op.get('parameters', [])]
                names = {p.get('name') for p in ps}
                if svc == 'edge':
                    if op.get('security') != []:
                        self.f('security', svc, w, 'edge operations are public (security: [])')
                else:
                    if op.get('security') == []:
                        self.f('security', svc, w, 'only the edge is public')
                    if 'X-Workspace-Id' not in names and not op.get('x-no-workspace'):
                        self.f('workspace', svc, w, 'X-Workspace-Id missing')
                last = path.rstrip('/').split('/')[-1]
                if m == 'post' and (last in IDEM_VERBS or op.get('x-idempotent')) and 'Idempotency-Key' not in names:
                    self.f('idempotency', svc, w, 'Idempotency-Key missing')
                for code, r in op.get('responses', {}).items():
                    if code >= '400' and '$ref' not in r:
                        self.f('nonstd_error', svc, w, f'{code} does not use a shared error response')
                # body field repeating a parameter
                body = op.get('requestBody', {}).get('content', {}).get('application/json', {})
                bs = body.get('schema', {})
                tb = schemas.get(bs['$ref'].split('/')[-1], {}) if '$ref' in bs else bs
                clash = {n.lower().replace('-', '_') for n in names if n} & set(tb.get('properties', {}))
                if clash:
                    self.f('body_clash', svc, w, f'body repeats {sorted(clash)}')
                # examples
                targets = [('request', body)] if body.get('schema') else []
                for code, r in op.get('responses', {}).items():
                    c = r.get('content', {}).get('application/json') if isinstance(r, dict) else None
                    if code.startswith('2') and c and c.get('schema'):
                        targets.append((code, c))
                        if m == 'get' and code == '200':
                            ref = c['schema'].get('$ref', '')
                            props = schemas.get(ref.split('/')[-1], {}).get('properties', {}) if ref else c['schema'].get('properties', {})
                            if {'items', 'total', 'page', 'limit'} <= set(props) and not ref.endswith('Page'):
                                self.f('page', svc, w, 'a page response is named <Item>Page')
                for label, c in targets:
                    if 'example' not in c:
                        self.f('example', svc, w, f'{label}: no example')
                        continue
                    errs = validate(c['schema'], c['example'])
                    if errs:
                        self.f('example', svc, w, f'{label}: example invalid: {errs}')
        # schemas
        for n, s in schemas.items():
            txt = json.dumps(s)
            if '"allOf"' in txt:
                self.f('allof', svc, n, 'allOf (flatten it; brrtrouter-gen emits no type)')
            if f'"#/components/schemas/{n}"' in txt:
                self.f('recursive', svc, n, 'refers to itself (brrtrouter-gen recursion)')
            for p, v in s.get('properties', {}).items():
                if not isinstance(v, dict):
                    continue
                t = v.get('type')
                if (t == 'number' or (isinstance(t, list) and 'number' in t)) and MONEY.match(p) and not MONEY_NOT.match(p) and (n, p) not in NOT_MONEY:
                    self.f('money', svc, f'{n}.{p}', 'money is {type: string, format: decimal}')
                if USER_REF.match(p) and v.get('format') == 'uuid':
                    self.f('user_ref', svc, f'{n}.{p}', 'Sesame user ids are strings, not uuids')
                if p.endswith('_at') and v.get('format') == 'date':
                    self.f('naming', svc, f'{n}.{p}', '*_at is a date-time; a date is *_date')
        return opids

    # ------------------------------------------------------------------ asyncapi
    def asyncapi(self, svc, adoc, odoc):
        msgs = adoc.get('components', {}).get('messages', {})
        published = set()
        for ch in adoc.get('channels', {}).values():
            pass
        pub_ch = adoc.get('channels', {}).get('published', {})
        for k, ref in pub_ch.get('messages', {}).items():
            published.add(msgs[ref['$ref'].split('/')[-1]]['name'])
        used = set()
        for path, item in odoc.get('paths', {}).items():
            for m, op in item.items():
                for ev in op.get('x-events', []) if isinstance(op, dict) else []:
                    used.add(ev)
                    if ev not in published:
                        self.f('events', svc, f'{m.upper()} {path}', f'x-events {ev} is not published in asyncapi.yaml')
        # refs to openapi schemas resolve
        txt = json.dumps(adoc)
        for ref in set(re.findall(r'"\$ref": "([^"#]*)#/components/(schemas|messages)/([A-Za-z0-9_]+)"', txt)):
            fpath, kind, name = ref
            if not fpath:
                continue
            target = (ROOT / svc / fpath).resolve()
            if not target.exists():
                self.f('events', svc, fpath, 'referenced file missing')
                continue
            d = yaml.safe_load(target.read_text())
            if name not in d.get('components', {}).get(kind, {}):
                self.f('events', svc, f'{fpath}#{kind}/{name}', 'unresolved reference')


def gen_docs(specs, adocs):
    """docs/10-events.md (catalogue tables) and docs/11-permissions.md (role matrix), from the contracts."""
    rows, by_svc = [], {}
    for svc in SERVICES:
        a = adocs[svc]
        msgs = a.get('components', {}).get('messages', {})
        for k, ref in a.get('channels', {}).get('published', {}).get('messages', {}).items():
            msg = msgs[ref['$ref'].split('/')[-1]]
            data = msg['payload']['properties']['data'].get('$ref', '')
            data = data.split('/')[-1]
            desc = msg.get('description', '')
            ops = re.search(r'Published by: (.*?)(\n|$)', desc)
            cons = re.search(r'Consumed by: (.*?)\.?$', desc, re.S)
            rows.append((svc, msg['name'], data, ops.group(1) if ops else 'workers', cons.group(1).strip() if cons else '',
                         msg.get('summary', '')))
        consumes = []
        for ch, c in a.get('channels', {}).items():
            if ch.startswith('from_'):
                consumes += list(c.get('messages', {}))
        by_svc[svc] = consumes
    out = ['# 10 — CRM events', '',
           '> Generated by `openapi/crm/tools/check_specs.py --docs` from each service\'s `asyncapi.yaml`; edit those, not this.',
           '> Design: [08-service-decomposition.md §8](08-service-decomposition.md), [ADR 003](../../../docs/adrs/003-crm-funnel-conversion-boundary.md),',
           '> [ADR 004](../../../docs/adrs/004-crm-capture-intake-and-workspaces.md); conventions: [09-api-conventions.md](09-api-conventions.md).',
           '', '## Transport', '',
           '- One Pub/Sub topic per publishing service, `crm.<service>.v1`, created by gcp-infrastructure (Crossplane).',
           '- One subscription per consuming service and publisher, `<consumer>-<publisher>`, filtered on `attributes.event_type`,',
           '  with a dead-letter topic `<subscription>-dlq` after 10 attempts and exponential backoff (10 s to 10 min).',
           '- Every message is an envelope: `event_id` (consumers are idempotent on it), `event_type`, `workspace_id`,',
           '  `occurred_at`, `producer`, `actor`, `request_id`, `entity_id`, and `data`. Attributes repeat `event_type` and',
           '  `workspace_id` for filtering; the ordering key is `entity_id`.',
           '- Publishing uses an outbox: the event row is written in the same transaction as the change and relayed after commit,',
           '  so an event is never published for a change that rolled back, and never lost for one that committed.',
           '- Edge events (`*.received`) are untrusted input: the consumer re-validates shape, signature, token and workspace.',
           '- `crm-conversions` (signed conversions to RERP crm/accounts, ADR 003) is a separate topic with its own contract',
           '  (`microservices/crm/conversion`).', '',
           '## Catalogue', '', '| Event | Publisher | `data` | Published by | Consumed by | Meaning |', '|---|---|---|---|---|---|']
    for svc, name, data, ops, cons, meaning in rows:
        out.append(f'| `{name}` | {svc} | `{data}` | {ops} | {cons} | {meaning} |')
    out += ['', '## What each service consumes', '', '| Service | Consumes |', '|---|---|']
    camel = {re.sub(r'[._](\w)', lambda x: x.group(1).upper(), r[1]): r[1] for r in rows}
    for svc in SERVICES:
        names = sorted({camel.get(k, k) for k in by_svc[svc]})
        out.append(f'| {svc} | {", ".join(f"`{n}`" for n in names) or "nothing"} |')
    (ROOT / 'docs' / '10-events.md').write_text('\n'.join(out) + '\n')

    p = ['# 11 — CRM permission matrix', '',
         '> Generated by `openapi/crm/tools/check_specs.py --docs` from `x-required-role` on every operation; edit the specs,',
         '> not this. Roles are per workspace (ADR 004): owner, admin, marketer, sales, read-only. The gateway checks',
         '> membership of the workspace in `X-Workspace-Id`; each service checks the role. API keys act with their scopes.',
         '> Sales users may change only deals and tasks they own unless they are admin or owner (US_55_39 AC-3).', '']
    order = ['owner', 'admin', 'marketer', 'sales', 'read-only']
    for svc in SERVICES:
        if svc == 'edge':
            continue
        p += [f'## {svc}', '', '| Operation | ' + ' | '.join(order) + ' |', '|---|' + '---|' * len(order)]
        for path, item in specs[svc]['paths'].items():
            for m, op in item.items():
                if m == 'parameters':
                    continue
                r = set(op.get('x-required-role', []))
                p.append(f'| `{m.upper()} {path}` | ' + ' | '.join('✓' if x in r else '' for x in order) + ' |')
        p.append('')
    p += ['## edge', '', 'Public: no roles. Signed tokens, origin checks and rate limits instead (ADR 004).', '']
    (ROOT / 'docs' / '11-permissions.md').write_text('\n'.join(p))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--docs', action='store_true', help='rewrite docs/10-events.md and docs/11-permissions.md')
    a = ap.parse_args()
    c = Checker()
    specs, adocs = {}, {}
    for svc in SERVICES:
        specs[svc] = yaml.safe_load((ROOT / svc / 'openapi.yaml').read_text())
        c.openapi(svc, specs[svc])
        ap_path = ROOT / svc / 'asyncapi.yaml'
        if not ap_path.exists():
            c.f('events', svc, 'asyncapi.yaml', 'missing')
            continue
        adocs[svc] = yaml.safe_load(ap_path.read_text())
        c.asyncapi(svc, adocs[svc], specs[svc])
    if a.docs and len(adocs) == len(SERVICES):
        gen_docs(specs, adocs)
    n_ops = sum(1 for s in specs.values() for it in s['paths'].values() for m in it if m != 'parameters')
    if c.findings:
        print('\n'.join(c.findings))
        print(f'\n{len(c.findings)} finding(s) across {n_ops} operations')
        return 1
    print(f'CRM contracts OK: {len(SERVICES)} services, {n_ops} operations, every example valid')
    return 0


if __name__ == '__main__':
    sys.exit(main())
