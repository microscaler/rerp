# Ad buying landscape — who sells impressions, how to hook in, and what "doing it ourselves" means

> **Note (2026-11-09):** `crm/marketing` `/ads/*` now lives in crm/acquisition ([acquisition/openapi.yaml](../acquisition/openapi.yaml)).

Companion to `crm/marketing` `/ads/*` (shape-diff § Ad audiences, ad tracking and attribution). Written
2026-11-06 from vendor pages and trade press; minimums and fees move, so re-check before signing anything.

## 1. The three ways to buy impressions

| Tier | What it is | Examples | API | Practical minimum | Fee | Our adapter kind |
|---|---|---|---|---|---|---|
| **Walled-garden APIs** | The platform sells its own inventory; audiences, campaigns, creatives, conversions all through one marketing API | Meta Marketing API, Google Ads API, LinkedIn Marketing API, X Ads API, TikTok Marketing API, Microsoft Advertising API, Reddit Ads API, Pinterest Ads API, Amazon Ads API (DSP self-serve since late 2025) | Yes, all of them, OAuth app review required | None hard; Google Customer Match needs ~1,000 matched, Meta ~100 | Built into the auction | `WALLED_GARDEN_API` |
| **Self-serve DSPs with an advertiser API** | A buyer-side platform bidding across many exchanges (open auction, PMP deals, guaranteed) | Adform (full API, strongest in EMEA), StackAdapt (self-serve, ~$5k/month recommended), Admixer DSP (self-serve + managed, CRM onboarding + DMP, open auction / PMP / direct deals), SmartyAds ($1k prepay), Epom white-label (5% of spend, $250–$2,000/month floor, 5,000 QPS) | Adform yes; StackAdapt and Admixer on request; Epom yes (white-label) | $1k–$5k/month | ~5–20% take on media | `SELF_SERVE_DSP_API` |
| **Big DSPs** | Same, at scale, sold through agencies | The Trade Desk ($25k–$50k+/month via agency; ~20% take), DV360 (~$50k/month, GMP contract or reseller), Amazon DSP managed ($50k) | TTD and DV360 have APIs once you have a seat | $25k+/month | 15–20% | `SELF_SERVE_DSP_API` (later) |
| **Bidder-as-a-service** | Our bidding logic and models, their bidder, exchange integrations and QPS | Beeswax (FreeWheel) — subscription, custom algorithms allowed | Yes | Subscription (unpublished) + media | subscription | `BIDDER_AS_A_SERVICE` |
| **Own OpenRTB bidder** | We answer bid requests from SSPs / exchanges ourselves | Bidswitch (exchange-of-exchanges), direct SSP seats (Magnite, PubMatic, Index, Admixer.Network oRTB) | OpenRTB 2.6 | Contract + QPS commitment per SSP; each integration is separate work | none (servers + people) | `OPENRTB_BIDDER` |

Sources: [SmartyAds self-serve DSP round-up](https://smartyads.com/blog/top-self-serve-programmatic-dsps-agencies),
[amBrain on Trade Desk alternatives and own-DSP costs](https://ambrain.org/adtech/alternatives/the-trade-desk/),
[Admixer DSP](https://admixer.com/dsp/), [Admixer.Network oRTB demand](https://blog.admixer.com/admixer-network-releases-ortb-demand/amp/),
[StackAdapt vs TTD](https://resources.rework.com/tools/advertising/stackadapt-vs-the-trade-desk), [G2 DSP review](https://learn.g2.com/best-demand-side-platforms).

## 2. What "handle our ad impressions ourselves" actually means

The ad is always *served* by the platform or the SSP; what we can own is the **buying decision** (who, when, how
much) and the **feedback loop** (what happened afterwards). That gives three honest levels of control:

1. **Orchestrate through APIs (ship first).** One `AdBuySpec` (objective, budget, bid, schedule, frequency cap,
   targeting = our synced audience, placements, creatives) pushed through adapters to each platform; delivery
   pulled back daily; conversions pushed server-side; attribution and recommendations computed on our data.
   This is 90% of the Hyros value and 100% of the warm-up use case. No minimums beyond the platforms' own.
2. **Own the bidding logic, rent the bidder.** Beeswax-style: our models decide bid price per request (e.g. bid
   more for a contact in a WARM_UP audience three days before the send, zero after they clicked). Needed only
   when we want per-impression control the DSP UIs don't give. Subscription cost; one integration.
3. **Own the bidder.** Only worth it at real scale: 10k–100k QPS of bid requests (most lost), sub-100 ms
   responses in several regions, per-SSP contracts, traffic-quality filtering, consent handling per market. Not
   for a CRM product's first year; the spec keeps the `OPENRTB_BIDDER` adapter kind so the door stays open.

Recommendation: level 1 now with Meta, Google Ads, LinkedIn, Reddit and one EMEA DSP (Adform or Admixer) as the
first adapters; level 2 when a customer's spend justifies it.

## 3. The URL parameters we see on ad landings (what the snippet captures)

From the sample landings (Reddit Ads via Google PMax, top4smm, babylovegrowth.ai):

| Parameter | Who sets it | Meaning | We store it as |
|---|---|---|---|
| `gclid` | Google Ads auto-tagging | Click id; needed for offline-conversion upload | `click_ids.gclid` |
| `gbraid` / `wbraid` | Google, iOS 14+ app and web | Privacy-safe click ids where `gclid` is withheld | `click_ids.gbraid` |
| `gad_source`, `gad_campaignid` | Google (2024–25 additions) | 1 = Google Ads; the campaign id even when the advertiser's template omits it | `utm`/`platform` derivation |
| `utm_source/medium/campaign/term/content/id` | The advertiser's tracking template | Standard; `utm_campaign` is often the numeric campaign id, `utm_content` the ad id (babylovegrowth) | `utm` |
| `hsa_acc`, `hsa_cam`, `hsa_grp`, `hsa_ad`, `hsa_src`, `hsa_tgt`, `hsa_kw`, `hsa_mt`, `hsa_net`, `hsa_ver` | HubSpot Ads tracking template | Account, campaign, ad group, ad, source, target, keyword, match type, network, version (Reddit's landing is a HubSpot customer) | `platform_params` |
| `cid`, `agid`, `adid`, `kw`, `dev`, `net`, `acc`, `src` | A home-grown ValueTrack template (`{campaignid}`, `{adgroupid}`, `{creative}`, `{keyword}`, `{device}`, `{network}`) | Same information, advertiser-named (top4smm) | `platform_params` |
| `fbclid`, `ttclid`, `li_fat_id`, `msclkid`, `twclid`, `epik`, `rdt_cid` | Meta, TikTok, LinkedIn, Microsoft, X, Pinterest, Reddit | Platform click ids | `click_ids.*` |
| `pw_cid` | Us (link-tag rules) | Our click id on every outbound link | `click_ids.pw_cid` |

`AdTrackingSettings.click_id_params` lists the first group; `platform_params` keeps anything else that looks like
a tracking template so a customer's own naming still joins the journey. Our own tracking template for buys we
create is `?pw_cid={pw_cid}&utm_source={platform}&utm_medium=cpc&utm_campaign={buy.slug}&utm_content={creative.id}`,
plus the platform's `{campaignid}`-style macros where the adapter supports them.

## 4. Hooks per adapter (what each one needs from us)

| Adapter | Auth | Audiences | Conversions back | Campaign control | Spend | Notes |
|---|---|---|---|---|---|---|
| Meta | OAuth, app review (ads_management, ads_read) | Custom Audiences (hashed email/phone/MAID), lookalikes | Conversions API (dataset id, event_id dedupe) | campaign → ad set → ad; frequency cap on reach objective | Insights API | Audiences under ~100 do not serve |
| Google Ads | OAuth + developer token (basic → standard access) | Customer Match (≥1,000 matched), similar segments | Offline conversion import (gclid/gbraid), enhanced conversions (hashed email) | campaign → ad group → ad; PMax takes audience *signals* only | reports | Developer-token approval is the slow part |
| LinkedIn | OAuth, Marketing Developer Platform access | Matched Audiences (contact, company lists) | Conversions API | campaign group → campaign → creative | analytics | Best B2B match on work emails |
| Reddit | OAuth (Ads API) | Custom Audiences (hashed email, MAID) | Conversions API | campaign → ad group → ad | reports | `rdt_cid` click id |
| X | OAuth 1.0a, Ads API access tier | Custom Audiences | Conversion API | campaign → line item → promoted tweet | stats | Access reviews are slow |
| TikTok | OAuth (Marketing API) | Custom Audiences | Events API | campaign → ad group → ad | reports | |
| Microsoft | OAuth + developer token | Customer Match | Offline conversions (msclkid) | campaign → ad group → ad | reports | Imports Google campaigns |
| Adform / StackAdapt / Admixer | API key or OAuth | CRM onboarding / DMP segments | Pixel or S2S | line items with deal ids, frequency caps, dayparting | reports | Open-web display / native / video / CTV |
| Beeswax | API key | Segments | S2S | our bid models | logs | Level 2 |
| LiveRamp | contract | RampID onboarding to any DSP | — | — | — | Audiences only |

Each adapter reports its capability matrix through `GET /ads/adapters`; `AdBuy.accounts[].unsupported[]` says
what a buy could not express on a given platform instead of silently dropping it.
