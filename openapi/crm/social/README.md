# Social

Social publishing for the CRM: connected platform accounts, a content calendar of scheduled posts with per-platform
variants, an approval step, engagement pulled back per post and per account, and lead capture from social
interactions into `crm/pipeline`.

Moved from the former `openapi/marketing/social-media` stub and given a real model.

## Shape

- `SocialAccount` - one per platform handle, with OAuth state and a pause switch.
- `SocialPost` - default body, media and link, a list of `PostTarget`s (one per account, each with its own publish
  result), a schedule, and an optional approval gate. `source` names what produced the post (a person, an automation,
  an external worker) so a generated feed and a hand-written post live in the same calendar.
- `Engagement` - impressions, likes, comments, shares, clicks, saves; pulled on a schedule per platform.
- `/leads/capture` - a comment, DM, mention, follow or platform lead form becomes a lead in `crm/pipeline` with the
  UTM campaign stamped.
- `/audit` - who approved, skipped, paused or connected what.

Platform adapters (X, Instagram, Threads, LinkedIn, Facebook, Bluesky, Mastodon) are deployment configuration.
