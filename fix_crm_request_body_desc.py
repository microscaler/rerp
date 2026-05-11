"""
Fix: Add descriptions to ALL 69 request bodies across 11 CRM specs.

Every POST/PUT/PATCH operation has a requestBody but none have descriptions.
This adds clear, consistent descriptions for each write operation.
"""
import yaml
import os

crm_base = '/home/casibbald/Workspace/microscaler/rerp/openapi/crm/'

# Request body descriptions mapped by service and operation
REQUEST_BODY_DESCRIPTIONS = {
    # === ACCOUNTS ===
    'accounts': {
        'POST /accounts': 'Create a new account/organization in the CRM',
        'PUT /accounts/{id}': 'Update an existing account by ID',
    },
    
    # === AUTOMATION ===
    'automation': {
        'POST /workflows': 'Create a new workflow automation',
        'PUT /workflows/{id}': 'Update an existing workflow',
        'POST /workflows/{id}/rules': 'Add a rule to a workflow',
        'POST /rules': 'Create a new workflow rule',
        'PUT /rules/{id}': 'Update an existing rule',
        'POST /triggers': 'Create a new workflow trigger',
        'PUT /triggers/{id}': 'Update an existing trigger',
        'POST /schedules': 'Create a scheduled action',
        'POST /recurring': 'Create a recurring action',
    },
    
    # === CONTACTS ===
    'contacts': {
        'POST /contacts': 'Create a new contact person',
        'PUT /contacts/{id}': 'Update an existing contact',
        'POST /contacts/{id}/merge': 'Merge two duplicate contacts',
        'POST /contacts/{id}/tree': 'Build the contact hierarchy tree',
    },
    
    # === ENGAGEMENT ===
    'engagement': {
        'POST /subscriptions': 'Create a new customer subscription',
        'PUT /subscriptions/{id}': 'Update an existing subscription',
        'PUT /subscriptions/{id}/renew': 'Renew a subscription',
        'POST /goals': 'Create a new performance goal',
        'PUT /goals/{id}': 'Update an existing goal',
        'PUT /goals/{id}/progress': 'Update goal progress and milestones',
        'POST /badges/award': 'Award a badge to a user',
    },
    
    # === INTELLIGENCE ===
    'intelligence': {
        'POST /leads/score': 'Calculate lead score based on criteria',
        'POST /leads/score-batch': 'Batch score multiple leads',
        'POST /leads/enrich': 'Enrich lead data from external sources',
        'POST /leads/enrich-batch': 'Batch enrich lead data',
        'POST /verify/email': 'Verify an email address',
        'POST /verify/phone': 'Verify a phone number',
        'POST /mining/search': 'Search for ICP-matched leads',
        'POST /mining/icp-match': 'Find leads matching ideal customer profile',
        'POST /scoring/frequencies': 'Create a scoring frequency configuration',
        'PUT /scoring/thresholds': 'Update lead scoring thresholds',
    },
    
    # === LIVECHAT ===
    'livechat': {
        'POST /chats': 'Create a new chat session',
        'POST /chats/{id}/message': 'Send a message in a chat session',
        'POST /chats/{id}/convert': 'Convert a chat session to a lead',
        'POST /chats/{id}/transcript': 'Retrieve chat transcript',
        'POST /agents': 'Create a new agent',
        'PUT /agents/{id}': 'Update an existing agent',
        'PUT /agents/{id}/status': 'Update agent online status',
    },
    
    # === MARKETING ===
    'marketing': {
        'POST /utm/campaigns': 'Create a new UTM campaign',
        'PUT /utm/campaigns/{id}': 'Update a UTM campaign',
        'POST /utm/mediums': 'Create a new UTM medium type',
        'POST /utm/sources': 'Create a new UTM source type',
        'POST /forms/{id}/submit': 'Submit a marketing form',
        'POST /analytics/track': 'Track a visitor page view',
        'POST /analytics/visitors/identify': 'Identify a returning visitor',
        'POST /events/register': 'Register for an event',
        'POST /surveys/submit': 'Submit a survey response',
    },
    
    # === PIPELINE ===
    'pipeline': {
        'POST /leads': 'Create a new lead',
        'PUT /leads/{id}': 'Update an existing lead',
        'PATCH /leads/{id}/stage': 'Change lead to a new pipeline stage',
        'POST /leads/{id}/convert': 'Convert a lead to contact and/or account',
        'POST /leads/detect-duplicates': 'Check for duplicate leads',
        'POST /leads/{id}/merge': 'Merge duplicate leads',
        'POST /stages': 'Create a new pipeline stage',
        'PUT /stages/{id}': 'Update a pipeline stage',
    },
    
    # === PLATFORM ===
    'platform': {
        'POST /custom-fields': 'Create a new custom field definition',
        'PUT /custom-fields/{id}': 'Update a custom field definition',
        'POST /webhooks': 'Create a new webhook endpoint',
        'PUT /webhooks/{id}': 'Update an existing webhook',
        'POST /api-keys': 'Create a new API key',
        'POST /integrations': 'Create a new integration',
        'PUT /integrations/{id}': 'Update an existing integration',
        'POST /integrations/{id}/sync': 'Sync integration data',
        'POST /reports/build': 'Build a report with current filters',
        'POST /reports/{id}/schedule': 'Schedule a report for periodic generation',
        'POST /reports/export': 'Export report data in specified format',
    },
    
    # === REPORTING ===
    'reporting': {
        'POST /reports/forecast': 'Generate a sales forecast',
        'POST /reports/performance': 'Generate performance report',
        'POST /reports/win-loss': 'Generate win/loss analysis report',
        'POST /reports/time-to-close': 'Generate time-to-close analysis',
    },
    
    # === TEAMS ===
    'teams': {
        'POST /teams': 'Create a new team',
        'PUT /teams/{id}': 'Update an existing team',
        'POST /teams/{id}/members': 'Add a member to a team',
        'POST /assign/run': 'Run automated lead assignment',
    },
}


def process_spec(spec_path, service):
    """Add descriptions to all request bodies in a spec."""
    with open(spec_path) as f:
        data = yaml.safe_load(f)
    
    modified = 0
    descriptions = REQUEST_BODY_DESCRIPTIONS.get(service, {})
    
    for path, methods in data['paths'].items():
        for method, operation in methods.items():
            if method not in ['post', 'put', 'patch']:
                continue
            
            request_body = operation.get('requestBody')
            if not request_body:
                continue
            
            key = f"{method.upper()} {path}"
            if key in descriptions and not request_body.get('description'):
                request_body['description'] = descriptions[key]
                modified += 1
    
    if modified:
        with open(spec_path, 'w') as f:
            yaml.dump(data, f, default_flow_style=False, sort_keys=False, allow_unicode=True)
    
    return modified


# Process all specs
total_fixed = 0
for item in sorted(os.listdir(crm_base)):
    if item.startswith('.') or item in ('CRM_ANALYSIS', 'docs'):
        continue
    if not os.path.isdir(os.path.join(crm_base, item)):
        continue
    
    spec_path = os.path.join(crm_base, item, 'openapi.yaml')
    if not os.path.exists(spec_path):
        continue
    
    fixed = process_spec(spec_path, item)
    if fixed:
        print(f"  {item}: {fixed} request body descriptions added")
        total_fixed += fixed

print(f"\nTotal: {total_fixed} request body descriptions added")
