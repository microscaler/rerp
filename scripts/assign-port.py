#!/usr/bin/env python3
"""
Port Registry Manager for RERP Microservices

This script manages port assignments for RERP services, ensuring no conflicts
and automatically updating configuration files.

Usage:
    assign-port.py assign <service-name>          # Assign next available port
    assign-port.py release <service-name>        # Release a port
    assign-port.py list                          # List all assignments
    assign-port.py query <service-name>          # Get port for a service
    assign-port.py update-configs <service-name> # Update config files with assigned port
"""

import json
import os
import sys
import argparse
from pathlib import Path
from typing import Dict, Optional, Tuple
from datetime import datetime
import yaml

# Constants
SCRIPT_DIR = Path(__file__).parent
PROJECT_ROOT = SCRIPT_DIR.parent
REGISTRY_FILE = SCRIPT_DIR / "port-registry.json"
START_PORT = 8001
RESERVED_PORTS = [8080]  # Ports that should never be assigned


class PortRegistry:
    """Manages port assignments for RERP services."""
    
    def __init__(self, registry_file: Path = REGISTRY_FILE):
        self.registry_file = registry_file
        self.registry = self._load_registry()
    
    def _load_registry(self) -> dict:
        """Load the port registry from JSON file."""
        if not self.registry_file.exists():
            # Initialize new registry
            return {
                "version": "1.0",
                "next_port": START_PORT,
                "reserved_ports": RESERVED_PORTS,
                "assignments": {},
                "metadata": {
                    "description": "Port registry for RERP microservices",
                    "last_updated": None,
                    "notes": "Ports start at 8001. Port 8080 is reserved due to conflicts."
                }
            }
        
        with open(self.registry_file, 'r') as f:
            return json.load(f)
    
    def _save_registry(self):
        """Save the port registry to JSON file."""
        self.registry["metadata"]["last_updated"] = datetime.now().isoformat()
        with open(self.registry_file, 'w') as f:
            json.dump(self.registry, f, indent=2)
    
    def _find_next_available_port(self) -> int:
        """Find the next available port, skipping reserved ports."""
        assigned_ports = set(self.registry["assignments"].values())
        reserved = set(self.registry.get("reserved_ports", RESERVED_PORTS))
        
        port = self.registry.get("next_port", START_PORT)
        while port in assigned_ports or port in reserved:
            port += 1
        
        return port
    
    def assign_port(self, service_name: str, force: bool = False) -> Tuple[int, bool]:
        """
        Assign a port to a service.
        
        Returns:
            Tuple of (port_number, was_new_assignment)
        """
        assignments = self.registry["assignments"]
        
        # Check if service already has a port
        if service_name in assignments and not force:
            return assignments[service_name], False
        
        # Find next available port
        port = self._find_next_available_port()
        
        # Assign port
        assignments[service_name] = port
        self.registry["next_port"] = port + 1
        self._save_registry()
        
        return port, True
    
    def release_port(self, service_name: str) -> Optional[int]:
        """Release a port assignment for a service."""
        assignments = self.registry["assignments"]
        if service_name not in assignments:
            return None
        
        port = assignments.pop(service_name)
        self._save_registry()
        return port
    
    def get_port(self, service_name: str) -> Optional[int]:
        """Get the assigned port for a service."""
        return self.registry["assignments"].get(service_name)
    
    def list_assignments(self) -> Dict[str, int]:
        """List all port assignments."""
        return self.registry["assignments"].copy()
    
    def update_config_files(self, service_name: str, port: int):
        """Update configuration files with the assigned port."""
        # NodePort formula: 31000 + (port - 8000)
        # Examples: 8001 -> 31001, 8002 -> 31002, 8003 -> 31003
        # Pattern: 8001 -> 31000 + 1 = 31001
        # This maps service ports 8001-8999 to NodePorts 31000-31999
        # Range 31000-31999 avoids conflicts with PriceWhisperer (30000-30999)
        # Kubernetes NodePort valid range: 30000-32767
        node_port = 31000 + (port - 8000)
        
        # Update Helm values file
        values_file = PROJECT_ROOT / "helm" / "rerp-microservice" / "values" / f"{service_name}.yaml"
        if values_file.exists():
            with open(values_file, 'r') as f:
                values = yaml.safe_load(f) or {}
            
            if "service" not in values:
                values["service"] = {}
            values["service"]["name"] = service_name
            values["service"]["port"] = port
            values["service"]["containerPort"] = port
            values["service"]["nodePort"] = node_port
            
            if "image" not in values:
                values["image"] = {}
            values["image"]["name"] = f"rerp-{service_name}"
            
            if "app" not in values:
                values["app"] = {}
            values["app"]["serviceName"] = service_name
            # Convert service name to binary name (kebab-case to snake_case)
            binary_name = service_name.replace("-", "_") + "_service_api"
            values["app"]["binaryName"] = binary_name
            
            with open(values_file, 'w') as f:
                yaml.dump(values, f, default_flow_style=False, sort_keys=False)
            print(f"✅ Updated {values_file}")
        else:
            print(f"⚠️  Helm values file not found: {values_file}")
            print(f"   Create it with: bootstrap_microservice.py {service_name}")
        
        # Update kind-config.yaml
        kind_config = PROJECT_ROOT / "kind-config.yaml"
        if kind_config.exists():
            with open(kind_config, 'r') as f:
                content = f.read()
            
            # Check if port mapping already exists
            port_mapping = f"  # {service_name.replace('-', ' ').title()} Service\n"
            port_mapping += f"  - containerPort: {node_port}\n"
            port_mapping += f"    hostPort: {port}\n"
            port_mapping += f"    protocol: TCP\n"
            
            # Find insertion point (after extraPortMappings:)
            if f"hostPort: {port}" not in content:
                # Find the last port mapping and insert after it
                lines = content.split('\n')
                insert_index = None
                for i, line in enumerate(lines):
                    if line.strip().startswith('#') and 'Service' in line:
                        # Find the end of this service's port mapping
                        for j in range(i+1, len(lines)):
                            if lines[j].strip() and not lines[j].strip().startswith('#') and 'containerPort' not in lines[j] and 'hostPort' not in lines[j] and 'protocol' not in lines[j]:
                                insert_index = j
                                break
                        if insert_index:
                            break
                
                if insert_index:
                    # Insert the new port mapping
                    indent = "  "
                    new_lines = [
                        indent + f"# {service_name.replace('-', ' ').title()} Service",
                        indent + f"- containerPort: {node_port}",
                        indent + f"  hostPort: {port}",
                        indent + f"  protocol: TCP"
                    ]
                    lines[insert_index:insert_index] = new_lines
                    content = '\n'.join(lines)
                    
                    with open(kind_config, 'w') as f:
                        f.write(content)
                    print(f"✅ Updated {kind_config}")
                else:
                    print(f"⚠️  Could not find insertion point in {kind_config}")
            else:
                print(f"ℹ️  Port mapping already exists in {kind_config}")


def main():
    parser = argparse.ArgumentParser(
        description="Manage port assignments for RERP microservices",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__
    )
    
    subparsers = parser.add_subparsers(dest='command', help='Command to execute')
    
    # Assign command
    assign_parser = subparsers.add_parser('assign', help='Assign a port to a service')
    assign_parser.add_argument('service_name', help='Service name (e.g., general-ledger)')
    assign_parser.add_argument('--force', action='store_true', help='Force reassignment even if port exists')
    assign_parser.add_argument('--update-configs', action='store_true', help='Update config files after assignment')
    
    # Release command
    release_parser = subparsers.add_parser('release', help='Release a port assignment')
    release_parser.add_argument('service_name', help='Service name')
    
    # List command
    subparsers.add_parser('list', help='List all port assignments')
    
    # Query command
    query_parser = subparsers.add_parser('query', help='Get port for a service')
    query_parser.add_argument('service_name', help='Service name')
    
    # Update configs command
    update_parser = subparsers.add_parser('update-configs', help='Update config files with assigned port')
    update_parser.add_argument('service_name', help='Service name')
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        sys.exit(1)
    
    registry = PortRegistry()
    
    if args.command == 'assign':
        port, is_new = registry.assign_port(args.service_name, force=args.force)
        if is_new:
            node_port = 31000 + (port - 8000)
            print(f"✅ Assigned port {port} to service '{args.service_name}'")
            print(f"   NodePort: {node_port}")
        else:
            print(f"ℹ️  Service '{args.service_name}' already has port {port}")
        
        if args.update_configs or is_new:
            registry.update_config_files(args.service_name, port)
    
    elif args.command == 'release':
        port = registry.release_port(args.service_name)
        if port:
            print(f"✅ Released port {port} from service '{args.service_name}'")
        else:
            print(f"⚠️  Service '{args.service_name}' has no assigned port")
    
    elif args.command == 'list':
        assignments = registry.list_assignments()
        if assignments:
            print("\nPort Assignments:")
            print("-" * 50)
            for service, port in sorted(assignments.items()):
                node_port = 31000 + (port - 8000)
                print(f"  {service:30} Port: {port:4}  NodePort: {node_port}")
            print("-" * 50)
            print(f"\nTotal: {len(assignments)} services")
        else:
            print("No port assignments found.")
    
    elif args.command == 'query':
        port = registry.get_port(args.service_name)
        if port:
            node_port = 31000 + (port - 8000)
            print(f"Service: {args.service_name}")
            print(f"  Port: {port}")
            print(f"  NodePort: {node_port}")
        else:
            print(f"⚠️  Service '{args.service_name}' has no assigned port")
            print(f"   Assign one with: {sys.argv[0]} assign {args.service_name}")
            sys.exit(1)
    
    elif args.command == 'update-configs':
        port = registry.get_port(args.service_name)
        if port:
            registry.update_config_files(args.service_name, port)
        else:
            print(f"⚠️  Service '{args.service_name}' has no assigned port")
            print(f"   Assign one first with: {sys.argv[0]} assign {args.service_name}")
            sys.exit(1)


if __name__ == '__main__':
    main()
