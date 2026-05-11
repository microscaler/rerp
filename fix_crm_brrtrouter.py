"""
Fix double-'s' pagination naming across intelligence, livechat, marketing, reporting, teams.

The previous run created:
  PaginatedLeadScoreSummarys → PaginatedLeadScoreSummary (correct)
  PaginatedLeadScoreRankeds → PaginatedLeadScoreRanked (correct)
  PaginatedAgentQueueItems → PaginatedAgentQueueItem (correct)
  PaginatedCampaignLeadResults → PaginatedCampaignLeadResult (correct)
  PaginatedLeadSourceReports → PaginatedLeadSourceReport (correct)
  PaginatedLeaderboardEntrys → PaginatedLeaderboardEntry (correct)
  PaginatedForecastAccuracyEntrys → PaginatedForecastAccuracyEntry (correct)
  PaginatedUnassignedLeads → correct, leave as-is

Also update all $ref references accordingly.
"""
import yaml
import os

base = '/home/casibbald/Workspace/microscaler/rerp/openapi/crm/'

# Map of wrong → correct names
RENAMES = {
    # intelligence
    ('intelligence', 'PaginatedLeadScoreSummarys'): 'PaginatedLeadScoreSummary',
    ('intelligence', 'PaginatedLeadScoreRankeds'): 'PaginatedLeadScoreRanked',
    # livechat
    ('livechat', 'PaginatedAgentQueueItems'): 'PaginatedAgentQueueItem',
    # marketing
    ('marketing', 'PaginatedCampaignLeadResults'): 'PaginatedCampaignLeadResult',
    # reporting
    ('reporting', 'PaginatedLeadSourceReports'): 'PaginatedLeadSourceReport',
    ('reporting', 'PaginatedLeaderboardEntrys'): 'PaginatedLeaderboardEntry',
    ('reporting', 'PaginatedForecastAccuracyEntrys'): 'PaginatedForecastAccuracyEntry',
    # teams - this one is correct, skip
}

def update_refs(obj, old_ref, new_ref):
    """Recursively update $ref values."""
    if isinstance(obj, dict):
        if '$ref' in obj and isinstance(obj['$ref'], str) and old_ref in obj['$ref']:
            obj['$ref'] = obj['$ref'].replace(old_ref, new_ref)
        for v in obj.values():
            update_refs(v, old_ref, new_ref)
    elif isinstance(obj, list):
        for item in obj:
            update_refs(item, old_ref, new_ref)

print("=" * 80)
print("FIXING DOUBLE-'S' PAGINATION NAMING")
print("=" * 80)

for (svc, old_name), new_name in RENAMES.items():
    svc_path = os.path.join(base, svc, 'openapi.yaml')
    with open(svc_path) as f:
        data = yaml.safe_load(f)
    
    comps = data['components']
    schemas = comps['schemas']
    
    if old_name in schemas:
        schemas[new_name] = schemas.pop(old_name)
        print(f"  {svc}: {old_name} → {new_name}")
    
    # Update all $refs in the document
    ref_old = f"#/components/schemas/{old_name}"
    ref_new = f"#/components/schemas/{new_name}"
    
    for path, methods in data.get('paths', {}).items():
        for method, op in methods.items():
            for status_code, resp in op.get('responses', {}).items():
                for ct, cv in resp.get('content', {}).items():
                    s = cv.get('schema', {})
                    if isinstance(s, dict) and ref_old in s.get('$ref', ''):
                        cv['schema']['$ref'] = s['$ref'].replace(ref_old, ref_new)
            
            req = op.get('requestBody', {})
            if req:
                for ct, cv in req.get('content', {}).items():
                    s = cv.get('schema', {})
                    if isinstance(s, dict) and ref_old in s.get('$ref', ''):
                        cv['schema']['$ref'] = s['$ref'].replace(ref_old, ref_new)
    
    for sname, s in schemas.items():
        update_refs(s, ref_old, ref_new)
    
    with open(svc_path, 'w') as f:
        yaml.dump(data, f, default_flow_style=False, sort_keys=False, width=120)

print("\nDone!")
