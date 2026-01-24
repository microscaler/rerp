"""`rerp ports` subcommand: assign, list, validate, update-configs, reconcile, fix-duplicates."""

from pathlib import Path

from rerp_tooling.ports import (
    PortRegistry,
    fix_duplicates,
    reconcile,
    validate,
)


def run_ports(args, project_root: Path, registry_path: Path) -> int:
    registry = PortRegistry(registry_path, project_root)

    if args.ports_cmd == "assign":
        port, is_new = registry.assign_port(
            args.service_name,
            force=getattr(args, "force", False),
            preferred_port=getattr(args, "port", None),
        )
        if is_new:
            node_port = 31000 + (port - 8000)
            print(f"✅ Assigned port {port} to service '{args.service_name}'")
            print(f"   NodePort: {node_port}")
        else:
            print(f"Info:  Service '{args.service_name}' already has port {port}")
        if getattr(args, "update_configs", False) or is_new:
            registry.update_config_files(args.service_name, port)
        return 0

    if args.ports_cmd == "release":
        port = registry.release_port(args.service_name)
        if port:
            print(f"✅ Released port {port} from service '{args.service_name}'")
        else:
            print(f"⚠️  Service '{args.service_name}' has no assigned port")
        return 0

    if args.ports_cmd == "list":
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
        return 0

    if args.ports_cmd == "query":
        port = registry.get_port(args.service_name)
        if port:
            node_port = 31000 + (port - 8000)
            print(f"Service: {args.service_name}")
            print(f"  Port: {port}")
            print(f"  NodePort: {node_port}")
        else:
            print(f"⚠️  Service '{args.service_name}' has no assigned port")
            print(f"   Assign one with: rerp ports assign {args.service_name}")
            return 1
        return 0

    if args.ports_cmd == "update-configs":
        port = registry.get_port(args.service_name)
        if port:
            registry.update_config_files(args.service_name, port)
        else:
            print(f"⚠️  Service '{args.service_name}' has no assigned port")
            print(f"   Assign one first with: rerp ports assign {args.service_name}")
            return 1
        return 0

    if args.ports_cmd == "validate":
        return validate(registry, project_root, json_out=getattr(args, "json", False))

    if args.ports_cmd == "reconcile":
        return reconcile(
            registry,
            project_root,
            update_configs=getattr(args, "update_configs", False),
        )

    if args.ports_cmd == "fix-duplicates":
        return fix_duplicates(registry, project_root, dry_run=getattr(args, "dry_run", False))

    return 0
