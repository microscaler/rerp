#!/usr/bin/env python3
"""The CRM data model: validate it, render the ERD document, and check LifeModels against it.

    python3 openapi/crm/tools/data_model.py               # validate openapi/crm/docs/data-model/*.yaml
    python3 openapi/crm/tools/data_model.py --docs        # also rewrite openapi/crm/docs/14-data-model.md
    python3 openapi/crm/tools/data_model.py --models      # also compare microservices/crm/<svc>/impl/src/models/*.rs

The YAML files are the design (one per service). LifeModels in each service's impl crate implement them, the migrator
generates the SQL from the LifeModels, and `--models` reports every table or column that differs, so an implementation
can be reviewed mechanically.

Column syntax: "<type> [null] [pk] [unique] [default <sql>] [fk <table>(<col>) [cascade|restrict|set null]]
[check(<v1>,<v2>,...)]". Types: uuid, text, varchar(n), char(n), integer, bigint, boolean, timestamptz, date,
numeric(p,s), double precision, jsonb.
"""
import argparse
import re
import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
REPO = ROOT.parents[1]
DM = ROOT / 'docs' / 'data-model'
SERVICES = ['workspace', 'audience', 'messaging', 'automation', 'sales', 'acquisition', 'insights', 'intake']
TYPES = re.compile(r'^(uuid|text|varchar\(\d+\)|char\(\d+\)|integer|bigint|boolean|timestamptz|date|numeric\(\d+,\d+\)|'
                   r'double precision|jsonb)$')
RUST = {'uuid': 'uuid::Uuid', 'text': 'String', 'varchar': 'String', 'char': 'String', 'integer': 'i32', 'bigint': 'i64',
        'boolean': 'bool', 'timestamptz': 'chrono::DateTime<chrono::Utc>', 'date': 'chrono::NaiveDate',
        'numeric': 'rust_decimal::Decimal', 'double precision': 'f64', 'jsonb': 'serde_json::Value'}
RESERVED = {'type', 'match', 'ref', 'move', 'self', 'loop', 'fn', 'use', 'mod', 'impl', 'trait', 'struct', 'enum',
            'where', 'as', 'in', 'for', 'if', 'else', 'return', 'static', 'const', 'crate', 'super', 'async', 'await',
            'dyn', 'try', 'gen', 'from', 'to', 'values', 'group', 'user', 'order', 'limit', 'end', 'default', 'check',
            'table', 'select', 'primary', 'references', 'offset', 'window', 'case', 'when', 'current', 'by', 'trigger',
            'text', 'all', 'and', 'or', 'not', 'null', 'only', 'on', 'column', 'constraint', 'unique'}


def parse_col(spec):
    """Split a column spec into its parts."""
    out = {'null': False, 'pk': False, 'unique': False, 'default': None, 'fk': None, 'on_delete': None, 'check': None}
    m = re.match(r'^(double precision|numeric\(\d+,\d+\)|\w+(?:\(\d+\))?)\s*(.*)$', spec.strip())
    out['type'], rest = m.group(1), m.group(2)
    c = re.search(r'check\(([^)]*)\)', rest)
    if c:
        out['check'] = [x.strip() for x in c.group(1).split(',')]
        rest = rest[:c.start()] + rest[c.end():]
    f = re.search(r'fk (\w+)\((\w+)\)(?: (cascade|restrict|set null))?', rest)
    if f:
        out['fk'], out['on_delete'] = f'{f.group(1)}({f.group(2)})', f.group(3) or 'restrict'
        rest = rest[:f.start()] + rest[f.end():]
    d = re.search(r"default (\S+(?:\(\))?)", rest)
    if d:
        out['default'] = d.group(1)
        rest = rest[:d.start()] + rest[d.end():]
    words = rest.split()
    out['null'] = 'null' in words
    out['pk'] = 'pk' in words
    out['unique'] = 'unique' in words
    left = [w for w in words if w not in ('null', 'pk', 'unique')]
    if left:
        out['unknown'] = left
    return out


def rust_type(col):
    base = re.sub(r'\(.*', '', col['type'])
    t = RUST[base if base in RUST else col['type']]
    return f'Option<{t}>' if col['null'] else t


def load():
    models = {}
    for svc in SERVICES:
        p = DM / f'{svc}.yaml'
        models[svc] = yaml.safe_load(p.read_text())
    return models


def validate(models):
    errs = []
    for svc, m in models.items():
        names = set(m['tables'])
        for t, d in m['tables'].items():
            cols = d['columns']
            if not d.get('description'):
                errs.append(f'{svc}.{t}: no description')
            pks = [k for k, v in cols.items() if parse_col(v)['pk']]
            if not pks:
                errs.append(f'{svc}.{t}: no primary key')
            if d.get('workspace_scoped', True) and t != 'workspaces' and 'workspace_id' not in cols:
                errs.append(f'{svc}.{t}: no workspace_id (mark workspace_scoped: false if intended)')
            for k, v in cols.items():
                c = parse_col(v)
                if not TYPES.match(c['type']):
                    errs.append(f'{svc}.{t}.{k}: type {c["type"]!r} is not one Lifeguard maps')
                if c.get('unknown'):
                    errs.append(f'{svc}.{t}.{k}: unknown words {c["unknown"]}')
                if k in RESERVED:
                    errs.append(f'{svc}.{t}.{k}: reserved word in Rust or SQL')
                if c['fk'] and c['fk'].split('(')[0] not in names:
                    errs.append(f'{svc}.{t}.{k}: foreign key to {c["fk"]} outside this service (only ids cross services)')
    return errs


def mermaid(svc, m):
    lines = ['erDiagram']
    for t, d in m['tables'].items():
        for k, v in d['columns'].items():
            c = parse_col(v)
            if c['fk']:
                tgt = c['fk'].split('(')[0]
                card = '|o--o{' if c['null'] else '||--o{'
                lines.append(f'    {tgt.upper()} {card} {t.upper()} : "{k}"')
    for t, d in m['tables'].items():
        lines.append(f'    {t.upper()} {{')
        for k, v in d['columns'].items():
            c = parse_col(v)
            ty = re.sub(r'[(),]', '_', c['type']).replace(' ', '_').strip('_')
            key = ' PK' if c['pk'] else (' FK' if c['fk'] else '')
            lines.append(f'        {ty} {k}{key}')
        lines.append('    }')
    return '\n'.join(lines)


def docs(models):
    out = ['# 14 — CRM data model', '',
           '> Generated by `openapi/crm/tools/data_model.py --docs` from `docs/data-model/<service>.yaml`; edit those, not',
           '> this. **Status:** Design for implementation, 2026-11-09. **Derived from:** the service contracts (every',
           '> entity schema), [08 §6](08-service-decomposition.md), [ADR 004](../../../docs/adrs/004-crm-capture-intake-and-workspaces.md).',
           '', '## 1. Rules', '',
           '| Rule | Detail |', '|---|---|',
           '| Stores | Database `crm`: one schema per service (audience, messaging, automation, sales, acquisition, workspace) plus `audience`-style infrastructure tables in each. Database `crm_insights`: schema `insights`. Database `crm_intake`: schema `intake`. Schema `audit` in `crm` holds the audit log. |',
           '| Ownership | One LifeModel per table, in the owning service\'s `impl/src/models/` with `#[schema_name = "<service>"]`. No foreign key crosses a schema: other services\' ids are plain uuid columns. |',
           '| Workspaces | Every table has `workspace_id uuid not null`, and row-level security: `USING (workspace_id = current_setting(\'crm.workspace_id\')::uuid)`, forced, for the service role. Exceptions are marked (`workspaces`, `memberships`: membership-based; `intake.rate_limits`: none). |',
           '| Keys | `id uuid` generated by the service (UUID v7 for time order). Composite keys where a row is a pair. |',
           '| Time | `timestamptz` everywhere (Rust `chrono::DateTime<chrono::Utc>`); `*_at` is a timestamp, `*_date` a date. |',
           '| Money | `numeric(19,4)` (Rust `rust_decimal::Decimal`); the API sends it as a decimal string. |',
           '| Enums | `text` with a CHECK of the allowed values (Lifeguard `#[check = "..."]`), matching the contract\'s enum. |',
           '| Lists | JSON arrays in `jsonb` (Lifeguard has no array mapping); a list someone queries by member is a join table instead (`contact_tags`, `deal_contacts`, `task_type_outcomes`). |',
           '| Users | Sesame user ids are `varchar(64)`, never uuid. |',
           '| Personal data | Erasure blanks personal columns and sets `erased_at`; append-only tables are erased by contact id. |',
           '| Names | No column is a Rust keyword or SQL reserved word (`from_address`, `field_values`, `event_type`, ...). The API field name may differ; the handler maps it. |',
           '| API-only fields | Listed per table: computed by the handler (counts, names of related records, rates), never stored. |',
           '', '## 2. From model to migration', '',
           '1. Write the LifeModels for a service from its YAML (`impl/src/models/<table>.rs`, one file per table; types per',
           '   the table below).',
           '2. `python3 openapi/crm/tools/data_model.py --models` must report nothing for that service.',
           '3. `cargo run -p rerp_migrator --features crm -- generate --suite crm` writes',
           '   `microservices/crm/migrations/<service>/0001_generated_entities.sql` and `apply_order.txt`.',
           '4. The authored `microservices/crm/migrations/<service>/0002_rls_and_infrastructure.sql` (from',
           '   `microservices/crm/migrations/templates/rls_and_infrastructure.sql`) enables RLS on every table and creates',
           '   `outbox`, `processed_events` and `idempotency_keys` in the schema.',
           '5. Apply to a local Postgres and run the service\'s isolation test.', '',
           '| Model type | Rust type | Lifeguard attribute |', '|---|---|---|',
           '| uuid | `uuid::Uuid` | |', '| text | `String` | |', '| varchar(n) / char(n) | `String` | `#[column_type = "VARCHAR(n)"]` |',
           '| integer / bigint | `i32` / `i64` | |', '| boolean | `bool` | |',
           '| timestamptz | `chrono::DateTime<chrono::Utc>` | `#[column_type = "TIMESTAMPTZ"]` |',
           '| date | `chrono::NaiveDate` | |', '| numeric(19,4) | `rust_decimal::Decimal` | `#[column_type = "NUMERIC(19,4)"]` |',
           '| double precision | `f64` | |', '| jsonb | `serde_json::Value` | `#[column_type = "JSONB"]` |',
           '| null | `Option<T>` | |', '| default now() | | `#[default_expr = "now()"]` |',
           '| fk t(c) on delete x | | `#[foreign_key = "t(c) ON DELETE X"]`, `#[indexed]` |',
           '| check(a,b) | | `#[check = "col IN (\'a\',\'b\')"]` |', '']
    n_tables = sum(len(m['tables']) for m in models.values())
    out += ['## 3. Services', '', f'{n_tables} tables. Infrastructure tables (per schema, authored SQL):', '',
            models['audience'].get('infrastructure', ''), '']
    for i, svc in enumerate(SERVICES, 1):
        m = models[svc]
        out += [f'### 3.{i} {svc} — database `{m["database"]}`, schema `{m["schema"]}`', '', '```mermaid', mermaid(svc, m), '```', '']
        for t, d in m['tables'].items():
            flags = [x for x in ('append_only', 'singleton', 'read_model') if d.get(x)]
            out += [f'#### `{d.get("schema", m["schema"])}.{t}`' + (f' ({", ".join(f.replace("_", "-") for f in flags)})' if flags else ''),
                    '', d['description'] + (f' API schema: `{d["api"]}`.' if d.get('api') else ''), '',
                    '| Column | Type | Null | Default | References / check |', '|---|---|---|---|---|']
            for k, v in d['columns'].items():
                c = parse_col(v)
                ref = (f'→ {c["fk"]} on delete {c["on_delete"]}' if c['fk'] else '') + \
                      (('; ' if c['fk'] else '') + 'in ' + ', '.join(c['check']) if c['check'] else '')
                key = ' (PK)' if c['pk'] else (' (unique)' if c['unique'] else '')
                out.append(f'| `{k}`{key} | {c["type"]} | {"yes" if c["null"] else ""} | {c["default"] or ""} | {ref} |')
            extras = []
            if d.get('unique'):
                extras.append('Unique: ' + '; '.join(f'`({u})`' for u in d['unique']))
            if d.get('indexes'):
                extras.append('Indexes: ' + '; '.join(f'`({x})`' for x in d['indexes']))
            if d.get('foreign_keys'):
                extras.append('Foreign keys: ' + '; '.join(f'`{x}`' for x in d['foreign_keys']))
            if d.get('api_only'):
                extras.append('API-only (computed): ' + ', '.join(f'`{x}`' for x in d['api_only']))
            if d.get('notes'):
                extras.append(d['notes'])
            if d.get('rls'):
                extras.append(f'Row-level security: {d["rls"]}.')
            out += [''] + [f'- {x}' for x in extras] + ['']
    (ROOT / 'docs' / '14-data-model.md').write_text('\n'.join(out))


FIELD = re.compile(r'^\s*pub\s+(r#)?(\w+)\s*:\s*([^,]+),', re.M)


def check_models(models):
    """Compare LifeModel structs with the design. Missing model files are reported once per service."""
    errs = []
    for svc, m in models.items():
        mdir = REPO / 'microservices' / 'crm' / svc / 'impl' / 'src' / 'models'
        found = {}
        for f in sorted(mdir.glob('*.rs')) if mdir.exists() else []:
            src = f.read_text()
            for tm in re.finditer(r'#\[table_name\s*=\s*"(\w+)"\]', src):
                body = src[tm.end():]
                end = body.find('\n}')
                found[tm.group(1)] = (f.name, dict((x.group(2), x.group(3).strip()) for x in FIELD.finditer(body[:end])),
                                      src)
        if not found:
            errs.append(f'{svc}: no LifeModels yet ({len(m["tables"])} tables to write)')
            continue
        for t, d in m['tables'].items():
            if t not in found:
                errs.append(f'{svc}.{t}: no LifeModel')
                continue
            fname, fields, src = found[t]
            schema = d.get('schema', m['schema'])
            if f'#[schema_name = "{schema}"]' not in src:
                errs.append(f'{svc}.{t}: {fname} lacks #[schema_name = "{schema}"]')
            for k, v in d['columns'].items():
                if k not in fields:
                    errs.append(f'{svc}.{t}.{k}: missing in {fname}')
                    continue
                want = rust_type(parse_col(v))
                got = re.sub(r'\s+', '', fields[k])
                if re.sub(r'\s+', '', want) != got and re.sub(r'\s+', '', want.replace('chrono::', '').replace('uuid::', '')
                                                             .replace('rust_decimal::', '').replace('serde_json::', '')) != got:
                    errs.append(f'{svc}.{t}.{k}: {fname} has {got}, design says {want}')
            for k in fields:
                if k not in d['columns']:
                    errs.append(f'{svc}.{t}.{k}: in {fname} but not in the design')
        for t in found:
            if t not in m['tables']:
                errs.append(f'{svc}: LifeModel table {t} is not in the design')
    return errs


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--docs', action='store_true')
    ap.add_argument('--models', action='store_true')
    a = ap.parse_args()
    models = load()
    errs = validate(models)
    if a.docs and not errs:
        docs(models)
    if a.models:
        errs += check_models(models)
    if errs:
        print('\n'.join(errs))
        return 1
    print(f'CRM data model OK: {sum(len(m["tables"]) for m in models.values())} tables in {len(models)} services')
    return 0


if __name__ == '__main__':
    sys.exit(main())
