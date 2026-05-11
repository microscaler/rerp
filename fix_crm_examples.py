"""
Item 1: Add example values to 192 operations across 11 CRM specs.

Generates realistic examples based on entity schemas for each operation's
response type. Focuses on 200 and 201 responses since those are the
most valuable for API documentation.
"""
import yaml
import os
import re
from collections import defaultdict

crm_base = '/home/casibbald/Workspace/microscaler/rerp/openapi/crm/'

# Example templates for common entity types
EXAMPLE_TEMPLATES = {
    'Lead': {
        'id': '00000000-0000-0000-0000-000000000001',
        'name': 'John Doe',
        'email': 'john@example.com',
        'phone': '+1-555-0123',
        'company': 'Acme Corp',
        'status': 'new',
        'stage': 'discovery',
        'score': 45,
        'source': 'website',
        'assigned_to': '00000000-0000-0000-0000-000000000010',
        'created_at': '2024-01-15T10:30:00Z',
        'updated_at': '2024-01-15T10:30:00Z',
        'properties': {},
    },
    'Contact': {
        'id': '00000000-0000-0000-0000-000000000002',
        'first_name': 'Jane',
        'last_name': 'Smith',
        'email': 'jane@example.com',
        'phone': '+1-555-0124',
        'title': 'Marketing Director',
        'account_id': '00000000-0000-0000-0000-000000000003',
        'created_at': '2024-01-15T10:30:00Z',
        'updated_at': '2024-01-15T10:30:00Z',
    },
    'Account': {
        'id': '00000000-0000-0000-0000-000000000003',
        'name': 'Acme Corp',
        'website': 'https://acme.com',
        'industry': 'Technology',
        'size': 'medium',
        'owner_id': '00000000-0000-0000-0000-000000000010',
        'created_at': '2024-01-15T10:30:00Z',
        'updated_at': '2024-01-15T10:30:00Z',
    },
    'Stage': {
        'id': '00000000-0000-0000-0000-000000000004',
        'name': 'Discovery',
        'order': 1,
        'pipeline_id': '00000000-0000-0000-0000-000000000005',
        'created_at': '2024-01-15T10:30:00Z',
        'updated_at': '2024-01-15T10:30:00Z',
    },
    'Team': {
        'id': '00000000-0000-0000-0000-000000000006',
        'name': 'Sales Team Alpha',
        'description': 'Enterprise sales team',
        'members': [],
        'created_at': '2024-01-15T10:30:00Z',
        'updated_at': '2024-01-15T10:30:00Z',
    },
    'Workflow': {
        'id': '00000000-0000-0000-0000-000000000007',
        'name': 'Lead Assignment',
        'description': 'Auto-assign leads based on region',
        'enabled': True,
        'created_at': '2024-01-15T10:30:00Z',
        'updated_at': '2024-01-15T10:30:00Z',
    },
    'Rule': {
        'id': '00000000-0000-0000-0000-000000000008',
        'name': 'Assign to region A',
        'condition': {'field': 'region', 'operator': 'equals', 'value': 'A'},
        'action': {'type': 'assign', 'value': 'team-001'},
        'created_at': '2024-01-15T10:30:00Z',
        'updated_at': '2024-01-15T10:30:00Z',
    },
    'Trigger': {
        'id': '00000000-0000-0000-0000-000000000009',
        'name': 'On Lead Create',
        'event': 'lead.created',
        'enabled': True,
        'created_at': '2024-01-15T10:30:00Z',
        'updated_at': '2024-01-15T10:30:00Z',
    },
    'ScoringFrequency': {
        'id': '00000000-0000-0000-0000-000000000011',
        'name': 'Daily',
        'interval': 'daily',
        'enabled': True,
        'created_at': '2024-01-15T10:30:00Z',
        'updated_at': '2024-01-15T10:30:00Z',
    },
    'EnrichmentResult': {
        'id': '00000000-0000-0000-0000-000000000012',
        'lead_id': '00000000-0000-0000-0000-000000000001',
        'company': 'Acme Corp',
        'industry': 'Technology',
        'employees': 500,
        'founded': 2010,
    },
    'ChatSession': {
        'id': '00000000-0000-0000-0000-000000000013',
        'visitor_id': '00000000-0000-0000-0000-000000000014',
        'agent_id': '00000000-0000-0000-0000-000000000015',
        'status': 'active',
        'started_at': '2024-01-15T10:30:00Z',
        'updated_at': '2024-01-15T10:30:00Z',
    },
    'ChatMessage': {
        'id': '00000000-0000-0000-0000-000000000016',
        'session_id': '00000000-0000-0000-0000-000000000013',
        'sender_id': '00000000-0000-0000-0000-000000000015',
        'content': 'Hello, how can I help you?',
        'created_at': '2024-01-15T10:30:00Z',
    },
    'Agent': {
        'id': '00000000-0000-0000-0000-000000000015',
        'name': 'Agent Smith',
        'email': 'agent@example.com',
        'status': 'available',
        'created_at': '2024-01-15T10:30:00Z',
        'updated_at': '2024-01-15T10:30:00Z',
    },
    'Campaign': {
        'id': '00000000-0000-0000-0000-000000000017',
        'name': 'Q1 Marketing Campaign',
        'type': 'email',
        'status': 'active',
        'created_at': '2024-01-15T10:30:00Z',
        'updated_at': '2024-01-15T10:30:00Z',
    },
    'Subscription': {
        'id': '00000000-0000-0000-0000-000000000018',
        'customer_id': '00000000-0000-0000-0000-000000000019',
        'plan': 'professional',
        'status': 'active',
        'mrr': 99.00,
        'created_at': '2024-01-15T10:30:00Z',
        'updated_at': '2024-01-15T10:30:00Z',
    },
    'Goal': {
        'id': '00000000-0000-0000-0000-000000000020',
        'name': 'Close 10 deals',
        'target': 10,
        'current': 7,
        'period': 'quarterly',
        'created_at': '2024-01-15T10:30:00Z',
        'updated_at': '2024-01-15T10:30:00Z',
    },
    'CustomField': {
        'id': '00000000-0000-0000-0000-000000000021',
        'entity': 'lead',
        'name': 'priority',
        'type': 'string',
        'required': False,
        'created_at': '2024-01-15T10:30:00Z',
        'updated_at': '2024-01-15T10:30:00Z',
    },
    'Webhook': {
        'id': '00000000-0000-0000-0000-000000000022',
        'url': 'https://example.com/webhook',
        'events': ['lead.created', 'lead.updated'],
        'enabled': True,
        'created_at': '2024-01-15T10:30:00Z',
        'updated_at': '2024-01-15T10:30:00Z',
    },
    'ApiKey': {
        'id': '00000000-0000-0000-0000-000000000023',
        'name': 'Production API',
        'key': 'sk_test_00000000000000000000',
        'enabled': True,
        'created_at': '2024-01-15T10:30:00Z',
        'updated_at': '2024-01-15T10:30:00Z',
    },
    'Integration': {
        'id': '00000000-0000-0000-0000-000000000024',
        'name': 'Slack Integration',
        'type': 'messaging',
        'status': 'connected',
        'created_at': '2024-01-15T10:30:00Z',
        'updated_at': '2024-01-15T10:30:00Z',
    },
    'Report': {
        'id': '00000000-0000-0000-0000-000000000025',
        'name': 'Monthly Pipeline Report',
        'type': 'pipeline',
        'schedule': 'monthly',
        'created_at': '2024-01-15T10:30:00Z',
        'updated_at': '2024-01-15T10:30:00Z',
    },
    'Summary': {
        'total_leads': 150,
        'active_leads': 85,
        'conversion_rate': 12.5,
        'pipeline_value': 250000,
        'period': 'monthly',
    },
    'Analytics': {
        'total_visitors': 1250,
        'unique_visitors': 890,
        'bounce_rate': 35.2,
        'avg_session_duration': 245,
        'period': 'daily',
    },
    'PipelineSummary': {
        'total_leads': 150,
        'stage_breakdown': {
            'discovery': 45,
            'qualified': 35,
            'proposal': 25,
            'negotiation': 30,
        },
        'weighted_value': 375000,
        'forecast': 420000,
    },
}

# Map entity names in schemas to examples
ENTITY_SCHEMA_MAP = {
    'Lead': 'Lead',
    'Contact': 'Contact',
    'Account': 'Account',
    'Stage': 'Stage',
    'Team': 'Team',
    'Workflow': 'Workflow',
    'Rule': 'Rule',
    'Trigger': 'Trigger',
    'ScoringFrequency': 'ScoringFrequency',
    'EnrichmentResult': 'EnrichmentResult',
    'ChatSession': 'ChatSession',
    'ChatMessage': 'ChatMessage',
    'Agent': 'Agent',
    'Campaign': 'Campaign',
    'Subscription': 'Subscription',
    'Goal': 'Goal',
    'CustomField': 'CustomField',
    'Webhook': 'Webhook',
    'ApiKey': 'ApiKey',
    'Integration': 'Integration',
    'Report': 'Report',
    'Summary': 'Summary',
    'Analytics': 'Analytics',
    'PipelineSummary': 'PipelineSummary',
}


def find_response_schema(operation, status_code='200'):
    """Extract the response schema name from an operation."""
    responses = operation.get('responses', {})
    if status_code in responses:
        response = responses[status_code]
        content = response.get('content', {})
        json_schema = content.get('application/json', {}).get('schema', {})
        
        if '$ref' in json_schema:
            ref = json_schema['$ref']
            # Extract schema name from ref like '#/components/schemas/Lead'
            return ref.split('/')[-1]
        
        if 'properties' in json_schema:
            # This is a direct schema definition
            # Try to match with known entities
            for entity_name, template_name in ENTITY_SCHEMA_MAP.items():
                if entity_name.lower() in json_schema.get('description', '').lower():
                    return template_name
            
            # Check properties against known templates
            props = json_schema.get('properties', {})
            for template_name, template in EXAMPLE_TEMPLATES.items():
                # Check if properties match
                if props:
                    props_match = sum(1 for k in template.keys() if k in props)
                    if props_match > len(template) * 0.3:
                        return template_name
    
    return None


def add_example_to_operation(operation, status_code='200'):
    """Add an example to an operation's response."""
    schema_name = find_response_schema(operation, status_code)
    
    if schema_name and schema_name in EXAMPLE_TEMPLATES:
        template = EXAMPLE_TEMPLATES[schema_name]
        
        responses = operation.setdefault('responses', {})
        if status_code in responses:
            response = responses[status_code]
            content = response.setdefault('content', {})
            json_schema = content.setdefault('application/json', {})
            
            json_schema['examples'] = {
                'default': template
            }
            return True
    
    return False


def add_paged_example(operation, status_code='200'):
    """Add an example for paginated responses."""
    # Check if it's a paginated response
    responses = operation.get('responses', {})
    if status_code in responses:
        response = responses[status_code]
        content = response.get('content', {})
        json_schema = content.get('application/json', {}).get('schema', {})
        
        # Look for PaginatedXxx patterns
        if '$ref' in json_schema:
            ref = json_schema['$ref']
            if 'Paginated' in ref:
                # Try to extract the item type
                schema_name = ref.split('/')[-1].replace('Paginated', '')
                if schema_name in EXAMPLE_TEMPLATES:
                    template = EXAMPLE_TEMPLATES[schema_name]
                    
                    responses[status_code]['content']['application/json']['examples'] = {
                        'default': {
                            'items': [template],
                            'total': 1,
                            'page': 1,
                            'per_page': 20,
                            'has_next': False,
                            'has_prev': False,
                        }
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
            if method not in ['get', 'post', 'put', 'patch', 'delete']:
                continue
            
            # Try to add 200 example
            if add_example_to_operation(operation, '200'):
                modified += 1
            # Try to add 201 example
            elif add_example_to_operation(operation, '201'):
                modified += 1
            
            # Try to add paginated example
            if add_paged_example(operation, '200'):
                modified += 1
            elif add_paged_example(operation, '201'):
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
        print(f"  {item}: {fixed} examples added")
        total_fixed += fixed

print(f"\nTotal: {total_fixed} operations now have examples")
