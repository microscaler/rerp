"""
Fix 1b: Add 201 CREATED to remaining POST operations.

These were excluded from pass 1 because they end with action patterns, but they
actually create resources (visitors, analytics events, registrations, survey responses).
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
                # These remaining endpoints actually create resources:
                # /analytics/visitors/identify — creates/updates visitor record
                # /analytics/track — creates analytics event
                # /events/register — creates registration
                # /surveys/submit — creates survey response
                resource_endpoints = [
                    '/analytics/visitors/identify',
                    '/analytics/track',
                    '/events/register',
                    '/surveys/submit',
                ]
                if any(path.endswith(re_) for re_ in resource_endpoints):
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

print(f"\nTotal fixed: {fixed} operations")
