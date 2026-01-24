#!/usr/bin/env python3
"""
Generate Backend for Frontend (BFF) OpenAPI specification from all microservice specs.

**NOTE**: For openapi/accounting/openapi_bff.yaml, the Tiltfile and CI use the
standalone bff-generator instead. See openapi/accounting/bff-suite-config.yaml and
`bff-generator generate-spec`. This script is kept for reference or fallback.

This script aggregates all paths, schemas, and components from microservice OpenAPI specs
into a single BFF spec that proxies requests to the appropriate microservices.

**IDEMPOTENT & CLOBBER APPROACH**:
- This script completely overwrites the output file each time it runs (clobber approach)
- Running the script multiple times with the same inputs produces identical output (idempotent)
- The output file should NEVER be manually edited - all changes will be overwritten
- All paths, schemas, and tags are sorted deterministically for consistent output

**DIRECTORY-BASED DISCOVERY**:
- Discovers services from openapi/accounting/ directories
- Maps discovered services to OpenAPI specs and port configurations
"""

import yaml
import json
import sys
from pathlib import Path
from typing import Dict, List, Any, Optional
from collections import defaultdict

# Service configuration: maps service name to port and base_path
# NOTE: Order matters for deterministic output - services are processed in this order
# Ports are managed by the port registry system (scripts/assign-port.py)
SERVICE_CONFIG = {
    # Accounting services
    'general-ledger': {'base_path': '/api/general-ledger', 'port': 8001},
    'invoice': {'base_path': '/api/invoice', 'port': 8002},
    'accounts-receivable': {'base_path': '/api/accounts-receivable', 'port': 8003},
    'accounts-payable': {'base_path': '/api/accounts-payable', 'port': 8004},
    'bank-sync': {'base_path': '/api/bank-sync', 'port': 8005},
    'asset': {'base_path': '/api/asset', 'port': 8006},
    'budget': {'base_path': '/api/budget', 'port': 8007},
    'edi': {'base_path': '/api/edi', 'port': 8008},
    'financial-reports': {'base_path': '/api/financial-reports', 'port': 8009},
    # Future services will be added here as they are created
}


def discover_services(service_type: str = 'accounting') -> Dict[str, Dict[str, Any]]:
    """
    Discover services from OpenAPI directory structure.
    
    Args:
        service_type: 'accounting' or other service type
    
    Returns:
        Dictionary mapping service names to their configuration (spec path, base_path, port)
    """
    base_dir = Path('openapi')
    openapi_dir = base_dir / service_type
    
    if not openapi_dir.exists():
        print(f"⚠️  Warning: {openapi_dir} does not exist")
        return {}
    
    discovered = {}
    
    # List all directories in openapi/accounting/
    for service_path in sorted(openapi_dir.iterdir()):
        if not service_path.is_dir():
            continue
        
        service_name = service_path.name
        
        # Skip hidden directories and root openapi.yaml
        if service_name.startswith('.') or service_name == 'openapi.yaml':
            continue
        
        # Check if service has configuration
        if service_name not in SERVICE_CONFIG:
            print(f"⚠️  Warning: {service_name} found in {openapi_dir} but not in SERVICE_CONFIG, skipping")
            continue
        
        # Find OpenAPI spec file in openapi/{service_type}/{service}/openapi.yaml
        spec_file = service_path / 'openapi.yaml'
        if not spec_file.exists():
            print(f"⚠️  Warning: OpenAPI spec not found for {service_name}: {spec_file}")
            continue
        
        # Build service configuration
        config = SERVICE_CONFIG[service_name].copy()
        config['spec'] = str(spec_file)
        discovered[service_name] = config
    
    return discovered


def load_spec(filepath: Path) -> Dict[str, Any]:
    """Load an OpenAPI spec from YAML file."""
    with open(filepath, 'r') as f:
        return yaml.safe_load(f)


def to_pascal_case(name: str) -> str:
    """Convert a service name to PascalCase (e.g., 'general-ledger' -> 'GeneralLedger')."""
    return ''.join(word.capitalize() for word in name.split('-'))


def merge_schemas(all_schemas: Dict[str, Dict[str, Any]], service_name: str, schemas: Dict[str, Any]) -> None:
    """Merge schemas from a service into the aggregated schemas, prefixing to avoid conflicts."""
    for schema_name, schema_def in schemas.items():
        # Prefix schema name with service name to avoid conflicts
        prefixed_name = f"{to_pascal_case(service_name)}{schema_name}"
        all_schemas[prefixed_name] = schema_def
        
        # Update $ref references in the schema to use prefixed names
        if isinstance(schema_def, dict):
            update_refs_in_schema(schema_def, schema_name, prefixed_name)


def update_refs_in_schema(schema: Dict[str, Any], old_name: str, new_name: str) -> None:
    """Recursively update $ref references in a schema."""
    if isinstance(schema, dict):
        if '$ref' in schema:
            ref = schema['$ref']
            if f'#/components/schemas/{old_name}' in ref:
                schema['$ref'] = ref.replace(f'#/components/schemas/{old_name}', f'#/components/schemas/{new_name}')
        for value in schema.values():
            if isinstance(value, (dict, list)):
                update_refs_in_schema(value, old_name, new_name)
    elif isinstance(schema, list):
        for item in schema:
            if isinstance(item, (dict, list)):
                update_refs_in_schema(item, old_name, new_name)


def update_refs_in_paths(paths: Dict[str, Any], old_name: str, new_name: str) -> None:
    """Update $ref references in paths to use prefixed schema names."""
    for path_def in paths.values():
        if isinstance(path_def, dict):
            for method, operation in path_def.items():
                if method in ['get', 'post', 'put', 'patch', 'delete', 'options', 'head', 'trace']:
                    if isinstance(operation, dict):
                        # Update requestBody refs
                        if 'requestBody' in operation:
                            update_refs_in_schema(operation['requestBody'], old_name, new_name)
                        # Update responses refs
                        if 'responses' in operation:
                            for response in operation['responses'].values():
                                if isinstance(response, dict) and 'content' in response:
                                    update_refs_in_schema(response['content'], old_name, new_name)


def update_all_refs_in_schema(schema: Dict[str, Any], schema_name_mapping: Dict[str, List[str]], all_schemas: Dict[str, Any]) -> None:
    """Recursively update all $ref references in a schema to use prefixed names."""
    if isinstance(schema, dict):
        if '$ref' in schema:
            ref = schema['$ref']
            # Extract schema name from ref (e.g., '#/components/schemas/UserDetails')
            if '#/components/schemas/' in ref:
                unprefixed_name = ref.split('#/components/schemas/')[-1]
                # If this is an unprefixed name and we have a mapping, update it
                if unprefixed_name in schema_name_mapping:
                    # Use the first prefixed name (prefer service-specific ones)
                    prefixed_names = schema_name_mapping[unprefixed_name]
                    if prefixed_names:
                        # Prefer the one that exists in all_schemas
                        for prefixed in prefixed_names:
                            if prefixed in all_schemas:
                                schema['$ref'] = ref.replace(unprefixed_name, prefixed)
                                break
        # Recursively process all values
        for value in schema.values():
            if isinstance(value, (dict, list)):
                update_all_refs_in_schema(value, schema_name_mapping, all_schemas)
    elif isinstance(schema, list):
        for item in schema:
            if isinstance(item, (dict, list)):
                update_all_refs_in_schema(item, schema_name_mapping, all_schemas)


def generate_bff_spec(output_path: Path, service_type: str = 'accounting') -> None:
    """
    Generate the BFF OpenAPI spec from all microservice specs.
    
    **IDEMPOTENT & CLOBBER APPROACH**:
    - Completely overwrites the output file (clobber approach)
    - Produces identical output when run multiple times with same inputs (idempotent)
    - All paths and schemas are sorted deterministically
    
    Args:
        output_path: Path to output OpenAPI spec file
        service_type: 'accounting' or other service type - determines which services to include
    """
    print(f"🔄 Generating {service_type.upper()} BFF OpenAPI specification (idempotent clobber mode)...")
    
    # Discover services from directory structure
    microservices = discover_services(service_type)
    
    if not microservices:
        print(f"❌ No services found for {service_type} BFF")
        sys.exit(1)
    
    print(f"  📦 Discovered {len(microservices)} services from openapi/{service_type}/")
    
    # Build service routing description dynamically
    service_routes = []
    for service_name, service_config in sorted(microservices.items()):
        service_routes.append(f"- `{service_config['base_path']}/*` → {service_name.replace('-', ' ').title()} Service (port {service_config['port']})")
    
    # Base BFF spec structure - completely rebuilt each time (clobber approach)
    bff_spec = {
        'openapi': '3.1.0',
        'info': {
            'title': f'RERP {service_type.title()} Backend for Frontend API',
            'description': (
                f'Backend for Frontend (BFF) API for the {service_type.title()} Suite. '
                'This aggregates and proxies requests to RERP microservices. '
                'This is the single entry point for the frontend application.\n\n'
                'All requests are proxied to the appropriate microservice:\n'
                + '\n'.join(service_routes) + '\n\n'
                '**Note**: This spec is automatically generated from microservice specs. '
                'Do not edit manually - changes will be overwritten on next generation.\n\n'
                '**Generation**: This file is completely regenerated (clobbered) each time '
                'the generation script runs, ensuring idempotent output.\n\n'
                f'**Service Discovery**: Services are discovered from openapi/{service_type}/ directory.'
            ),
            'version': '1.0.0',
            'contact': {
                'name': 'RERP API Support',
                'email': 'api-support@rerp.ai',
            },
        },
        'servers': [
            {
                'url': 'https://api.rerp.ai',
                'description': 'Production server',
            },
            {
                'url': 'http://localhost:8000',
                'description': 'Local development server (BFF)',
            },
        ],
        'tags': [],
        'paths': {},
        'components': {
            'securitySchemes': {
                'ApiKeyHeader': {
                    'type': 'apiKey',
                    'in': 'header',
                    'name': 'X-API-KEY',
                    'description': 'API key for authenticating requests to the BFF. The BFF will forward this to the appropriate microservice.',
                },
            },
            'schemas': {},
        },
        'security': [
            {'ApiKeyHeader': []},
        ],
    }
    
    all_schemas = bff_spec['components']['schemas']
    all_tags = set()
    all_paths = {}
    
    # Process each microservice in deterministic order (sorted by service name)
    # This ensures idempotent output - same inputs always produce same output
    for service_name, service_config in sorted(microservices.items()):
        spec_path = Path(service_config['spec'])
        if not spec_path.exists():
            print(f"⚠️  Warning: {spec_path} not found, skipping {service_name}")
            continue
        
        print(f"  📦 Processing {service_name} service...")
        spec = load_spec(spec_path)
        
        # Collect tags
        if 'tags' in spec:
            for tag in spec['tags']:
                all_tags.add(tag.get('name', ''))
        
        # Merge paths (keep original paths, BFF proxies them as-is)
        if 'paths' in spec:
            for path, path_def in spec['paths'].items():
                # Add x-service extension to track which service handles this path
                if isinstance(path_def, dict):
                    for method in path_def.keys():
                        if method in ['get', 'post', 'put', 'patch', 'delete', 'options', 'head', 'trace']:
                            if 'x-service' not in path_def[method]:
                                path_def[method]['x-service'] = service_name
                                path_def[method]['x-service-port'] = service_config['port']
                
                all_paths[path] = path_def
        
        # Merge schemas with prefixing
        if 'components' in spec and 'schemas' in spec['components']:
            merge_schemas(all_schemas, service_name, spec['components']['schemas'])
            
            # Update refs in paths to use prefixed schema names
            for schema_name, schema_def in spec['components']['schemas'].items():
                prefixed_name = f"{to_pascal_case(service_name)}{schema_name}"
                update_refs_in_paths(all_paths, schema_name, prefixed_name)
    
    # Second pass: Update all nested $ref references in all schemas to use prefixed names
    print("  🔧 Updating nested schema references...")
    schema_name_mapping = {}  # Map unprefixed -> prefixed names
    for prefixed_name in sorted(all_schemas.keys()):  # Sort for deterministic processing
        # Extract unprefixed name if it's a prefixed one
        for service_name in sorted(microservices.keys()):  # Sort for deterministic processing
            prefix = to_pascal_case(service_name)
            if prefixed_name.startswith(prefix):
                unprefixed = prefixed_name[len(prefix):]
                if unprefixed not in schema_name_mapping:
                    schema_name_mapping[unprefixed] = []
                schema_name_mapping[unprefixed].append(prefixed_name)
    
    # Update all nested references in schemas (sorted for deterministic processing)
    for schema_name, schema_def in sorted(all_schemas.items()):
        if isinstance(schema_def, dict):
            update_all_refs_in_schema(schema_def, schema_name_mapping, all_schemas)
    
    # Add common Error schema (use from first service that has it, or create generic)
    if 'Error' not in all_schemas:
        all_schemas['Error'] = {
            'type': 'object',
            'required': ['error', 'message'],
            'properties': {
                'error': {
                    'type': 'string',
                    'description': 'Error code',
                },
                'message': {
                    'type': 'string',
                    'description': 'Human-readable error message',
                },
                'details': {
                    'type': 'object',
                    'nullable': True,
                    'description': 'Additional error details',
                    'additionalProperties': True,
                },
            },
        }
    
    # Update refs in all paths to use prefixed Error schema
    for service_name in sorted(microservices.keys()):
        prefixed_error = f"{to_pascal_case(service_name)}Error"
        if prefixed_error in all_schemas:
            update_refs_in_paths(all_paths, 'Error', prefixed_error)
            # Also add a generic Error that references the prefixed one
            all_schemas['Error'] = {'$ref': f'#/components/schemas/{prefixed_error}'}
            break
    
    # Set aggregated data - sorted for deterministic output (idempotency)
    bff_spec['tags'] = sorted([{'name': tag} for tag in all_tags], key=lambda x: x['name'])
    bff_spec['paths'] = dict(sorted(all_paths.items()))  # Sort paths for deterministic output
    
    # Sort schemas for deterministic output (idempotency)
    bff_spec['components']['schemas'] = dict(sorted(all_schemas.items()))
    
    # Write output - CLOBBER APPROACH: completely overwrite the file
    output_path.parent.mkdir(parents=True, exist_ok=True)
    
    # Remove existing file if it exists to ensure clean clobber
    if output_path.exists():
        output_path.unlink()
    
    # Write new file (complete overwrite)
    with open(output_path, 'w') as f:
        yaml.dump(bff_spec, f, sort_keys=False, default_flow_style=False, allow_unicode=True)
    
    path_count = len(bff_spec['paths'])
    schema_count = len(bff_spec['components']['schemas'])
    service_count = len([s for s in microservices.values() if Path(s['spec']).exists()])
    print(f"✅ Generated {service_type.upper()} BFF spec (clobbered) with {path_count} paths, {schema_count} schemas from {service_count} services")
    print(f"   Output: {output_path}")
    print(f"   ⚠️  This file is auto-generated - manual edits will be lost on next generation")


if __name__ == '__main__':
    # Parse arguments: [accounting|...] [output_file]
    service_type = 'accounting'
    output_file = Path('openapi/accounting/openapi_bff.yaml')
    
    if len(sys.argv) > 1:
        arg1 = sys.argv[1].lower()
        if arg1 in ['accounting']:  # Add other service types as needed
            service_type = arg1
            output_file = Path(f'openapi/{service_type}/openapi_bff.yaml')
            if len(sys.argv) > 2:
                output_file = Path(sys.argv[2])
        else:
            # Legacy: first arg is output file
            output_file = Path(sys.argv[1])
    
    try:
        generate_bff_spec(output_file, service_type)
        sys.exit(0)
    except Exception as e:
        print(f"❌ Error generating {service_type} BFF spec: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)
