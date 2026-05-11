"""
Fix: Add descriptions to all shared parameters across 11 CRM specs.

Fixes 40/40 missing parameter descriptions (0% → 100%).
"""
import yaml
import os

crm_base = '/home/casibbald/Workspace/microscaler/rerp/openapi/crm/'

PARAM_DESCRIPTIONS = {
    'Id': 'Unique identifier for the resource',
    'Page': 'Page number for pagination (1-indexed)',
    'Limit': 'Maximum number of items per page (default: 20, max: 100)',
    'search': 'Search query string for filtering resources',
    'Search': 'Search query string for filtering resources',
    'filter_parent_id': 'Filter by parent resource ID',
    'filter_industry_id': 'Filter by industry category ID',
    'filter_status': 'Filter by status value',
    'filter_team_id': 'Filter by team membership ID',
    'filter_email': 'Filter by email address',
    'filter_stage': 'Filter by pipeline stage',
    'filter_source': 'Filter by lead source',
    'filter_assigned_to': 'Filter by assigned representative ID',
    'filter_period': 'Filter by time period (daily, weekly, monthly, quarterly)',
    'filter_status_active': 'Filter by active/inactive status',
    'entity': 'Entity type for custom field lookup (e.g., lead, contact, account)',
}


def process_spec(spec_path):
    """Add descriptions to all shared parameters in a spec."""
    with open(spec_path) as f:
        data = yaml.safe_load(f)
    
    modified = 0
    parameters = data.get('components', {}).get('parameters', {})
    
    for param_name, param_def in parameters.items():
        if isinstance(param_def, dict) and not param_def.get('description'):
            if param_name in PARAM_DESCRIPTIONS:
                param_def['description'] = PARAM_DESCRIPTIONS[param_name]
                modified += 1
    
    if modified:
        with open(spec_path, 'w') as f:
            yaml.dump(data, f, default_flow_style=False, sort_keys=False, allow_unicode=True)
    
    return modified


# Process all specs
total_fixed = 0
for item in sorted(os.listdir(crm_base)):
    spec_path = os.path.join(crm_base, item, 'openapi.yaml')
    if not os.path.exists(spec_path):
        continue
    
    fixed = process_spec(spec_path)
    if fixed:
        print(f"  {item}: {fixed} parameter descriptions added")
        total_fixed += fixed

print(f"\nTotal: {total_fixed} parameter descriptions added")
