#!/usr/bin/env python3
"""Seed an ActiveCampaign trial as a reference tenant and dump every object shape.

Reads AC_URL and AC_KEY from the environment (source ~/.config/activecampaign/key first; the key is never
printed). Idempotent: looks objects up by name before creating. Writes the raw API responses under
./dump/<resource>/<id>.json next to this script so the shapes can be diffed against openapi/crm/*.

Usage:  set -a; . ~/.config/activecampaign/key; set +a; python3 seed.py [--dump-only]
"""
import json
import os
import pathlib
import sys
import urllib.error
import urllib.parse
import urllib.request

URL = os.environ["AC_URL"].rstrip("/") + "/api/3"
KEY = os.environ["AC_KEY"]
HERE = pathlib.Path(__file__).resolve().parent
DUMP = HERE / "dump"
DUMP_ONLY = "--dump-only" in sys.argv


def api(method, path, body=None, params=None):
    url = URL + path + (("?" + urllib.parse.urlencode(params)) if params else "")
    data = json.dumps(body).encode() if body is not None else None
    req = urllib.request.Request(url, data=data, method=method, headers={"Api-Token": KEY, "Content-Type": "application/json", "Accept": "application/json"})
    try:
        with urllib.request.urlopen(req, timeout=40) as r:
            return json.loads(r.read() or b"{}")
    except urllib.error.HTTPError as e:
        msg = e.read().decode()[:400]
        print(f"  ! {method} {path} -> {e.code}: {msg}")
        return None


def dump(resource, obj):
    d = DUMP / resource
    d.mkdir(parents=True, exist_ok=True)
    (d / f"{obj.get('id', 'x')}.json").write_text(json.dumps(obj, indent=2, sort_keys=True))


def find(resource, key, name, params=None):
    res = api("GET", f"/{resource}", params=dict(params or {}, limit=100))
    for o in (res or {}).get(resource, []):
        if o.get(key) == name:
            return o
    return None


def ensure(resource, key, name, body, singular=None):
    singular = singular or resource[:-1]
    o = find(resource, key, name)
    if o is None and not DUMP_ONLY:
        res = api("POST", f"/{resource}", {singular: body})
        o = (res or {}).get(singular)
        print(f"  + {resource}: {name}")
    elif o is not None:
        print(f"  = {resource}: {name} (#{o['id']})")
    if o:
        dump(resource, o)
    return o


# ---------------------------------------------------------------- lists
print("lists")
lists = {}
for name, desc, dbl in [
    ("Newsletter", "Monthly product news", True),
    ("Customers", "Account and billing updates", False),
    ("Webinars", "Invitations and recordings", True),
    ("Staff test list", "Only staff addresses; campaigns are sent here", False),
]:
    o = ensure("lists", "name", name, {"name": name, "stringid": name.lower().replace(" ", "-"), "sender_url": "https://example.com", "sender_reminder": "You signed up on our website.", "send_last_broadcast": 0, "subscription_notify": "", "unsubscription_notify": ""})
    if o:
        lists[name] = o["id"]

# ---------------------------------------------------------------- custom fields
print("fields")
fields = {}
for title, ftype, perstag, opts in [
    ("Plan", "dropdown", "PLAN", ["Free", "Team", "Business"]),
    ("Renewal date", "date", "RENEWAL_DATE", None),
    ("Seats", "text", "SEATS", None),
    ("Team size", "dropdown", "TEAM_SIZE", ["1-10", "11-50", "51+"]),
    ("Industry", "text", "INDUSTRY", None),
]:
    o = ensure("fields", "title", title, {"title": title, "type": ftype, "perstag": perstag, "visible": 1})
    if o:
        fields[title] = o["id"]
        if opts and not DUMP_ONLY:
            api("POST", "/fieldOption/bulk", {"fieldOptions": [{"field": o["id"], "label": v, "value": v, "orderid": i + 1} for i, v in enumerate(opts)]})
        if not DUMP_ONLY:
            api("POST", "/fieldRels", {"fieldRel": {"field": o["id"], "relid": 0}})

# ---------------------------------------------------------------- tags
print("tags")
tags = {}
for t in ["customer", "trial", "waitlist", "enterprise", "smb", "webinar-oct", "pricing-page", "lead", "churned", "vip"]:
    o = ensure("tags", "tag", t, {"tag": t, "tagType": "contact", "description": ""})
    if o:
        tags[t] = o["id"]

# ---------------------------------------------------------------- contacts
print("contacts")
first = ["Priya", "Tom", "Lena", "Marcus", "Sofia", "Daniel", "Aisha", "Jonas", "Mei", "Oliver", "Chiara", "Noah", "Fatima", "Lucas", "Ingrid", "Ravi", "Elena", "Kofi", "Hanna", "Diego"]
last = ["Natarajan", "Okafor", "Fischer", "Reid", "Alvarez", "Kim", "Bello", "Weber", "Lin", "Brown", "Rossi", "Schmidt", "Hassan", "Moreau", "Larsen", "Patel", "Petrova", "Mensah", "Novak", "Silva"]
companies = ["Northwind", "Fabrikam", "Contoso", "Adventure Works", "Tailspin", "Wide World", "Litware", "Proseware", "Coho", "Alpine Ski"]
contacts = []
for i in range(20):
    email = f"{first[i].lower()}.{last[i].lower()}@{companies[i % 10].lower().replace(' ', '')}.example"
    body = {"email": email, "firstName": first[i], "lastName": last[i], "phone": f"+4930{1000000 + i * 7919}",
            "fieldValues": [{"field": fields.get("Plan"), "value": ["Free", "Team", "Business"][i % 3]},
                            {"field": fields.get("Seats"), "value": str(5 + i * 3)},
                            {"field": fields.get("Industry"), "value": ["Logistics", "Retail", "Finance", "SaaS"][i % 4]}]}
    o = find("contacts", "email", email, {"email": email})
    if o is None and not DUMP_ONLY:
        o = (api("POST", "/contact/sync", {"contact": body}) or {}).get("contact")
        print(f"  + contact {email}")
    if o:
        dump("contacts", o)
        contacts.append(o)
        if not DUMP_ONLY:
            lst = ["Newsletter", "Customers", "Webinars"][i % 3]
            api("POST", "/contactLists", {"contactList": {"list": lists[lst], "contact": o["id"], "status": 1}})
            for t in ([["customer"], ["trial"], ["waitlist"], ["lead", "pricing-page"]][i % 4]):
                api("POST", "/contactTags", {"contactTag": {"contact": o["id"], "tag": tags[t]}})
# staff list gets the account owner only
me = find("contacts", "email", "charles@pricewhisperer.ai", {"email": "charles@pricewhisperer.ai"})
if me and not DUMP_ONLY:
    api("POST", "/contactLists", {"contactList": {"list": lists["Staff test list"], "contact": me["id"], "status": 1}})

# ---------------------------------------------------------------- pipeline, stages, deals, tasks
print("pipeline")
pipe = ensure("dealGroups", "title", "Sales", {"title": "Sales", "currency": "eur", "allgroups": 1, "allusers": 1, "autoassign": 0}, "dealGroup")
stages = {}
if pipe:
    for i, (name, color) in enumerate([("New", "C1C1C1"), ("Qualified", "9DB6F7"), ("Proposal", "1F5AF6"), ("Negotiation", "6B4EE6")]):
        s = find("dealStages", "title", name, {"filters[d_groupid]": pipe["id"]})
        if s is None and not DUMP_ONLY:
            s = (api("POST", "/dealStages", {"dealStage": {"title": name, "group": pipe["id"], "order": i + 1, "color": color, "dealOrder": "next-action DESC", "cardRegion1": "title", "cardRegion2": "next-action", "cardRegion3": "show-avatar", "cardRegion4": "contact-fullname-orgname", "cardRegion5": "value"}}) or {}).get("dealStage")
        if s:
            stages[name] = s["id"]
            dump("dealStages", s)
print("deals")
deals = []
for i, c in enumerate(contacts[:8]):
    title = f"{companies[i % 10]} · {['trial to team', 'pricing enquiry', '25 seats', 'pilot', 'expansion', 'Business plan', '3-year', 'migration'][i]}"
    stage = list(stages.values())[i % max(1, len(stages))] if stages else None
    d = find("deals", "title", title)
    if d is None and not DUMP_ONLY and stage:
        d = (api("POST", "/deals", {"deal": {"title": title, "contact": c["id"], "value": str((i + 1) * 480000), "currency": "eur", "group": pipe["id"], "stage": stage, "owner": 1, "status": 0}}) or {}).get("deal")
        print(f"  + deal {title}")
    if d:
        dump("deals", d)
        deals.append(d)
print("tasks")
for i, d in enumerate(deals[:4]):
    t = find("dealTasks", "title", f"Follow up on {d['title']}")
    if t is None and not DUMP_ONLY:
        t = (api("POST", "/dealTasks", {"dealTask": {"title": f"Follow up on {d['title']}", "relid": d["id"], "reltype": "Deal", "duedate": "2026-11-12T10:00:00+02:00", "dealTasktype": 1, "note": "Seeded task"}}) or {}).get("dealTask")
    if t:
        dump("dealTasks", t)

# ---------------------------------------------------------------- scores (read only - UI-created), segments, forms, campaigns, automations
print("read-only dumps (scores, segments, forms, campaigns, automations, messages, siteTracking, conversions)")
for res in ["scores", "segments", "forms", "campaigns", "automations", "messages", "siteTrackingDomains", "conversions", "goals", "addresses", "users", "brandings", "accounts", "dealGroups", "dealStages", "dealTasktypes", "dealRoles", "fields", "tags", "lists", "contactLists", "contactTags", "contactAutomations", "automationTriggers", "webhooks", "savedResponses", "calendars", "templates", "ecomCustomers"]:
    r = api("GET", f"/{res}", params={"limit": 100})
    if r is None:
        continue
    items = r.get(res) or next((v for k, v in r.items() if isinstance(v, list)), [])
    for o in items:
        if isinstance(o, dict):
            dump(res, o)
    print(f"  {res}: {len(items)}")

print("done; dump at", DUMP)
