# ActiveCampaign reference tenant

A trial account configured by script so that every object shape of a market-leading marketing platform
can be pulled through its v3 API and diffed field by field against `openapi/crm/*`. See
`docs/06-marketing-feature-map.md` for the feature inventory this supports.

- `seed.py` — idempotent seed (lists, fields, tags, 20 contacts, pipeline + stages + deals + tasks) and a
  read-only dump of every other resource. Credentials come from the environment, never from the repo:
  `set -a; . ~/.config/activecampaign/key; set +a; python3 seed.py` (`--dump-only` skips creation).
- `dump/` — raw API responses, one file per object. Committed so the diff is reviewable; no credentials or
  real customer data are in it (all contacts are synthetic `.example` addresses apart from the account owner).
- `shape-diff.md` — the running comparison of their fields against ours, written as the diffs are worked.

Objects that only the UI can create on a trial (campaign content, automation graphs, score rules, forms,
conversions, goals) are built by hand and then dumped here.
