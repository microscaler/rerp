"""
Fix: Add examples to 37 operations with inline schemas.

These operations don't use $ref - they define their response schema inline
as type: object with properties. We generate examples from the properties.
"""
import yaml
import os
import copy

crm_base = '/home/casibbald/Workspace/microscaler/rerp/openapi/crm/'

# Custom examples for specific operations that need hand-crafted responses
SPECIFIC_EXAMPLES = {
    'automation': {
        'POST /triggers/{id}/fire': {'success': True, 'trigger_id': '00000000-0000-0000-0000-000000000008', 'workflow_execution_id': '00000000-0000-0000-0000-000000000032'},
        'POST /rules/test': {'rule_id': '00000000-0000-0000-0000-000000000007', 'condition_met': True, 'actions_fired': 1},
    },
    'intelligence': {
        'POST /leads/score-batch': {'scores_computed': 5, 'time_ms': 120},
        'POST /leads/score-bulk': {'scores_computed': 10, 'time_ms': 250},
        'POST /scoring/frequencies/rebuild': {'success': True, 'frequencies_rebuilt': 3},
        'PUT /scoring/thresholds': {'hot': 80, 'warm': 50, 'cold': 20},
        'POST /enrichment/lookup': {'company': {'name': 'Example Inc', 'domain': 'example.com', 'industry': 'Technology'}, 'person': {'name': 'John Doe', 'title': 'CTO'}},
        'POST /enrichment/website': {'company': {'name': 'Target Corp', 'domain': 'target.com', 'industry': 'Retail'}, 'confidence': 0.85},
        'POST /enrichment/batch': {'enriched': 10, 'failed': 0, 'results': []},
        'POST /enrichment/auto-fill': {'company': {'name': 'Example Inc', 'domain': 'example.com'}, 'person': {'name': 'John Doe'}},
        'POST /verify/email': {'email': 'test@example.com', 'valid': True, 'disposable': False, 'role_based': False},
        'POST /verify/batch': {'verified': 10, 'valid': 8, 'invalid': 2},
        'POST /verify/phone': {'phone': '+1-555-0123', 'valid': True, 'carrier': 'Verizon', 'phone_type': 'mobile'},
        'POST /mining/search': {'total_matches': 150, 'leads': [{'name': 'John Doe', 'email': 'john@target.com', 'company': 'Target Corp'}]},
        'POST /mining/icp-match': {'total_matches': 50, 'leads': [{'name': 'Jane Smith', 'email': 'jane@ideal.com', 'company': 'Ideal Corp'}]},
    },
    'contacts': {
        'POST /contacts/merge': {'success': True, 'kept_id': '00000000-0000-0000-0000-000000000001', 'merged_id': '00000000-0000-0000-0000-000000000002'},
    },
    'engagement': {
        'POST /subscriptions/{id}/renew': {'success': True, 'subscription_id': '00000000-0000-0000-0000-000000000013', 'new_end_date': '2025-01-01T00:00:00Z'},
    },
    'livechat': {
        'POST /chats/convert': {'success': True, 'lead_id': '00000000-0000-0000-0000-000000000003', 'chat_id': '00000000-0000-0000-0000-000000000009'},
    },
    'marketing': {
        'POST /analytics/visitors/identify': {'visitor_id': 'v001', 'identified': True, 'email': 'user@example.com'},
        'GET /campaigns/{id}/revenue': {'campaign_id': '00000000-0000-0000-0000-000000000012', 'total_revenue': 25000, 'conversions': 15},
    },
    'pipeline': {
        'POST /leads/{id}/convert': {'success': True, 'contact_id': '00000000-0000-0000-0000-000000000002', 'account_id': '00000000-0000-0000-0000-000000000001'},
        'POST /leads/detect-duplicates': {'is_duplicate': True, 'matches': [{'id': '00000000-0000-0000-0000-000000000004', 'confidence': 0.95}]},
        'POST /leads/{id}/merge': {'success': True, 'kept_id': '00000000-0000-0000-0000-000000000001', 'merged_id': '00000000-0000-0000-0000-000000000004'},
    },
    'platform': {
        'POST /webhooks/{id}/test': {'success': True, 'webhook_id': '00000000-0000-0000-0000-000000000016', 'status_code': 200, 'response_time_ms': 45},
        'GET /audit-log/{entity}/{entity_id}': {'entity': 'lead', 'entity_id': '00000000-0000-0000-0000-000000000003', 'log_entries': []},
        'POST /auth/validate': {'valid': True, 'user_id': 'u001', 'permissions': ['crm:read', 'crm:write']},
        'POST /integrations/{id}/sync': {'success': True, 'integration_id': '00000000-0000-0000-0000-000000000018', 'records_synced': 25},
    },
    'reporting': {
        'POST /reports/build': {'report_id': '00000000-0000-0000-0000-000000000019', 'data': {'total_opportunities': 50, 'total_revenue': 250000}, 'saved': True},
        'POST /reports/export': {'success': True, 'format': 'csv', 'file_url': '/reports/export/00000000-0000-0000-0000-000000000019.csv'},
    },
}


def generate_example_from_inline_schema(schema_obj):
    """Generate an example from an inline schema's properties."""
    properties = schema_obj.get('properties', {})
    if not properties:
        return {}
    
    example = {}
    for prop_name, prop_def in properties.items():
        prop_type = prop_def.get('type', 'string')
        prop_format = prop_def.get('format', '')
        
        if prop_type == 'string':
            if prop_format == 'uuid':
                example[prop_name] = '00000000-0000-0000-0000-000000000001'
            elif prop_name == 'email':
                example[prop_name] = 'user@example.com'
            elif prop_name == 'name':
                example[prop_name] = 'Example Name'
            else:
                example[prop_name] = f'example_{prop_name}'
        elif prop_type == 'integer':
            if 'count' in prop_name or 'total' in prop_name or 'num' in prop_name:
                example[prop_name] = 1
            elif 'time' in prop_name:
                example[prop_name] = 100
            else:
                example[prop_name] = 1
        elif prop_type == 'number':
            example[prop_name] = 1.0
        elif prop_type == 'boolean':
            example[prop_name] = True
        elif prop_type == 'object':
            example[prop_name] = generate_example_from_inline_schema(prop_def)
        elif prop_type == 'array':
            items = prop_def.get('items', {})
            if items.get('type') == 'object':
                example[prop_name] = [generate_example_from_inline_schema(items)]
            else:
                example[prop_name] = []
    
    return example


def process_spec(spec_path):
    """Add examples to all responses missing them (inline schema case)."""
    with open(spec_path) as f:
        data = yaml.safe_load(f)
    
    modified = 0
    service = spec_path.split('/')[-2]
    specific_examples = SPECIFIC_EXAMPLES.get(service, {})
    
    for path, methods in data['paths'].items():
        for method, op in methods.items():
            if method not in ['get', 'post', 'put', 'patch', 'delete']:
                continue
            
            responses = op.get('responses', {})
            for sc in ['200', '201']:
                if sc not in responses:
                    continue
                
                content = responses[sc].get('content', {})
                json_schema = content.get('application/json', {})
                
                # Skip if already has examples
                if json_schema.get('examples'):
                    continue
                
                # Check for specific example first
                key = f"{method.upper()} {path}"
                if key in specific_examples:
                    json_schema['examples'] = {'default': specific_examples[key]}
                    modified += 1
                    continue
                
                # Try $ref first
                schema_obj = json_schema.get('schema', {})
                ref = schema_obj.get('$ref', '')
                if ref:
                    schema_name = ref.split('/')[-1]
                    if schema_name.startswith('Paginated'):
                        # Handle paginated
                        base_name = schema_name.replace('Paginated', '')
                        template_map = {
                            'Account': 'Account', 'Contact': 'Contact', 'Lead': 'Lead',
                            'Stage': 'Stage', 'Team': 'Team', 'Workflow': 'Workflow',
                            'Rule': 'Rule', 'Trigger': 'Trigger', 'ChatSession': 'ChatSession',
                            'ChatMessage': 'ChatMessage', 'Agent': 'Agent', 'Campaign': 'Campaign',
                            'Subscription': 'Subscription', 'Goal': 'Goal', 'CustomField': 'CustomField',
                            'Webhook': 'Webhook', 'ApiKey': 'ApiKey', 'Integration': 'Integration',
                            'Report': 'Report', 'UTMCampaign': 'UTMCampaign', 'UTMMedium': 'UTMMedium',
                            'UTMSource': 'UTMSource', 'EventRegistration': 'EventRegistration',
                            'VisitorTracking': 'VisitorTracking', 'CampaignLeadResult': 'CampaignLeadResult',
                            'PipelineSummary': 'PipelineSummary', 'LeadScoreSummary': 'LeadScoreSummary',
                            'LeadScoreRanked': 'LeadScoreRanked', 'ConversionRate': 'ConversionRate',
                            'ForecastEntry': 'ForecastEntry', 'LeadSourceReport': 'LeadSourceReport',
                            'LeaderboardEntry': 'LeaderboardEntry', 'ForecastAccuracyEntry': 'ForecastAccuracyEntry',
                            'AgentStatus': 'AgentStatus', 'AgentQueueItem': 'AgentQueueItem',
                            'UnassignedLead': 'UnassignedLead', 'ScheduledAction': 'ScheduledAction',
                            'UserBadge': 'UserBadge', 'AccountContactSummary': 'AccountContactSummary',
                        }
                        example_type = template_map.get(base_name)
                        if example_type:
                            response_examples = {
                                'Account': {'id': '00000000-0000-0000-0000-000000000001', 'name': 'Acme Corp'},
                                'Contact': {'id': '00000000-0000-0000-0000-000000000002', 'first_name': 'John', 'last_name': 'Doe'},
                                'Lead': {'id': '00000000-0000-0000-0000-000000000003', 'name': 'Jane Smith', 'email': 'jane@example.com'},
                                'Stage': {'id': '00000000-0000-0000-0000-000000000004', 'name': 'Qualification'},
                                'Team': {'id': '00000000-0000-0000-0000-000000000005', 'name': 'Sales Team Alpha'},
                                'Workflow': {'id': '00000000-0000-0000-0000-000000000006', 'name': 'Lead Assignment'},
                                'Rule': {'id': '00000000-0000-0000-0000-000000000007', 'name': 'Assign to region A'},
                                'Trigger': {'id': '00000000-0000-0000-0000-000000000008', 'name': 'On Lead Create'},
                                'ChatSession': {'id': '00000000-0000-0000-0000-000000000009', 'status': 'active'},
                                'ChatMessage': {'id': '00000000-0000-0000-0000-000000000010', 'message': 'Hello!'},
                                'Agent': {'id': '00000000-0000-0000-0000-000000000011', 'name': 'Agent Smith'},
                                'Campaign': {'id': '00000000-0000-0000-0000-000000000012', 'name': 'Q1 Campaign'},
                                'Subscription': {'id': '00000000-0000-0000-0000-000000000013', 'status': 'active'},
                                'Goal': {'id': '00000000-0000-0000-0000-000000000014', 'name': 'Close 10 deals'},
                                'CustomField': {'id': '00000000-0000-0000-0000-000000000015', 'name': 'priority'},
                                'Webhook': {'id': '00000000-0000-0000-0000-000000000016', 'url': 'https://example.com/hook'},
                                'ApiKey': {'id': '00000000-0000-0000-0000-000000000017', 'name': 'Production API'},
                                'Integration': {'id': '00000000-0000-0000-0000-000000000018', 'name': 'Slack Integration'},
                                'Report': {'id': '00000000-0000-0000-0000-000000000019', 'name': 'Pipeline Report'},
                                'UTMCampaign': {'id': '00000000-0000-0000-0000-000000000020', 'name': 'Q1 Email Blast'},
                                'UTMMedium': {'id': '00000000-0000-0000-0000-000000000021', 'name': 'email'},
                                'UTMSource': {'id': '00000000-0000-0000-0000-000000000022', 'name': 'newsletter'},
                                'EventRegistration': {'id': '00000000-0000-0000-0000-000000000028', 'status': 'confirmed'},
                                'VisitorTracking': {'id': '00000000-0000-0000-0000-000000000031', 'url_visited': '/pricing'},
                                'CampaignLeadResult': {'lead_id': '00000000-0000-0000-0000-000000000003', 'campaign_id': '00000000-0000-0000-0000-000000000012'},
                                'PipelineSummary': {'total_opportunities': 50, 'total_revenue': 250000},
                                'LeadScoreSummary': {'hot': 10, 'warm': 25, 'cold': 50},
                                'LeadScoreRanked': {'id': '00000000-0000-0000-0000-000000000003', 'score': 95},
                                'ConversionRate': {'overall_rate': 15.5},
                                'ForecastEntry': {'period': '2024-Q2', 'predicted_revenue': 500000},
                                'LeadSourceReport': {'total_leads': 500},
                                'LeaderboardEntry': {'rank': 1, 'rep_name': 'Jane Doe', 'deals_closed': 10},
                                'ForecastAccuracyEntry': {'period': '2024-Q1', 'forecasted': 450000, 'actual': 500000},
                                'AgentStatus': {'agent_id': 'a001', 'is_online': True},
                                'AgentQueueItem': {'id': 'v123', 'waiting_time': 120},
                                'UnassignedLead': {'lead_id': '00000000-0000-0000-0000-000000000003', 'name': 'John Doe'},
                                'ScheduledAction': {'id': '00000000-0000-0000-0000-000000000025', 'status': 'pending'},
                                'UserBadge': {'id': '00000000-0000-0000-0000-000000000024', 'badge_id': '00000000-0000-0000-0000-000000000023'},
                                'AccountContactSummary': {'account_id': '00000000-0000-0000-0000-000000000001'},
                            }
                            item_example = template_map.get(base_name)
                            if item_example and item_example in response_examples:
                                json_schema['examples'] = {'default': {'items': [response_examples[item_example]], 'total': 1, 'page': 1, 'per_page': 20}}
                                modified += 1
                    else:
                        # Known non-paginated schemas
                        known_schemas = {
                            'Account': {'id': '00000000-0000-0000-0000-000000000001', 'name': 'Acme Corp'},
                            'Contact': {'id': '00000000-0000-0000-0000-000000000002', 'first_name': 'John', 'last_name': 'Doe'},
                            'Lead': {'id': '00000000-0000-0000-0000-000000000003', 'name': 'Jane Smith', 'email': 'jane@example.com'},
                            'Stage': {'id': '00000000-0000-0000-0000-000000000004', 'name': 'Qualification'},
                            'Team': {'id': '00000000-0000-0000-0000-000000000005', 'name': 'Sales Team Alpha'},
                            'Workflow': {'id': '00000000-0000-0000-0000-000000000006', 'name': 'Lead Assignment'},
                            'Rule': {'id': '00000000-0000-0000-0000-000000000007', 'name': 'Assign to region A'},
                            'Trigger': {'id': '00000000-0000-0000-0000-000000000008', 'name': 'On Lead Create'},
                            'ChatSession': {'id': '00000000-0000-0000-0000-000000000009', 'status': 'active'},
                            'ChatMessage': {'id': '00000000-0000-0000-0000-000000000010', 'message': 'Hello!'},
                            'Agent': {'id': '00000000-0000-0000-0000-000000000011', 'name': 'Agent Smith'},
                            'ConversionResult': {'success': True, 'contact_id': '00000000-0000-0000-0000-000000000002'},
                            'Summary': {'total': 150, 'active': 120},
                            'Analytics': {'total_visitors': 1000, 'page_views': 5000},
                            'PipelineSummary': {'total_opportunities': 50, 'total_revenue': 250000},
                        }
                        if schema_name in known_schemas:
                            json_schema['examples'] = {'default': known_schemas[schema_name]}
                            modified += 1
                
                elif 'properties' in schema_obj:
                    # Inline schema - generate from properties
                    example = generate_example_from_inline_schema(schema_obj)
                    if example:
                        json_schema['examples'] = {'default': example}
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
    
    fixed = process_spec(spec_path)
    if fixed:
        print(f"  {item}: {fixed} response examples added")
        total_fixed += fixed

print(f"\nTotal: {total_fixed} response examples added")
