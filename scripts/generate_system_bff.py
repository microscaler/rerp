#!/usr/bin/env python3
"""
Generate system-level Backend for Frontend (BFF) OpenAPI specifications from sub-service specs.

This script aggregates all paths, schemas, and components from sub-service OpenAPI specs
into a single system-level BFF spec that can proxy requests to the appropriate sub-services.

**IDEMPOTENT & CLOBBER APPROACH**:
- This script completely overwrites the output file each time it runs (clobber approach)
- Running the script multiple times with the same inputs produces identical output (idempotent)
- The output file should NEVER be manually edited - all changes will be overwritten
- All paths, schemas, and tags are sorted deterministically for consistent output

**DIRECTORY-BASED DISCOVERY**:
- Discovers sub-services from openapi/{system}/ directories
- Automatically finds all sub-service OpenAPI specs
- Maps discovered services to their base paths
"""

import yaml
import sys
from pathlib import Path
from typing import Dict, List, Any, Optional
from collections import defaultdict

BASE_DIR = Path(__file__).parent.parent
OPENAPI_DIR = BASE_DIR / "openapi"


def discover_sub_services(system: str) -> Dict[str, Dict[str, Any]]:
    """
    Discover sub-services from OpenAPI directory structure.
    
    Args:
        system: System name (e.g., 'accounting', 'sales')
    
    Returns:
        Dictionary mapping service names to their configuration (spec path, base_path)
    """
    system_dir = OPENAPI_DIR / system
    
    if not system_dir.exists():
        print(f"⚠️  Warning: {system_dir} does not exist")
        return {}
    
    discovered = {}
    
    # List all subdirectories in openapi/{system}/
    for service_path in sorted(system_dir.iterdir()):
        if not service_path.is_dir():
            continue
        
        service_name = service_path.name
        
        # Skip hidden directories and system-level files
        if service_name.startswith('.'):
            continue
        
        # Skip if this is the system-level directory itself (shouldn't happen, but safety check)
        if service_name == system:
            continue
        
        # Find OpenAPI spec file in openapi/{system}/{service}/openapi.yaml
        spec_file = service_path / 'openapi.yaml'
        if not spec_file.exists():
            print(f"⚠️  Warning: OpenAPI spec not found for {service_name}: {spec_file}")
            continue
        
        # Build service configuration
        # Base path follows pattern: /api/v1/{system}/{service}
        base_path = f"/api/v1/{system}/{service_name}"
        discovered[service_name] = {
            'spec': str(spec_file),
            'base_path': base_path,
        }
    
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


def generate_system_bff_spec(system: str, output_path: Path) -> None:
    """
    Generate the system-level BFF OpenAPI spec from all sub-service specs.
    
    **IDEMPOTENT & CLOBBER APPROACH**:
    - Completely overwrites the output file (clobber approach)
    - Produces identical output when run multiple times with same inputs (idempotent)
    - All paths and schemas are sorted deterministically
    
    Args:
        system: System name (e.g., 'accounting', 'sales')
        output_path: Path to output OpenAPI spec file
    """
    print(f"🔄 Generating {system.upper()} system BFF OpenAPI specification (idempotent clobber mode)...")
    
    # Discover sub-services from directory structure
    sub_services = discover_sub_services(system)
    
    if not sub_services:
        print(f"⚠️  No sub-services found for {system} system")
        return
    
    print(f"  📦 Discovered {len(sub_services)} sub-services from openapi/{system}/")
    
    # Build service routing description dynamically
    service_routes = []
    for service_name, service_config in sorted(sub_services.items()):
        service_routes.append(f"- `{service_config['base_path']}/*` → {to_pascal_case(service_name)} Service")
    
    # Load system README to get system description
    system_readme = OPENAPI_DIR / system / "README.md"
    system_title = system.replace('-', ' ').title()
    system_description = f"System-level API gateway for all {system_title} services"
    
    if system_readme.exists():
        with open(system_readme, 'r') as f:
            readme_content = f.read()
            # Extract overview from README
            if "## Overview" in readme_content:
                overview_section = readme_content.split("## Overview")[1].split("##")[0].strip()
                if overview_section:
                    # Get first non-empty line
                    for line in overview_section.split('\n'):
                        line = line.strip()
                        if line and not line.startswith('#'):
                            system_description = line
                            break
    
    # Base BFF spec structure - completely rebuilt each time (clobber approach)
    bff_spec = {
        'openapi': '3.1.0',
        'info': {
            'title': f'{system_title} API Gateway',
            'description': (
                f'{system_description}. '
                f'This aggregates and proxies requests to {system_title} microservices. '
                f'This is the single entry point for the {system_title} system.\n\n'
                'All requests are proxied to the appropriate sub-service:\n'
                + '\n'.join(service_routes) + '\n\n'
                '**Note**: This spec is automatically generated from sub-service specs. '
                'Do not edit manually - changes will be overwritten on next generation.\n\n'
                '**Generation**: This file is completely regenerated (clobbered) each time '
                'the generation script runs, ensuring idempotent output.\n\n'
                f'**Service Discovery**: Sub-services are discovered from openapi/{system}/ directory.'
            ),
            'version': '1.0.0',
        },
        'servers': [
            {
                'url': f'/api/v1/{system}',
                'description': f'{system_title} API Gateway',
            },
        ],
        'tags': [],
        'paths': {},
        'components': {
            'parameters': {
                'Page': {
                    'name': 'page',
                    'in': 'query',
                    'schema': {'type': 'integer', 'minimum': 1, 'default': 1}
                },
                'Limit': {
                    'name': 'limit',
                    'in': 'query',
                    'schema': {'type': 'integer', 'minimum': 1, 'maximum': 100, 'default': 20}
                },
                'Search': {
                    'name': 'search',
                    'in': 'query',
                    'schema': {'type': 'string'}
                }
            },
            'schemas': {},
        },
    }
    
    all_schemas = bff_spec['components']['schemas']
    all_tags = set()
    all_paths = {}
    
    # Process each sub-service in deterministic order (sorted by service name)
    # This ensures idempotent output - same inputs always produce same output
    for service_name, service_config in sorted(sub_services.items()):
        spec_path = Path(service_config['spec'])
        if not spec_path.exists():
            print(f"⚠️  Warning: {spec_path} not found, skipping {service_name}")
            continue
        
        print(f"  📦 Processing {service_name} service...")
        spec = load_spec(spec_path)
        
        # Collect tags
        if 'tags' in spec:
            for tag in spec['tags']:
                if isinstance(tag, dict):
                    all_tags.add(tag.get('name', ''))
                else:
                    all_tags.add(str(tag))
        
        # Merge paths (keep original paths, BFF proxies them as-is)
        if 'paths' in spec:
            for path, path_def in spec['paths'].items():
                # Add x-service extension to track which service handles this path
                if isinstance(path_def, dict):
                    for method in path_def.keys():
                        if method in ['get', 'post', 'put', 'patch', 'delete', 'options', 'head', 'trace']:
                            if isinstance(path_def[method], dict):
                                if 'x-service' not in path_def[method]:
                                    path_def[method]['x-service'] = service_name
                                    path_def[method]['x-service-base-path'] = service_config['base_path']
                
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
        for service_name in sorted(sub_services.keys()):  # Sort for deterministic processing
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
    
    # Add common Error schema if not present
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
    # Find which service has Error and use its prefixed version (sorted for idempotency)
    for service_name in sorted(sub_services.keys()):
        prefixed_error = f"{to_pascal_case(service_name)}Error"
        if prefixed_error in all_schemas:
            update_refs_in_paths(all_paths, 'Error', prefixed_error)
            # Also add a generic Error that references the prefixed one
            all_schemas['Error'] = {'$ref': f'#/components/schemas/{prefixed_error}'}
            break
    
    # Merge common parameters from sub-services
    if 'components' in spec and 'parameters' in spec.get('components', {}):
        for param_name, param_def in spec['components']['parameters'].items():
            if param_name not in bff_spec['components']['parameters']:
                bff_spec['components']['parameters'][param_name] = param_def
    
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
        yaml.dump(bff_spec, f, sort_keys=False, default_flow_style=False, allow_unicode=True, width=120)
    
    path_count = len(bff_spec['paths'])
    schema_count = len(bff_spec['components']['schemas'])
    service_count = len([s for s in sub_services.values() if Path(s['spec']).exists()])
    print(f"✅ Generated {system.upper()} BFF spec (clobbered) with {path_count} paths, {schema_count} schemas from {service_count} services")
    print(f"   Output: {output_path}")
    print(f"   ⚠️  This file is auto-generated - manual edits will be lost on next generation")


def generate_all_system_bffs() -> None:
    """Generate BFF specs for all systems."""
    print("🔄 Generating system-level BFF OpenAPI specifications for all systems...\n")
    
    systems = []
    for system_dir in sorted(OPENAPI_DIR.iterdir()):
        if not system_dir.is_dir():
            continue
        
        system = system_dir.name
        if system.startswith('.'):
            continue
        
        # Check if system has sub-services
        sub_services = discover_sub_services(system)
        if sub_services:
            systems.append(system)
    
    print(f"📦 Found {len(systems)} systems with sub-services\n")
    
    for system in systems:
        output_path = OPENAPI_DIR / system / "openapi.yaml"
        generate_system_bff_spec(system, output_path)
        print()  # Empty line between systems


if __name__ == '__main__':
    if len(sys.argv) > 1:
        # Generate for specific system
        system = sys.argv[1]
        output_file = OPENAPI_DIR / system / "openapi.yaml"
        
        if len(sys.argv) > 2:
            output_file = Path(sys.argv[2])
        
        try:
            generate_system_bff_spec(system, output_file)
            sys.exit(0)
        except Exception as e:
            print(f"❌ Error generating {system} BFF spec: {e}", file=sys.stderr)
            import traceback
            traceback.print_exc()
            sys.exit(1)
    else:
        # Generate for all systems
        try:
            generate_all_system_bffs()
            sys.exit(0)
        except Exception as e:
            print(f"❌ Error generating system BFF specs: {e}", file=sys.stderr)
            import traceback
            traceback.print_exc()
            sys.exit(1)
