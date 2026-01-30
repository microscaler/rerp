# Option B: Sync impl signatures from OpenAPI (design)

**Goal:** When the OpenAPI spec changes (e.g. new optional response fields), update impl controller **signatures and struct literals** to match without overwriting the rest of the function body (business logic).

**Current gap:** Option A (`brrtrouter-gen generate-stubs --force`) overwrites entire impl files. That keeps impl in sync with the spec but wipes any custom logic. Option B would allow “patch only the shape” so impl stays compilable and in sync while preserving hand-written logic.

---

## 1. Scope of “sync”

- **In scope:** Things that must match the gen crate and OpenAPI:
  - Function signature: handler type (request/response types from gen).
  - `Response { ... }` (and optionally `Request`) struct literal in the impl: add/remove fields to match the generated type; set new optional fields to `None` or a default.
- **Out of scope (preserved):** Everything else in the impl file:
  - Function body logic (validation, DB calls, mapping).
  - Comments, formatting, extra helpers in the same file.

So: **signatures + struct-initializer shape** are derived from OpenAPI; **bodies** are user-owned.

---

## 2. Sentinel: protect from overwrite (including `--force`)

To avoid accidental overwrite of custom logic when using `--force`, use a **sentinel line** inside the handler. The generator treats it as “user-owned: do not overwrite this function body.”

- **Sentinel (proposed):** A single line comment inside the handler function body, e.g.  
  `// BRRTRouter: user-owned`  
  or the shorter convention:  
  `// Implemented`

- **Rules:**
  - **Sentinel present** (anywhere in the handler function body):  
    - Even with `--force`, do **not** overwrite the function body.  
    - At most perform Option B “sync”: patch signature and `Response`/`Request` struct literal (add/remove fields to match gen); leave the rest of the body unchanged.  
    - So `--force` no longer wipes user logic for that handler.
  - **Sentinel absent:**  
    - Stub is treated as codegen-owned.  
    - `--force` (or initial generation) may overwrite the entire handler.  
  - **User removes the sentinel:**  
    - They opt back into “managed by codegen”; the next `generate-stubs --force` may overwrite that handler again.

- **Placement:** The sentinel must appear inside the handler function (e.g. right after `{` before the `Response { ... }`). Example:

```rust
#[handler(CreateEdiDocumentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // BRRTRouter: user-owned
    Response {
        document_number: req.document_number.clone(),
        // ... real logic
    }
}
```

This gives a clear, reversible way to protect handlers from the force concept while still allowing sync of signatures and struct shape.

---

## 3. Behaviour (high level)

- **Input:** OpenAPI spec + existing impl controller file(s) (e.g. `create_edi_document.rs`).
- **Output:** Same file path(s), with:
  - Handler signature updated if the gen types changed (e.g. response type gained `total_amount: Option<Decimal>`).
  - The `Response { ... }` (and if desired `Request { ... }`) literal in that handler updated: new optional fields added with `None`, removed fields deleted, so the struct matches the current gen type.
  - Rest of the function (and file) left as-is (no overwrite of business logic).

This could be a **new BRRTRouter command**, e.g.:

- `brrtrouter-gen sync-impl-signatures`  
  or  
- `brrtrouter-gen generate-stubs --sync-only`  
  (semantics: only patch signatures and struct literals; do not replace entire file.)

RERP could expose it as e.g. `rerp gen stubs <suite> [--service NAME] --sync` (distinct from `--force`).

---

## 4. Technical sketch

- **Discovery:** Same as today: from OpenAPI, get operation IDs and paths; from gen crate (or spec), get the Rust request/response type names and fields for each operation.
- **Per impl file:**
  - Parse the existing Rust file (e.g. with `syn` or a small AST/pattern pass).
  - Locate the handler function that corresponds to the operation (e.g. by name: `create_edi_document` ↔ operationId `createEdiDocument`).
  - **Sentinel check:** If the handler body contains the sentinel (e.g. `// BRRTRouter: user-owned` or `// Implemented`), do not overwrite the body; only apply sync (signature + struct literal merge). If `--force` is set but sentinel is present, still do not overwrite the body.
  - Compare current function signature (request/response types) with the types exported from gen for that operation. If the gen type has changed (e.g. new optional field), update the signature to use the current gen type.
  - Locate the `Response { ... }` (and optionally `Request { ... }`) initializer in that function. Build the “target” struct literal from the current gen type (required fields + optional fields with `None`). **Merge** into the existing literal:
    - Add new optional fields with `None` if missing.
    - Remove fields that no longer exist in the type.
    - Leave existing field values (and any user-written expressions) unchanged where the field still exists.
  - Write back only the modified regions (signature + one or two struct literals); leave the rest of the file untouched (no full-file overwrite).

- **Idempotency:** Running sync twice should be a no-op once impl and gen are aligned.
- **Safety:** If the file doesn’t parse or the handler/literal can’t be matched, fail with a clear error and do not overwrite the file.

---

## 5. UX (RERP side)

- **Option A (current):**  
  `rerp gen stubs accounting [--service edi] --force`  
  → Full overwrite of impl stubs (brrtrouter-gen generate-stubs --force).  
  **With sentinel (future):** Handlers that contain the sentinel are not overwritten; only signature + struct literal are synced.

- **Option B (future):**  
  `rerp gen stubs accounting [--service edi] --sync`  
  → Only sync signatures and Response/Request struct literals; keep bodies.  
  (Implementation in RERP: call new brrtrouter-gen sync-impl-signatures or equivalent.)

- **Default (no --force, no --sync):**  
  Same as today: create stubs only when they don’t exist; do not overwrite.

---

## 6. Implementation order (BRRTRouter)

1. **Design and agree** on CLI surface (`sync-impl-signatures` vs `--sync-only` on generate-stubs), sentinel text (e.g. `// BRRTRouter: user-owned` and optionally `// Implemented`), and how “struct literal merge” works (add/remove fields, keep existing values).
2. **Sentinel detection:** When generating/overwriting, scan handler body for the sentinel; if present, skip full overwrite and only run sync (signature + struct literal). Apply this even when `--force` is set.
3. **Rust parsing:** Use `syn` (or equivalent) to parse impl files and find handler + struct literals; implement a minimal “patch” that only touches signature and those literals.
4. **Type source:** Reuse existing logic that knows “this operation → this Request/Response type and fields” (from OpenAPI/gen) so the tool knows what the target shape is.
5. **Tests:** Idempotency; add optional field → sync → optional field present with `None`; remove field → sync → field removed from literal; body unchanged. Sentinel present + `--force` → body unchanged (sync only); sentinel absent + `--force` → full overwrite.
6. **RERP:** Once BRRTRouter has the command, add `rerp gen stubs ... --sync` that calls it.

---

## 7. Summary

| Option | Command / flag | Effect |
|--------|----------------|--------|
| A      | `--force`      | Overwrite entire impl stub files — **unless** handler contains sentinel (then only sync). Impl matches spec; custom logic is lost only where sentinel is absent. |
| B      | `--sync` (design) | Patch only handler signatures and Response/Request struct literals from OpenAPI; preserve function bodies. |

**Sentinel:** A line such as `// BRRTRouter: user-owned` or `// Implemented` inside the handler body marks it as user-owned. When present, even `--force` does not overwrite the body (only sync). Removing the sentinel opts the handler back into “can be overwritten” by the next `--force`.

Option A is implemented (RERP: `rerp gen stubs <suite> [--service NAME] --force` and `rerp bootstrap microservice <name> --force-stubs`). Option B and sentinel behaviour are this design for a future BRRTRouter feature.
