"""
Item 4: Add request examples for all POST/PUT/PATCH operations.

Generates realistic request bodies based on the schema of each write operation's
requestBody. Focuses on the most common entity types across the CRM suite.
"""
import yaml
import os

crm_base = '/home/casibbald/Workspace/microscaler/rerp/openapi/crm/'

# Request body templates for each entity type
REQUEST_TEMPLATES = {
    'Lead': {
        'name': 'John Doe',
        'email': 'john@example.com',
        'phone': '+1-555-0123',
        'company': 'Acme Corp',
        'source': 'website',
    },
    'Contact': {
        'first_name': 'Jane',
        'last_name': 'Smith',
        'email': 'jane@example.com',
        'phone': '+1-555-0124',
        'title': 'Marketing Director',
        'account_id': '00000000-0000-0000-0000-000000000003',
    },
    'Account': {
        'name': 'Acme Corp',
        'website': 'https://acme.com',
        'industry': 'Technology',
        'size': 'medium',
        'owner_id': '00000000-0000-0000-0000-000000000010',
    },
    'Stage': {
        'name': 'Discovery',
        'order': 1,
        'pipeline_id': '00000000-0000-0000-0000-000000000005',
    },
    'Team': {
        'name': 'Sales Team Alpha',
        'description': 'Enterprise sales team',
    },
    'Workflow': {
        'name': 'Lead Assignment',
        'description': 'Auto-assign leads based on region',
        'enabled': True,
    },
    'Rule': {
        'name': 'Assign to region A',
        'condition': {'field': 'region', 'operator': 'equals', 'value': 'A'},
        'action': {'type': 'assign', 'value': 'team-001'},
    },
    'Trigger': {
        'name': 'On Lead Create',
        'event': 'lead.created',
        'enabled': True,
    },
    'Subscription': {
        'customer_id': '00000000-0000-0000-0000-000000000019',
        'plan': 'professional',
    },
    'Goal': {
        'name': 'Close 10 deals',
        'target': 10,
        'period': 'quarterly',
    },
    'CustomField': {
        'entity': 'lead',
        'name': 'priority',
        'type': 'string',
        'required': False,
    },
    'Webhook': {
        'url': 'https://example.com/webhook',
        'events': ['lead.created', 'lead.updated'],
        'enabled': True,
    },
    'ApiKey': {
        'name': 'Production API',
        'enabled': True,
    },
    'Integration': {
        'name': 'Slack Integration',
        'type': 'messaging',
    },
    'ChatMessage': {
        'content': 'Hello, how can I help you?',
    },
    'Agent': {
        'name': 'Agent Smith',
        'email': 'agent@example.com',
    },
    'Campaign': {
        'name': 'Q1 Marketing Campaign',
        'type': 'email',
    },
    'UTMCampaign': {
        'name': 'Q1 Email Blast',
        'type': 'email',
    },
    'UTMMedium': {
        'name': 'email',
        'description': 'Email marketing medium',
    },
    'UTMSource': {
        'name': 'newsletter',
        'description': 'Newsletter traffic source',
    },
    'Report': {
        'name': 'Monthly Pipeline Report',
        'type': 'pipeline',
        'schedule': 'monthly',
    },
    'EventRegistration': {
        'event_id': '00000000-0000-0000-0000-000000000029',
        'contact_id': '00000000-0000-0000-0000-000000000002',
    },
}


def find_request_schema(operation):
    """Extract the request schema name from an operation's requestBody."""
    request_body = operation.get('requestBody', {})
    content = request_body.get('content', {})
    json_schema = content.get('application/json', {}).get('schema', {})
    
    if '$ref' in json_schema:
        return json_schema['$ref'].split('/')[-1]
    
    return None


def add_request_example(operation, request_schema_name):
    """Add a request example based on schema name."""
    if not request_schema_name:
        return False
    
    # Direct match
    if request_schema_name in REQUEST_TEMPLATES:
        operation['requestBody']['content']['application/json']['examples'] = {
            'default': REQUEST_TEMPLATES[request_schema_name]
        }
        return True
    
    # Try without 'Request' suffix
    if request_schema_name.endswith('Request'):
        base_name = request_schema_name[:-7]
        if base_name in REQUEST_TEMPLATES:
            operation['requestBody']['content']['application/json']['examples'] = {
                'default': REQUEST_TEMPLATES[base_name]
            }
            return True
    
    # Try partial match
    for template_name, template in REQUEST_TEMPLATES.items():
        if template_name.lower() in request_schema_name.lower():
            operation['requestBody']['content']['application/json']['examples'] = {
                'default': template
            }
            return True
    
    return False


def process_spec(spec_path):
    """Process a single OpenAPI spec file."""
    with open(spec_path) as f:
        data = yaml.safe_load(f)
    
    modified = 0
    for path, methods in data['paths'].items():
        for method, operation in methods.items():
            if method not in ['post', 'put', 'patch']:
                continue
            
            # Check if operation has a requestBody
            if not operation.get('requestBody'):
                continue
            
            # Skip if already has examples
            if operation['requestBody'].get('content', {}).get('application/json', {}).get('examples'):
                continue
            
            # Get the request schema name
            request_schema = find_request_schema(operation)
            
            # Add example
            if add_request_example(operation, request_schema):
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
        print(f"  {item}: {fixed} request examples added")
        total_fixed += fixed

print(f"\nTotal: {total_fixed} operations now have request examples")
