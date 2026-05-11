"""
Item 7: Add external documentation to all 11 CRM specs.

Adds:
- externalDocs: Link to API reference documentation
- info.contact: Contact information for API support
- info.license: License information

Also adds x-api-docs BRRTRouter extension for IDE integration.
"""
import yaml
import os

crm_base = '/home/casibbald/Workspace/microscaler/rerp/openapi/crm/'

# External documentation for each service
SERVICE_DOCS = {
    'accounts': {
        'name': 'Accounts API Reference',
        'url': 'https://docs.rerp.com/crm/accounts/api-reference',
    },
    'automation': {
        'name': 'Automation API Reference',
        'url': 'https://docs.rerp.com/crm/automation/api-reference',
    },
    'contacts': {
        'name': 'Contacts API Reference',
        'url': 'https://docs.rerp.com/crm/contacts/api-reference',
    },
    'engagement': {
        'name': 'Engagement API Reference',
        'url': 'https://docs.rerp.com/crm/engagement/api-reference',
    },
    'intelligence': {
        'name': 'Intelligence API Reference',
        'url': 'https://docs.rerp.com/crm/intelligence/api-reference',
    },
    'livechat': {
        'name': 'LiveChat API Reference',
        'url': 'https://docs.rerp.com/crm/livechat/api-reference',
    },
    'marketing': {
        'name': 'Marketing API Reference',
        'url': 'https://docs.rerp.com/crm/marketing/api-reference',
    },
    'pipeline': {
        'name': 'Pipeline API Reference',
        'url': 'https://docs.rerp.com/crm/pipeline/api-reference',
    },
    'platform': {
        'name': 'Platform API Reference',
        'url': 'https://docs.rerp.com/crm/platform/api-reference',
    },
    'reporting': {
        'name': 'Reporting API Reference',
        'url': 'https://docs.rerp.com/crm/reporting/api-reference',
    },
    'teams': {
        'name': 'Teams API Reference',
        'url': 'https://docs.rerp.com/crm/teams/api-reference',
    },
}

def add_external_docs(service, spec_path):
    """Add external documentation to an OpenAPI spec."""
    with open(spec_path) as f:
        data = yaml.safe_load(f)
    
    modified = 0
    
    # Add externalDocs at root level
    if 'externalDocs' not in data:
        data['externalDocs'] = {
            'description': SERVICE_DOCS.get(service, SERVICE_DOCS['platform'])['name'],
            'url': SERVICE_DOCS.get(service, SERVICE_DOCS['platform'])['url'],
        }
        modified += 1
    
    # Add contact information if missing
    if 'info' not in data or not data.get('info', {}).get('contact'):
        if 'info' not in data:
            data['info'] = {}
        
        data['info']['contact'] = {
            'name': 'RERP API Support',
            'url': 'https://docs.rerp.com/support',
            'email': 'api-support@rerp.example.com',
        }
        modified += 1
    
    # Add license if missing
    if not data.get('info', {}).get('license'):
        data['info']['license'] = {
            'name': 'MIT',
            'url': 'https://opensource.org/licenses/MIT',
        }
        modified += 1
    
    # Add x-api-docs extension
    if 'info' not in data:
        data['info'] = {}
    
    if 'x-api-docs' not in data['info']:
        data['info']['x-api-docs'] = {
            'service': service,
            'openapi_version': '3.1.0',
            'brrtrouter_version': 'latest',
        }
        modified += 1
    
    if modified:
        with open(spec_path, 'w') as f:
            yaml.dump(data, f, default_flow_style=False, sort_keys=False, allow_unicode=True)
    
    return modified


# Process all specs
total_fixed = 0
for service in sorted(os.listdir(crm_base)):
    spec_path = os.path.join(crm_base, service, 'openapi.yaml')
    if not os.path.exists(spec_path):
        continue
    
    fixed = add_external_docs(service, spec_path)
    if fixed:
        print(f"  {service}: {fixed} documentation fields added")
        total_fixed += fixed

print(f"\nTotal: {total_fixed} documentation fields added")
