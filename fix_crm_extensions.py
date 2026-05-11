"""
Item 2: Add OpenAPI extensions (X-notations) to all operations.

Adds three BRRTRouter-relevant extensions to every operation:
1. x-operation-id: Human-readable ID for logging/tracing
2. x-controller: Controller module path for routing
3. x-handler-name: Handler function name override

Based on BRRTRouter conventions from the workspace.
"""
import yaml
import os

crm_base = '/home/casibbald/Workspace/microscaler/rerp/openapi/crm/'

# Map service names to controller module names
SERVICE_TO_CONTROLLER = {
    'accounts': 'rerp_crm::accounts::controller',
    'automation': 'rerp_crm::automation::controller',
    'contacts': 'rerp_crm::contacts::controller',
    'engagement': 'rerp_crm::engagement::controller',
    'intelligence': 'rerp_crm::intelligence::controller',
    'livechat': 'rerp_crm::livechat::controller',
    'marketing': 'rerp_crm::marketing::controller',
    'pipeline': 'rerp_crm::pipeline::controller',
    'platform': 'rerp_crm::platform::controller',
    'reporting': 'rerp_crm::reporting::controller',
    'teams': 'rerp_crm::teams::controller',
}


def generate_operation_id(service, method, path):
    """Generate a unique operation ID from method and path."""
    # Convert path to snake_case operation name
    path_parts = path.strip('/').split('/')
    operation = path_parts[-1] if path_parts else 'index'
    
    # Remove braces and params
    operation = operation.replace('{', '').replace('}', '')
    
    # Build: {method}_{entity}_{action}
    if len(path_parts) == 1:
        # List operation
        return f"{method}_{operation}_list"
    elif '{' in path:
        # Detail or action operation
        if operation in ['run', 'fire', 'close', 'test', 'sync', 'renew', 'merge', 'convert', 'track', 'identify', 'register', 'submit', 'validate']:
            # Action on resource
            return f"{method}_{path_parts[0]}_{operation}"
        else:
            # Detail operation
            return f"{method}_{operation}_by_id"
    else:
        return f"{method}_{operation}"


def generate_controller(service, method, path):
    """Generate controller module path."""
    controller_module = SERVICE_TO_CONTROLLER.get(service, 'rerp_crm::controller')
    
    # Add method suffix
    return f"{controller_module}::{path.strip('/').replace('/', '::').replace('{', '').replace('}', '')}"


def generate_handler_name(service, method, path):
    """Generate handler function name."""
    # Convert to snake_case
    parts = path.strip('/').replace('{', '').replace('}', '').split('/')
    
    # Build snake_case name
    handler = '_'.join(parts).lower()
    
    # Clean up
    handler = handler.strip('_')
    
    # Limit length
    return handler[:64]


def process_spec(service, spec_path):
    """Process a single OpenAPI spec file."""
    with open(spec_path) as f:
        data = yaml.safe_load(f)
    
    modified = 0
    for path, methods in data['paths'].items():
        for method, operation in methods.items():
            if method not in ['get', 'post', 'put', 'patch', 'delete']:
                continue
            
            # Add x-operation-id
            operation_id = generate_operation_id(service, method, path)
            if operation.get('x-operation-id') != operation_id:
                operation['x-operation-id'] = operation_id
                modified += 1
            
            # Add x-controller
            controller = generate_controller(service, method, path)
            if operation.get('x-controller') != controller:
                operation['x-controller'] = controller
                modified += 1
            
            # Add x-handler-name
            handler_name = generate_handler_name(service, method, path)
            if operation.get('x-handler-name') != handler_name:
                operation['x-handler-name'] = handler_name
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
    
    fixed = process_spec(service, spec_path)
    if fixed:
        print(f"  {service}: {fixed} extensions added")
        total_fixed += fixed

print(f"\nTotal: {total_fixed} operations now have extensions")
