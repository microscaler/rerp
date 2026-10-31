# PR Review Comment Merge Gate

- **Status**: `verified`
- **Source docs**: `CONTRIBUTING.md` ("Submit a Pull Request"), `microservices/accounting/BUILDOUT_ANALYSIS.md` (prerequisite 8)
- **Code anchors**: `docs/llmwiki/log.md` entries dated 2026-10-31
- **Last updated**: 2026-10-31

## The rule

A pull request may be merged only when BOTH hold:

1. All CI checks are green on the final head commit, and
2. Every review comment on that PR has been addressed — fixed in a follow-up
   commit on the same PR, or answered with a reasoned reply — before the
   merge. This includes comments that arrive after the initial review pass.

CI green is a necessary condition, never a sufficient one. This gate applies
to every PR without exception, **including review-backlog PRs**: a PR whose
purpose is resolving review comments on earlier PRs is itself subject to the
same gate. Merging a review-fix PR while it carries unresolved review
comments (as happened on PR #129) is a process violation, not a shortcut.

## Auditing a PR before merge

Bot reviews split findings across three surfaces; fetching only one silently
misses findings:

```bash
# inline review comments (with file/line anchors) — --paginate because a
# multi-page comment list would otherwise hide unanswered findings
gh api --paginate repos/microscaler/rerp/pulls/<n>/comments
# PR conversation comments
gh api --paginate repos/microscaler/rerp/issues/<n>/comments
# review bodies — "Follow-up findings (not blocking merge)" style sections
# live here and have no thread to reply into
gh api --paginate repos/microscaler/rerp/pulls/<n>/reviews --jq '.[].body'
```

- Body-only findings get a PR issue comment as the reply surface.
- A bot review with state `COMMENTED` and an empty body usually means the bot
  hit a usage limit, not "nothing found" — check review timestamps after each
  push.
- After posting fixes, re-fetch all three surfaces on the NEW head commit and
  confirm zero unanswered threads; re-verify `headRefOid` equals the fix
  commit so stale watcher results cannot authorize an early merge.

## After merge

- Anything a review caught that was fixed post-merge still needs the original
  thread answered with the commit that fixed it.
- Never rewrite `docs/llmwiki/log.md` history to reflect a correction; the
  log is append-only — add a dated correction entry at the bottom.
