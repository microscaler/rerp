# CRM-RT-05: rerp_crm_runtime — the condition tree

**Crate:** `microservices/crm/runtime` (module `conditions`) · **Build step:** 1 · **Depends on:** CRM-AUD-00 · **Points:** 8

## Goal

One implementation of the two-level condition tree used by segments, campaign audiences, automation IF/ELSE, GOAL and
WAIT steps, trigger filters and export filters.

## Read first

- `ConditionGroup`, `ConditionSubgroup` and `Condition` in `openapi/crm/audience/openapi.yaml` (subjects and operators
  are in the `Condition.subject` description and the operator enum).
- `openapi/crm/docs/data-model/audience.yaml` → `contacts`, `contact_tags`, `subscriptions`, `contact_facts`.

## API to provide

```rust
pub fn validate(tree: &ConditionGroup) -> Result<(), Vec<FieldError>>;   // known subjects, operators valid for them
/// Parameterised SQL selecting matching contact ids in the current workspace (RLS applies), for segments and audiences.
pub fn to_sql(tree: &ConditionGroup) -> (String, Vec<SqlParam>);
/// One contact, for automation: the facts the tree needs are loaded by the caller.
pub fn evaluate(tree: &ConditionGroup, facts: &ContactFacts) -> bool;
```

## Acceptance criteria

- AC-1 Every subject in the contract's list and every operator compiles to SQL; unknown ones are rejected by `validate`.
- AC-2 `to_sql` and `evaluate` agree on a generated set of 1 000 contacts and 200 random trees.
- AC-3 Segment evaluation p95 ≤ 700 ms at 100 k contacts on the local Postgres with the indexes in the data model.
- AC-4 No string concatenation of values into SQL (parameters only).
