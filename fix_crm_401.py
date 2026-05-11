"""
Fix Pass 4: Add 401 UNAUTHORIZED response to all 251 operations missing it.

Every operation should have 401 in responses since we require bearerAuth.
"""
import yaml
import os

crm_base = '/home/casibbald/Workspace/microscaler/rerp/openapi/crm/'
fixed = 0

for item in sorted(os.listdir(crm_base)):
    spec_path = os.path.join(crm_base, item, 'openapi.yaml')
    if not os.path.exists(spec_path):
        continue
    
    with open(spec_path) as f:
        data = yaml.safe_load(f)
    
    modified = False
    for path, methods in data['paths'].items():
        for method, operation in methods.items():
            if method in ['get', 'post', 'put', 'patch', 'delete']:
                responses = operation.setdefault('responses', {})
                if '401' not in responses:
                    responses['401'] = {
                        'description': 'Unauthorized - Invalid or missing authentication token'
                    }
                    modified = True
                    fixed += 1
    
    if modified:
        with open(spec_path, 'w') as f:
            yaml.dump(data, f, default_flow_style=False, sort_keys=False, allow_unicode=True)

print(f"Fixed: {fixed} operations across 11 services")
