"""
Fix ALL remaining response code gaps across 11 CRM specs.

Based on fresh audit:
- 201: 129 missing from POST ops (that create resources, not action triggers)
- 204: 154 missing from PUT/PATCH ops (that update resources)
- 400: 111 missing from write ops
- 409: 136 missing from conflict-prone ops

Total: 530 missing response codes
"""
import yaml
import os

crm_base = '/home/casibbald/Workspace/microscaler/rerp/openapi/crm/'

# Action triggers that execute, don't create/update - keep 200
TRUE_ACTIONS = {
    'POST /workflows/{id}/run',
    'POST /triggers/{id}/fire',
    'POST /rules/test',
    'POST /scoring/frequencies/rebuild',
    'POST /assign/run',
    'POST /webhooks/{id}/test',
    'POST /chats/{id}/close',  # closes a chat session
}

def should_have_201(method, path):
    """POST ops that create resources should return 201."""
    if method != 'post':
        return False
    if f"{method} {path}" in TRUE_ACTIONS:
        return False
    return True

def should_have_204(method, path):
    """PUT/PATCH ops that update resources should return 204."""
    if method not in ('put', 'patch'):
        return False
    return True

def should_have_400(method, path):
    """Write operations that accept request bodies should return 400."""
    if method in ('post', 'put', 'patch'):
        return True
    return False

def should_have_409(method, path):
    """Conflict-prone operations should return 409."""
    conflict_patterns = ['/merge', '/duplicate', '/sync', '/renew', '/convert', '/enrich', '/validate']
    for pattern in conflict_patterns:
        if pattern in path.lower():
            return True
    return False

def process_spec(spec_path):
    with open(spec_path) as f:
        data = yaml.safe_load(f)
    
    fixed = 0
    for path, methods in data['paths'].items():
        for method, op in methods.items():
            if method not in ('get', 'post', 'put', 'patch', 'delete'):
                continue
            
            responses = op.setdefault('responses', {})
            
            # 201 for POST that creates
            if should_have_201(method, path) and '201' not in responses:
                responses['201'] = {
                    'description': 'Resource created successfully',
                    'content': {'application/json': {'schema': {'type': 'object'}}}
                }
                fixed += 1
            
            # 204 for PUT/PATCH that updates
            if should_have_204(method, path) and '204' not in responses:
                responses['204'] = {
                    'description': 'Resource updated successfully',
                }
                fixed += 1
            
            # 400 for write operations
            if should_have_400(method, path) and '400' not in responses:
                responses['400'] = {
                    'description': 'Bad request - invalid input parameters',
                }
                fixed += 1
            
            # 409 for conflict-prone operations
            if should_have_409(method, path) and '409' not in responses:
                responses['409'] = {
                    'description': 'Conflict - resource conflict or operation cannot be completed',
                }
                fixed += 1
    
    if fixed:
        with open(spec_path, 'w') as f:
            yaml.dump(data, f, default_flow_style=False, sort_keys=False, allow_unicode=True)
    
    return fixed

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
        print(f"  {item}: {fixed} response codes added")
        total_fixed += fixed

print(f"\nTotal: {total_fixed} response codes added")
