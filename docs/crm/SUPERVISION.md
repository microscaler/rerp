# CRM implementation — supervising the coding agent

How a story the local model finished is reviewed before it is merged from `hermes/crm` to `main`. The aim is that the
supervisor reads the report, runs the gates, looks at a few specific things, and either merges or sends the story back
with a precise list.

## 1. Mechanical gates (all must pass)

```bash
git fetch && git log --oneline main..hermes/crm          # one commit per story
git diff --stat main...hermes/crm -- openapi/ microservices/crm/*/gen   # must be empty (no contract or gen edits)
python3 openapi/crm/tools/check_specs.py
python3 openapi/crm/tools/data_model.py --models          # data-model stories: the service reports nothing
cd microservices && cargo fmt --all -- --check
cargo clippy -p <crate> --all-targets -- -D warnings && cargo test -p <crate>
```

For data-model stories also: `microservices/crm/scripts/crm-db.sh reset && crm-db.sh up && crm-db.sh apply <service>`
twice, then the RLS test from the story.

## 2. Review checklist (read the diff with these in mind)

| Check | Look for |
|---|---|
| Controllers are thin | No SQL, no loops over records, no business rule in `controllers/` |
| One transaction per write | Audit row and outbox row inside the same transaction as the change |
| Workspace | Every query runs under the context's transaction; no `workspace_id` taken from the body |
| Roles | `require_role` with exactly the operation's `x-required-role` |
| Errors | Shared `ErrorResponse` with the reasons the story names; no 500 for a user error |
| Events | Published only through the outbox; names match `x-events` |
| Coroutines | No blocking calls in handlers (`std::thread::sleep`, `reqwest::blocking`, `std::net` DNS) |
| Personal data | No emails, names or IPs logged above debug |
| Tests | One test per acceptance criterion; isolation and role tests present; no `#[ignore]` added |
| Scope | Nothing outside the story's files; no new dependency without approval |

## 3. Decide

- **Merge** when the gates pass and the checklist is clean: fast-forward or rebase `hermes/crm` onto `main`, update the
  story's status in `docs/crm/README.md`, and move to the next story.
- **Send back** with a list of numbered findings appended to the story's report under **Review** — each finding names
  the file and line and the expected change. The agent fixes only those.
- **Fix the design** when the report shows a contract, event or data-model problem: change the source (spec, AsyncAPI,
  data-model YAML), regenerate (`rerp gen suite crm --service <service>`, `check_specs.py --docs`,
  `data_model.py --docs`), commit on `main`, and tell the agent to rebase.

## 4. Known tooling notes

- `rerp gen stubs crm <service> --force` does not overwrite existing stubs today. To refresh stubs after a contract
  change, run `brrtrouter-gen generate-stubs --spec openapi/crm/<service>/openapi.yaml --output <tmp>
  --component-name rerp_crm_<service>_gen` and copy only the stubs you need; never overwrite a `// BRRTRouter: user-owned`
  controller.
- The migrator (`--features crm`) refuses to run until at least one CRM LifeModel exists (`no registered entity models`).
