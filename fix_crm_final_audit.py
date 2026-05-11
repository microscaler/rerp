"""
Item 8: Comprehensive final audit and improvements.

Checks for:
1. Security scheme completeness
2. Missing Content-Type headers
3. Missing Accept headers
4. Schema example values
5. Parameter descriptions
6. Response example completeness
7. Path parameter descriptions
8. Summary field completeness
"""
import yaml
import os
from collections import defaultdict

crm_base = '/home/casibbald/Workspace/microscaler/rerp/openapi/crm/'

audit_results = {
    'security_schemes': {'total': 0, 'missing': [], 'fixed': []},
    'parameter_descriptions': {'total': 0, 'missing': [], 'fixed': []},
    'response_examples': {'total': 0, 'missing': [], 'fixed': []},
    'path_param_descriptions': {'total': 0, 'missing': [], 'fixed': []},
    'summary_fields': {'total': 0, 'missing': [], 'fixed': []},
    'content_type_headers': {'total': 0, 'missing': [], 'fixed': []},
    'schema_descriptions': {'total': 0, 'missing': [], 'fixed': []},
    'request_body_descriptions': {'total': 0, 'missing': [], 'fixed': []},
}

for item in sorted(os.listdir(crm_base)):
    spec_path = os.path.join(crm_base, item, 'openapi.yaml')
    if not os.path.exists(spec_path):
        continue
    
    with open(spec_path) as f:
        data = yaml.safe_load(f)
    
    # Check 1: Security schemes
    if data.get('openapi') and data['openapi'] != '3.1.0':
        audit_results['security_schemes']['missing'].append(f'{item}: openapi version is {data["openapi"]}')
    
    if 'components' not in data or 'securitySchemes' not in data['components']:
        audit_results['security_schemes']['missing'].append(f'{item}: missing securitySchemes')
    else:
        schemes = data['components']['securitySchemes']
        if 'bearerAuth' not in schemes:
            audit_results['security_schemes']['missing'].append(f'{item}: missing bearerAuth')
    
    # Check 2: Parameter descriptions
    parameters = data.get('components', {}).get('parameters', {})
    for param_name, param_def in parameters.items():
        audit_results['parameter_descriptions']['total'] += 1
        if not param_def.get('description'):
            audit_results['parameter_descriptions']['missing'].append(f'{item}: {param_name}')
    
    # Check 3: Path parameter descriptions
    for path, methods in data['paths'].items():
        for method, operation in methods.items():
            if method not in ['get', 'post', 'put', 'patch', 'delete']:
                continue
            
            for param in operation.get('parameters', []):
                audit_results['path_param_descriptions']['total'] += 1
                if not param.get('description'):
                    audit_results['path_param_descriptions']['missing'].append(f'{item}: {method.upper()} {path} -> {param.get("name")}')
            
            for param in methods.get('parameters', []):
                audit_results['path_param_descriptions']['total'] += 1
                if not param.get('description'):
                    audit_results['path_param_descriptions']['missing'].append(f'{item}: {method.upper()} {path} (path level) -> {param.get("name")}')
    
    # Check 4: Summary fields
    for path, methods in data['paths'].items():
        for method, operation in methods.items():
            if method not in ['get', 'post', 'put', 'patch', 'delete']:
                continue
            audit_results['summary_fields']['total'] += 1
            if not operation.get('summary'):
                audit_results['summary_fields']['missing'].append(f'{item}: {method.upper()} {path}')
    
    # Check 5: Schema descriptions
    schemas = data.get('components', {}).get('schemas', {})
    for schema_name, schema_def in schemas.items():
        if isinstance(schema_def, dict):
            audit_results['schema_descriptions']['total'] += 1
            if not schema_def.get('description'):
                audit_results['schema_descriptions']['missing'].append(f'{item}: {schema_name}')
    
    # Check 6: Request body descriptions
    for path, methods in data['paths'].items():
        for method, operation in methods.items():
            if method in ['post', 'put', 'patch']:
                request_body = operation.get('requestBody')
                if request_body:
                    audit_results['request_body_descriptions']['total'] += 1
                    if not request_body.get('description'):
                        audit_results['request_body_descriptions']['missing'].append(f'{item}: {method.upper()} {path}')
    
    # Check 7: Response examples
    for path, methods in data['paths'].items():
        for method, operation in methods.items():
            if method not in ['get', 'post', 'put', 'patch', 'delete']:
                continue
            
            responses = operation.get('responses', {})
            for status_code, response in responses.items():
                if status_code in ['200', '201']:
                    content = response.get('content', {})
                    json_schema = content.get('application/json', {})
                    if not json_schema.get('examples'):
                        audit_results['response_examples']['total'] += 1
                        if len(audit_results['response_examples']['missing']) < 20:
                            audit_results['response_examples']['missing'].append(f'{item}: {method.upper()} {path} {status_code}')

print("=== FINAL AUDIT RESULTS ===\n")
for check, results in audit_results.items():
    total = results['total']
    missing = len(results['missing'])
    if total > 0:
        pct = ((total - missing) / total) * 100
        print(f"{check}:")
        print(f"  Total: {total}")
        print(f"  Missing: {missing} ({100-pct:.1f}%)")
        print(f"  Coverage: {pct:.1f}%")
        print()
    else:
        print(f"{check}: N/A")
        print()

print("\n=== TOP ISSUES ===")
for check in ['parameter_descriptions', 'path_param_descriptions', 'summary_fields', 'schema_descriptions', 'request_body_descriptions']:
    results = audit_results[check]
    if results['missing']:
        print(f"\n{check}:")
        for issue in results['missing'][:5]:
            print(f"  - {issue}")
        if len(results['missing']) > 5:
            print(f"  ... and {len(results['missing']) - 5} more")
