"""
Fix 1: Add 201 CREATED response to all POST operations that create resources.

This fixes 29 operations across 10 CRM services.
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
            if method == 'post' and '201' not in operation.get('responses', {}):
                # Check if this is a resource creation endpoint (not an action trigger)
                # Exclude: /run, /fire, /close, /rebuild, /test, /sync, /identify, /track, /register, /submit
                action_endpoints = ['/run', '/fire', '/close', '/rebuild', '/test', 
                                  '/sync', '/identify', '/track', '/register', '/submit',
                                  '/assign/run', '/validate']
                is_action = any(path.endswith(ae) for ae in action_endpoints)
                
                if not is_action:
                    responses = operation.setdefault('responses', {})
                    responses['201'] = {
                        'description': 'Resource created successfully',
                        'content': {
                            'application/json': {
                                'schema': {
                                    'type': 'object',
                                    'properties': {
                                        'id': {'type': 'string', 'format': 'uuid'}
                                    }
                                }
                            }
                        }
                    }
                    modified = True
                    fixed += 1
                    print(f"  Fixed {item}: POST {path}")
    
    if modified:
        with open(spec_path, 'w') as f:
            yaml.dump(data, f, default_flow_style=False, sort_keys=False, allow_unicode=True)

print(f"\nTotal fixed: {fixed} operations across 10 services")
