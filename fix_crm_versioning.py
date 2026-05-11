"""
Item 6: Add versioning to all 11 CRM specs.

Adds OpenAPI versioning information:
- info.version: API version number (v1.0.0)
- info.x-api-version: Human-readable API version
- x-crm-api-version: BRRTRouter-specific version marker

Also adds info.description and info.termsOfService if missing.
"""
import yaml
import os

crm_base = '/home/casibbald/Workspace/microscaler/rerp/openapi/crm/'

# Versioning info for each service
SERVICE_VERSIONS = {
    'accounts': {'api_version': 'v1.0.0', 'human_version': 'CRM Accounts API v1.0.0'},
    'automation': {'api_version': 'v1.0.0', 'human_version': 'CRM Automation API v1.0.0'},
    'contacts': {'api_version': 'v1.0.0', 'human_version': 'CRM Contacts API v1.0.0'},
    'engagement': {'api_version': 'v1.0.0', 'human_version': 'CRM Engagement API v1.0.0'},
    'intelligence': {'api_version': 'v1.0.0', 'human_version': 'CRM Intelligence API v1.0.0'},
    'livechat': {'api_version': 'v1.0.0', 'human_version': 'CRM LiveChat API v1.0.0'},
    'marketing': {'api_version': 'v1.0.0', 'human_version': 'CRM Marketing API v1.0.0'},
    'pipeline': {'api_version': 'v1.0.0', 'human_version': 'CRM Pipeline API v1.0.0'},
    'platform': {'api_version': 'v1.0.0', 'human_version': 'CRM Platform API v1.0.0'},
    'reporting': {'api_version': 'v1.0.0', 'human_version': 'CRM Reporting API v1.0.0'},
    'teams': {'api_version': 'v1.0.0', 'human_version': 'CRM Teams API v1.0.0'},
}


def add_versioning(service, spec_path):
    """Add versioning to an OpenAPI spec."""
    with open(spec_path) as f:
        data = yaml.safe_load(f)
    
    version_info = SERVICE_VERSIONS.get(service, SERVICE_VERSIONS['platform'])
    modified = 0
    
    # Ensure info section exists
    if 'info' not in data:
        data['info'] = {}
        modified += 1
    
    # Add version
    if data['info'].get('version') != version_info['api_version']:
        data['info']['version'] = version_info['api_version']
        modified += 1
    
    # Add human-readable version
    if 'x-api-version' not in data['info']:
        data['info']['x-api-version'] = version_info['human_version']
        modified += 1
    
    # Add x-crm-api-version for BRRTRouter
    if 'x-crm-api-version' not in data['info']:
        data['info']['x-crm-api-version'] = version_info['api_version']
        modified += 1
    
    # Add description if missing
    if not data['info'].get('description'):
        data['info']['description'] = f'{version_info["human_version"]} — OpenAPI specification for the {service} microservice in the RERP CRM suite'
        modified += 1
    
    # Add terms of service if missing
    if not data['info'].get('termsOfService'):
        data['info']['termsOfService'] = 'https://rerp.example.com/terms'
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
    
    fixed = add_versioning(service, spec_path)
    if fixed:
        print(f"  {service}: {fixed} versioning fields added")
        total_fixed += fixed

print(f"\nTotal: {total_fixed} versioning fields added")
