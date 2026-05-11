"""
Fix 2: Add 204 NO CONTENT response to all PUT/PATCH operations.

Fixes 18 operations across 10 CRM services.
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
            if method in ['put', 'patch'] and '204' not in operation.get('responses', {}):
                responses = operation.setdefault('responses', {})
                responses['204'] = {
                    'description': 'Resource updated successfully'
                }
                modified = True
                fixed += 1
                print(f"  Fixed {item}: {method.upper()} {path}")
    
    if modified:
        with open(spec_path, 'w') as f:
            yaml.dump(data, f, default_flow_style=False, sort_keys=False, allow_unicode=True)

print(f"\nTotal fixed: {fixed} operations across 10 services")
