# Accounts Payable Build: Missing Fields Analysis

**Source of errors:** `tooling/.venv/bin/rerp build microservice accounts-payable` (equivalent to Tilt resource `build-accounts-payable`).

**Purpose:** Tabulate missing fields reported in build logs against the OpenAPI spec to determine whether the spec is non-compliant or BRRTRouter needs to be enriched.

---

## 0. Goal: fully generated impl controllers (like the gen path)

We want **impl-generated controllers to be fully generated**, in the same way the **gen** path produces complete handler/controller/types/registry code. The impl crate should give a developer a **complete starter**: every controller stub must return a **complete `Response { ... }`** literal (all fields present with placeholder or example values) so that:

1. The impl crate **compiles out of the box** after stub generation.
2. The developer can **start from a valid struct** and replace placeholders with real business logic, without fixing missing-field errors first.

So the impl stub generator in BRRTRouter must emit the same level of completeness as the gen path: **every response field** from the OpenAPI spec must appear in the generated `Response { ... }` initializer.

---

## 1. Build log errors (summary)

The build fails with **12 `E0063: missing fields ... in initializer of ... Response`** errors in the **impl** crate controllers. The **gen** crate defines full `Response` structs (from the OpenAPI spec); the **impl** stub returns a `Response { ... }` literal that is missing one or more fields.

---

## 2. Tabulation: missing fields vs OpenAPI spec

### 2.1 List endpoints (inline response schema: `items`, `total`, `page`, `limit`)

| Controller            | Missing fields (from build log) | OpenAPI location | In spec? |
|----------------------|---------------------------------|------------------|----------|
| `list_ap_agings`     | `limit`, `page`, `total`        | `GET /ap-agings` → 200 → `application/json` → schema `properties`: `items`, `total`, `page`, `limit` | Yes (lines 469–478) |
| `list_payments`      | `limit`, `page`, `total`        | `GET /payments` → 200 → schema `properties`: `items`, `total`, `page`, `limit` | Yes (lines 341–350) |
| `list_vendor_invoices` | `limit`, `page`, `total`     | `GET /vendor-invoices` → 200 → schema `properties`: `items`, `total`, `page`, `limit` | Yes (lines 26–36) |

**Conclusion:** All list response fields are defined in the OpenAPI spec. The gen `Response` types correctly have `items`, `limit`, `page`, `total`. The impl stub’s `Response { }` is missing these fields.

---

### 2.2 Single-entity responses: VendorInvoice (`#/components/schemas/VendorInvoice`)

| Controller               | Missing fields (from build log) | OpenAPI schema | In spec? |
|--------------------------|----------------------------------|----------------|----------|
| `create_vendor_invoice`  | `approval_status`, `approved_at`, `approved_by` and 12 other | `VendorInvoice` (lines 724–786) | Yes: id, invoice_id, vendor_id, outstanding_amount, days_until_due, aging_bucket, purchase_order_id, matching_status, approval_status, approved_at, approved_by, early_payment_discount_percent, early_payment_discount_date, metadata, created_at, updated_at |
| `get_vendor_invoice`     | same as above                    | Same           | Yes      |
| `update_vendor_invoice`  | same as above                    | Same           | Yes      |

**Conclusion:** Every VendorInvoice response field is in `components/schemas/VendorInvoice`. Gen matches the spec; impl stub is incomplete.

---

### 2.3 Single-entity responses: ApPayment (`#/components/schemas/ApPayment`)

| Controller      | Missing fields (from build log) | OpenAPI schema | In spec? |
|-----------------|----------------------------------|----------------|----------|
| `create_payment` | `bank_account_id`, `company_id`, `created_at` and 14 other | `ApPayment` (lines 787–864) | Yes: id, vendor_id, payment_number, payment_date, payment_amount, currency_code, payment_method, payment_reference, bank_account_id, status, applied_amount, company_id, notes, metadata, created_at, updated_at, created_by, updated_by |
| `get_payment`    | same as above                    | Same           | Yes      |
| `update_payment`| same as above                    | Same           | Yes      |

**Conclusion:** All ApPayment fields are in `components/schemas/ApPayment`. Gen matches; impl stub is incomplete.

---

### 2.4 Single-entity responses: ApAging (`#/components/schemas/ApAging`)

| Controller        | Missing fields (from build log) | OpenAPI schema | In spec? |
|-------------------|----------------------------------|----------------|----------|
| `create_ap_aging` | `company_id`, `created_at`, `currency_code` and 9 other | `ApAging` (lines 865–908) | Yes: id, vendor_id, aging_date, current, days_31_60, days_61_90, days_91_120, over_120, total_outstanding, currency_code, company_id, created_at, updated_at |
| `get_ap_aging`    | same as above                    | Same           | Yes      |
| `update_ap_aging`  | same as above                    | Same           | Yes      |

**Conclusion:** All ApAging fields are in `components/schemas/ApAging`. Gen matches; impl stub is incomplete.

---

## 3. Root cause

- **OpenAPI spec:** Defines all response shapes (inline for list endpoints, `#/components/schemas/` for entities). No missing or extra response fields; the spec is compliant for BRRTRouter’s expectations.
- **Gen crate:** Correctly derives `Request`/`Response` types and handler wiring from the spec. All fields from the spec are present in the gen `Response` structs.
- **Impl crate:** Controller stubs return a `Response { ... }` literal. That literal is either:
  - **Empty** (`Response { }`), or  
  - **Partial** (some fields omitted), or  
  - **Out of sync** (spec/gen was extended later, impl stubs were not regenerated with full literals).

So the failure is **not** due to the OpenAPI file being non-compliant. It is due to **impl stub generation**: stubs do not consistently emit a **complete** `Response` initializer for every field of the response schema.

---

## 4. Recommended fix approach (no code changes in this repo)

**Do not change:**  
- `openapi/accounting/accounts-payable/openapi.yaml`  
- BRRTRouter or accounts-payable code in this analysis.

**Do:**

1. **Enrich BRRTRouter’s impl stub generator** so that for every handler it generates a `Response { ... }` literal that includes **every** field of the response type:
   - For **list** responses: include `items`, `limit`, `page`, `total` (from the inline schema), with placeholder values (e.g. `None`, empty vec, 0) or values taken from the operation’s `examples` if present.
   - For **entity** responses (`$ref` to `#/components/schemas/...`): include every property of that schema, using:
     - The operation’s response `examples` when available (e.g. first example’s value per field), or  
     - Type-based defaults (e.g. `None` for Option, `Default::default()` for scalars, placeholder strings for uuid/date-time).

2. **Stub generator input:** The generator already has access to the same route/schema/example data used for gen (response schema and, where present, response examples). It should use that to build the full field list and default/example values for the impl `Response` literal.

3. **Regeneration:** After enriching the stub generator, regenerate impl stubs for accounts-payable (e.g. `brrtrouter-gen generate-stubs` or the rerp equivalent). The new stubs should compile without E0063.

4. **Optional:** Add a test or CI step that builds the impl crate after stub generation to catch any future mismatch between gen `Response` and impl `Response` literal.

---

## 5. Summary table (all 12 failing controllers)

| # | Impl controller           | Response type (gen)     | Missing (per build log) | In openapi.yaml? |
|---|---------------------------|-------------------------|--------------------------|------------------|
| 1 | list_ap_agings            | list response           | limit, page, total       | Yes (inline)     |
| 2 | list_payments             | list response           | limit, page, total       | Yes (inline)     |
| 3 | list_vendor_invoices      | list response           | limit, page, total       | Yes (inline)     |
| 4 | create_ap_aging            | ApAging                 | company_id, created_at, currency_code + 9 | Yes (ApAging)    |
| 5 | get_ap_aging               | ApAging                 | same                     | Yes              |
| 6 | update_ap_aging            | ApAging                 | same                     | Yes              |
| 7 | create_payment             | ApPayment               | bank_account_id, company_id, created_at + 14 | Yes (ApPayment) |
| 8 | get_payment                | ApPayment               | same                     | Yes              |
| 9 | update_payment             | ApPayment               | same                     | Yes              |
|10 | create_vendor_invoice      | VendorInvoice           | approval_status, approved_at, approved_by + 12 | Yes (VendorInvoice) |
|11 | get_vendor_invoice         | VendorInvoice           | same                     | Yes              |
|12 | update_vendor_invoice      | VendorInvoice           | same                     | Yes              |

**Verdict:** The OpenAPI spec is compliant and complete for these responses. The issue is that impl controllers are generated with incomplete `Response` literals. Fix by enriching BRRTRouter’s impl stub generator to emit a full `Response { ... }` for every response schema (using examples or type-based defaults), then regenerate stubs.

---

## 6. Call points: so BRRTRouter generates complete impl stubs

These are the entry points and code paths that must be used (and, where needed, enriched) so that BRRTRouter produces **fully generated** impl controller stubs—i.e. a complete `Response { ... }` for every handler.

### 6.1 RERP (invoker)

| Layer | Call point | Purpose |
|-------|------------|--------|
| **CLI** | `rerp gen stubs accounting [--service accounts-payable] [--force] [--sync]` | User-facing command to regenerate impl stubs for one or all accounting services. |
| **CLI handler** | `tooling/src/rerp_tooling/cli/gen.py` → `regenerate_impl_stubs(project_root, suite, service=..., force=..., sync=...)` | Parses `gen stubs` and delegates to bootstrap. |
| **Bootstrap** | `tooling/src/rerp_tooling/bootstrap/microservice.py` → `regenerate_impl_stubs()` → `generate_impl_stubs_with_brrtrouter()` | Resolves spec path, impl dir, component name; calls BRRTRouter. |
| **BRRTRouter invoker** | `brrtrouter_tooling.gen.brrtrouter.call_brrtrouter_generate_stubs(spec_path, impl_dir, component_name=..., force=..., sync=...)` | Runs `brrtrouter-gen generate-stubs` with the correct arguments (lives in brrtrouter_tooling; RERP uses it as a dependency). |

**Accounts-payable example:**  
`rerp gen stubs accounting --service accounts-payable --force`  
→ `regenerate_impl_stubs(project_root, "accounting", service="accounts-payable", force=True)`  
→ `generate_impl_stubs_with_brrtrouter(spec_path=openapi/accounting/accounts-payable/openapi.yaml, impl_dir=microservices/accounting/accounts-payable/impl, ..., component_name=rerp_accounting_accounts_payable_gen)`  
→ subprocess: `brrtrouter-gen generate-stubs --spec <spec_path> --output <impl_dir> --component-name rerp_accounting_accounts_payable_gen [--force] [--sync]`.

### 6.2 BRRTRouter (Rust) – stub generation

| Layer | File / symbol | Purpose |
|-------|----------------|--------|
| **CLI** | `src/bin/brrtrouter_gen.rs` | Binary entry; parses CLI and dispatches to `run_cli()`. |
| **CLI dispatch** | `src/cli/commands.rs` → `Commands::GenerateStubs { spec, output, component_name, path, force, sync }` | Parses `generate-stubs` and calls `generate_impl_stubs(...)`. |
| **Impl stub orchestration** | `src/generator/project/generate.rs` → **`generate_impl_stubs(spec_path, impl_output_dir, component_name, handler_name, force, sync)`** | Loads spec via `load_spec()`, gets routes; for each handler builds `request_fields` / **`response_fields`** and calls **`write_impl_controller_stub(ImplControllerStubParams { ..., res_fields: &response_fields, example: route.example.clone(), ... })`**. **Enrichment point:** ensure `response_fields` is the full list of response properties (see 6.3). |
| **Stub writer** | `src/generator/templates.rs` → **`write_impl_controller_stub(params: ImplControllerStubParams)`** | Builds **`enriched_fields`** from `params.res_fields` + `params.example` (using **`rust_literal_for_example(field, val)`** or **`field.value`**); passes them to the template as **`response_fields`**. **Enrichment point:** every field in `res_fields` must get a valid Rust literal so the template emits a complete struct. |
| **Template** | `templates/impl_controller_stub.rs.txt` | Renders **`Response { {% for field in response_fields %} {{ field.name }}: {% if field.optional %}None{% else %}{{ field.value }}{% endif %}, {% endfor %} }`**. Only emits fields that are in `response_fields`; each must have `name`, `optional`, and `value`. |
| **Response field source** | `src/generator/project/generate.rs`: `response_fields = route.response_schema.as_ref().map_or(vec![], extract_fields)` | **`response_fields`** comes from **`extract_fields(route.response_schema)`**. **Enrichment point:** `route.response_schema` must be the **resolved** response schema (object with `properties`), not a bare `$ref`, so `extract_fields` returns every property. |
| **Schema → fields** | `src/generator/schema.rs` → **`extract_fields(schema: &Value) -> Vec<FieldDef>`** | If `schema` has no `properties` (e.g. raw `{"$ref": "#/components/schemas/VendorInvoice"}`), returns **empty**. So impl path only gets full fields if **response_schema** is already resolved in `src/spec/build.rs` (e.g. `extract_response_schema_and_example` resolves refs). **Enrichment point:** if spec build does not pass a resolved schema for the chosen response (e.g. 201), either resolve before calling `extract_fields` or resolve inside a new helper used by the impl path. |
| **Example / default value** | `src/generator/schema.rs` → **`rust_literal_for_example(field, value)`**; `src/generator/templates.rs` uses **`field.value`** (from **`dummy_value::dummy_value(&ty)`** in `extract_fields`) | Ensures each field in the stub has a compilable Rust literal (from operation example or type-based dummy). **Enrichment point:** ensure every `FieldDef` used for response has a non-empty `value` (and optional handling) so the template never emits a missing field. |

### 6.3 Data flow (summary)

1. **Spec** → `load_spec` → **routes** (each with `response_schema`, `example`).
2. **Route** → **`response_schema`** (must be resolved object with `properties`) → **`extract_fields(response_schema)`** → **`response_fields: Vec<FieldDef>`** (each with `name`, `ty`, `optional`, **`value`**).
3. **`generate_impl_stubs`** → for each handler → **`write_impl_controller_stub(ImplControllerStubParams { res_fields: &response_fields, example, ... })`**.
4. **`write_impl_controller_stub`** → **enriched_fields** = `res_fields` with values from **example** (or keep **field.value**) → **ImplControllerStubTemplateData { response_fields: enriched_fields }** → **render** → write `impl/src/controllers/<handler>.rs`.
5. **Template** loops over **response_fields** and outputs **`field.name`: field.value** for every field.

To get **fully generated** impl controllers (complete `Response { ... }`), ensure:

- **Response schema** passed to `extract_fields` is the **resolved** schema (all properties), for every response (200, 201, etc.) used for stub generation.
- **Every** property of that schema becomes a **FieldDef** with a valid **value** (from `rust_literal_for_example` or `dummy_value`).
- The **template** receives that full list so it emits **every** field in the `Response { ... }` literal.

### 6.4 How to run (accounts-payable)

From the **RERP repo** (so impl stubs are generated and then built):

```bash
# Regenerate impl stubs for accounts-payable (overwrites existing stubs except user-owned)
tooling/.venv/bin/rerp gen stubs accounting --service accounts-payable --force

# Or regenerate for all accounting services
tooling/.venv/bin/rerp gen stubs accounting --force
```

Direct **BRRTRouter** invocation (for development/debugging of the generator):

```bash
cd /path/to/BRRTRouter
cargo run --bin brrtrouter-gen -- generate-stubs \
  --spec /path/to/rerp/openapi/accounting/accounts-payable/openapi.yaml \
  --output /path/to/rerp/microservices/accounting/accounts-payable/impl \
  --component-name rerp_accounting_accounts_payable_gen \
  --force
```

After BRRTRouter is enriched to emit full `Response { ... }` literals, re-running the same commands will produce impl controllers that compile without missing-field errors and give developers a complete starter for business logic.

### 6.5 Template fix: one field per line (E0063 when all fields on one line)

Even when the stub generator emits **all** response fields, the **template** must output each field on its **own line**. If every field is rendered on a single line, the first `// TODO: Set from your business logic` comment comments out the rest of the line in Rust, so the compiler only sees the first field and reports **E0063: missing fields**.

**Fix (BRRTRouter):** In **`templates/impl_controller_stub.rs.txt`**, the `Response { ... }` block must not strip newlines between fields. Using Tera’s `{%- endfor %}` strips the newline after each field, putting everything on one line. Use **`{% for field in response_fields %}`** and **`{% endfor %}`** (without the `-`) so each field is on its own line. After changing the template, **rebuild** `brrtrouter-gen` (`cargo build --bin brrtrouter-gen`) so the binary embeds the updated template, then regenerate impl stubs.

**Regeneration after template change:** If you use a pre-built `target/debug/brrtrouter-gen`, rebuild it first so the new template is used:  
`cd BRRTRouter && cargo build --bin brrtrouter-gen`  
then from RERP:  
`rerp gen stubs accounting --service accounts-payable --force`  
and  
`rerp build microservice accounts-payable`.

---

## 7. OpenAPI decimal/money audit and lint (money types)

We need to ensure OpenAPI specs use **correct decimal definitions** for money and other financial values, and to **lint misuse** of plain `type: number` (which maps to `f64` and is unsuitable for currency).

### 7.1 What is correct

- **Money / currency / accounting amounts** must use a decimal-capable type so that generated code uses `rust_decimal::Decimal` (or equivalent), not `f64`, to avoid rounding and representation errors.
- In OpenAPI 3.x, the correct way is:
  - **`type: number`** plus **`format: decimal`** → BRRTRouter generates `rust_decimal::Decimal`.
  - **`type: number`** plus **`format: money`** → same (Decimal for API; Money in domain if needed).
- **Incorrect:** `type: number` with **no** `format`, or `format: double` / `format: float` → generator maps to `f64`; using that for money is a misuse.

### 7.2 Audit result (RERP accounting specs)

- **accounts-payable** `openapi.yaml`: Every `type: number` in `components/schemas` (and inline response schemas) has **`format: decimal`** (e.g. `outstanding_amount`, `payment_amount`, `applied_amount`, `total_outstanding`, `current`, `days_31_60`, …, `exchange_rate`, `early_payment_discount_percent`). **No misuse found.**
- **Other accounting specs** (general-ledger, invoice, accounts-receivable, bank-sync, asset, budget, edi, financial-reports) and the aggregated **openapi_bff.yaml**: Grep shows `type: number` in schema properties is consistently paired with **`format: decimal`** in the same property block. **Conclusion:** RERP accounting OpenAPI files are currently correct for decimal/money; the risk is **future** misuse (new properties added as plain `number`).

### 7.3 How to lint misuse (money without decimal format)

Add a **linter rule** that flags properties that look like money/amounts but are declared as plain `type: number` without `format: decimal` or `format: money`.

**Rule (conceptual):**

- If a property has **`type` === `"number"`** and does **not** have **`format` in `["decimal", "money"]`**, and the property **name** matches a **money-related pattern**, then emit a **warning** (or configurable **error**):  
  *"Property 'X' looks like a monetary/amount field but has no format: decimal or format: money; use format: decimal for currency/amounts."*

**Money-related property names (pattern list):**

- Names that indicate money/amounts and should use decimal:
  - Contains: `amount`, `balance`, `price`, `cost`, `total`, `outstanding`, `payment_`, `applied_`, `exchange_rate`, `rate` (when numeric), `discount` (percent or amount), `depreciation`, `variance`, `credit_limit`, `credit_used`, `write_off`, `gain_loss`, `sale_proceeds`, `impairment`, `subtotal`, `tax_amount`, `discount_amount`, `paid_amount`, `unapplied`, `book_balance`, `bank_balance`, `difference`, `deposit`, `withdrawal`, `quantity` (when used with unit_price in line items), `unit_price`, `line_subtotal`, `line_total`, `opening_balance`, `closing_balance`, `total_debits`, `total_credits`, `budget_amount`, `actual_amount`, `total_budget_amount`, `total_actual_amount`, `total_variance`, `variance_percent`, `purchase_cost`, `current_value`, `accumulated_depreciation`, `salvage_value`, `depreciation_rate`, `document_total_amount`, `tax_rate`, `discount_percent`, etc.
- Implementation options:
  - **Strict:** Any `type: number` without `format: decimal` or `format: money` in a schema that also has a `currency_code` (or similar) property could be flagged.
  - **Name-based:** Maintain a list of substrings/suffixes (e.g. `_amount`, `amount`, `_balance`, `_price`, `total_`, `outstanding`, `payment_`, `exchange_rate`, `_rate` for numeric rates) and flag `type: number` without decimal/money format when the property name matches.

**Where to implement:**

- **BRRTRouter linter** (`src/linter.rs`): Extend **`lint_property`** (or add a helper called from it). For each property:
  1. If `property.type === "number"` and `property.format` is not `"decimal"` and not `"money"`.
  2. If the property name (e.g. `prop_name`) matches the money-related pattern list (e.g. contains `_amount`, `amount`, `_balance`, `balance`, `_price`, `price`, `total_`, `outstanding`, `payment_`, `exchange_rate`, `_rate`, `cost`, `variance`, etc.).
  3. Push a **LintIssue** (e.g. severity **Warning**, kind **`money_type_without_decimal_format`**) with message and suggestion: add `format: decimal` (or `format: money`).

**Call points (same as existing OpenAPI lint):**

- **RERP:** Each microservice has a Tilt resource **`<service>-lint`** (e.g. **accounts-payable-lint**) that runs:  
  `brrtrouter-gen lint --spec ./openapi/accounting/<service>/openapi.yaml --fail-on-error`  
  (see **Tiltfile** `create_microservice_lint(name, 'accounting/%s/openapi.yaml' % name)`).
- **BRRTRouter:**  
  `cargo run --bin brrtrouter-gen -- lint --spec <path-to-openapi.yaml> [--fail-on-error]`  
  implemented in **`src/cli/commands.rs`** (Commands::Lint) → **`lint_spec(spec_path)`** in **`src/linter.rs`** → **`lint_schema`** → **`lint_schema_object`** → **`lint_property`** for each property.

Once the new check is in **`lint_property`**, every existing RERP lint run (Tilt or local `brrtrouter-gen lint`) will report misuse of non-decimal types for money-like fields.

### 7.4 Summary

| Item | Status / action |
|------|------------------|
| **Correct definition** | `type: number` + `format: decimal` (or `format: money`) for any monetary/amount field. |
| **Audit** | RERP accounting OpenAPI files (including accounts-payable) currently use `format: decimal` consistently for number types; no current misuse. |
| **Lint rule** | Add in BRRTRouter **`lint_property`**: flag `type: number` without `format: decimal`/`money` when property name matches money-related patterns; suggest adding `format: decimal`. |
| **Invocation** | Unchanged: `brrtrouter-gen lint --spec <openapi.yaml>`, and RERP Tilt **`<service>-lint`** resources; no RERP code change required once the rule is in BRRTRouter. |
