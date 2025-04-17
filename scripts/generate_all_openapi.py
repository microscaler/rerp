#!/usr/bin/env python3
"""
Generate OpenAPI 3.1.0 specifications for all 71 RERP services.

This script generates comprehensive OpenAPI specs with paths and schemas
based on service descriptions. No examples are included in this iteration.
"""

import os
import yaml
from pathlib import Path
from typing import Dict, List, Any, Optional
import re

BASE_DIR = Path(__file__).parent.parent
OPENAPI_DIR = BASE_DIR / "openapi"

# Service definitions with their expected endpoints
SERVICE_DEFINITIONS = {
    # Phase 1: Core Foundation
    ("auth", "idam"): {
        "title": "Identity & Access Management",
        "description": "Unified authentication and user management service supporting multiple auth methods (LDAP, OAuth, Passkeys, TOTP) with session management.",
        "endpoints": [
            ("POST", "/auth/login", "login", "Authenticate user"),
            ("POST", "/auth/logout", "logout", "Logout user"),
            ("POST", "/auth/refresh", "refreshToken", "Refresh access token"),
            ("POST", "/auth/register", "register", "Register new user"),
            ("POST", "/auth/password/reset", "requestPasswordReset", "Request password reset"),
            ("POST", "/auth/password/reset/confirm", "confirmPasswordReset", "Confirm password reset"),
            ("GET", "/users", "listUsers", "List users"),
            ("POST", "/users", "createUser", "Create user"),
            ("GET", "/users/{userId}", "getUser", "Get user by ID"),
            ("PUT", "/users/{userId}", "updateUser", "Update user"),
            ("DELETE", "/users/{userId}", "deleteUser", "Delete user"),
            ("POST", "/users/{userId}/verify/email", "verifyEmail", "Verify email address"),
            ("POST", "/users/{userId}/verify/phone", "verifyPhone", "Verify phone number"),
            ("POST", "/mfa/enable", "enableMfa", "Enable MFA for user"),
            ("POST", "/mfa/verify", "verifyMfa", "Verify MFA code"),
            ("GET", "/sessions", "listSessions", "List active sessions"),
            ("DELETE", "/sessions/{sessionId}", "revokeSession", "Revoke session"),
        ]
    },
    ("auth", "rbac"): {
        "title": "Role-Based Access Control",
        "description": "Role-based access control service managing permissions, roles, and resource-level authorization across all RERP microservices.",
        "endpoints": [
            ("GET", "/roles", "listRoles", "List roles"),
            ("POST", "/roles", "createRole", "Create role"),
            ("GET", "/roles/{roleId}", "getRole", "Get role by ID"),
            ("PUT", "/roles/{roleId}", "updateRole", "Update role"),
            ("DELETE", "/roles/{roleId}", "deleteRole", "Delete role"),
            ("GET", "/permissions", "listPermissions", "List permissions"),
            ("POST", "/permissions", "createPermission", "Create permission"),
            ("GET", "/roles/{roleId}/permissions", "getRolePermissions", "Get permissions for role"),
            ("POST", "/roles/{roleId}/permissions", "assignPermission", "Assign permission to role"),
            ("DELETE", "/roles/{roleId}/permissions/{permissionId}", "revokePermission", "Revoke permission from role"),
            ("GET", "/users/{userId}/roles", "getUserRoles", "Get roles for user"),
            ("POST", "/users/{userId}/roles", "assignRole", "Assign role to user"),
            ("DELETE", "/users/{userId}/roles/{roleId}", "revokeRole", "Revoke role from user"),
            ("POST", "/check", "checkPermission", "Check if user has permission"),
        ]
    },
    # Add more services here - this is a template
}

def generate_crud_paths(resource_name: str, resource_id: str = "id") -> Dict[str, Any]:
    """Generate standard CRUD paths for a resource."""
    resource_singular = resource_name.rstrip('s') if resource_name.endswith('s') else resource_name
    resource_plural = resource_name if resource_name.endswith('s') else f"{resource_name}s"
    
    return {
        f"/{resource_plural}": {
            "get": {
                "operationId": f"list{to_pascal_case(resource_plural)}",
                "summary": f"List {resource_plural}",
                "parameters": [
                    {"$ref": "#/components/parameters/Page"},
                    {"$ref": "#/components/parameters/Limit"},
                    {"$ref": "#/components/parameters/Search"}
                ],
                "responses": {
                    "200": {
                        "description": f"List of {resource_plural}",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "items": {
                                            "type": "array",
                                            "items": {"$ref": f"#/components/schemas/{to_pascal_case(resource_singular)}"}
                                        },
                                        "total": {"type": "integer"},
                                        "page": {"type": "integer"},
                                        "limit": {"type": "integer"}
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "operationId": f"create{to_pascal_case(resource_singular)}",
                "summary": f"Create {resource_singular}",
                "requestBody": {
                    "required": True,
                    "content": {
                        "application/json": {
                            "schema": {"$ref": f"#/components/schemas/Create{to_pascal_case(resource_singular)}Request"}
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": f"{to_pascal_case(resource_singular)} created",
                        "content": {
                            "application/json": {
                                "schema": {"$ref": f"#/components/schemas/{to_pascal_case(resource_singular)}"}
                            }
                        }
                    },
                    "400": {"description": "Invalid request"}
                }
            }
        },
        f"/{resource_plural}/{{{resource_id}}}": {
            "get": {
                "operationId": f"get{to_pascal_case(resource_singular)}",
                "summary": f"Get {resource_singular} by {resource_id}",
                "parameters": [
                    {
                        "name": resource_id,
                        "in": "path",
                        "required": True,
                        "schema": {"type": "string", "format": "uuid"}
                    }
                ],
                "responses": {
                    "200": {
                        "description": f"{to_pascal_case(resource_singular)} details",
                        "content": {
                            "application/json": {
                                "schema": {"$ref": f"#/components/schemas/{to_pascal_case(resource_singular)}"}
                            }
                        }
                    },
                    "404": {"description": "Not found"}
                }
            },
            "put": {
                "operationId": f"update{to_pascal_case(resource_singular)}",
                "summary": f"Update {resource_singular}",
                "parameters": [
                    {
                        "name": resource_id,
                        "in": "path",
                        "required": True,
                        "schema": {"type": "string", "format": "uuid"}
                    }
                ],
                "requestBody": {
                    "required": True,
                    "content": {
                        "application/json": {
                            "schema": {"$ref": f"#/components/schemas/Update{to_pascal_case(resource_singular)}Request"}
                        }
                    }
                },
                "responses": {
                    "200": {
                        "description": f"{to_pascal_case(resource_singular)} updated",
                        "content": {
                            "application/json": {
                                "schema": {"$ref": f"#/components/schemas/{to_pascal_case(resource_singular)}"}
                            }
                        }
                    },
                    "404": {"description": "Not found"}
                }
            },
            "delete": {
                "operationId": f"delete{to_pascal_case(resource_singular)}",
                "summary": f"Delete {resource_singular}",
                "parameters": [
                    {
                        "name": resource_id,
                        "in": "path",
                        "required": True,
                        "schema": {"type": "string", "format": "uuid"}
                    }
                ],
                "responses": {
                    "204": {"description": f"{to_pascal_case(resource_singular)} deleted"},
                    "404": {"description": "Not found"}
                }
            }
        }
    }

def to_pascal_case(snake_str: str) -> str:
    """Convert snake_case to PascalCase."""
    components = snake_str.replace('-', '_').split('_')
    return ''.join(x.title() for x in components)

def to_camel_case(snake_str: str) -> str:
    """Convert snake_case to camelCase."""
    components = snake_str.replace('-', '_').split('_')
    return components[0] + ''.join(x.title() for x in components[1:])

def generate_base_spec(system: str, module: str, title: str, description: str) -> Dict[str, Any]:
    """Generate base OpenAPI spec structure."""
    return {
        "openapi": "3.1.0",
        "info": {
            "title": title,
            "version": "1.0.0",
            "description": description
        },
        "servers": [
            {"url": f"/api/v1/{system}/{module}", "description": f"{title} API"}
        ],
        "tags": [],
        "paths": {},
        "components": {
            "parameters": {
                "Page": {
                    "name": "page",
                    "in": "query",
                    "schema": {"type": "integer", "minimum": 1, "default": 1}
                },
                "Limit": {
                    "name": "limit",
                    "in": "query",
                    "schema": {"type": "integer", "minimum": 1, "maximum": 100, "default": 20}
                },
                "Search": {
                    "name": "search",
                    "in": "query",
                    "schema": {"type": "string"}
                }
            },
            "schemas": {}
        }
    }

def generate_spec_from_definition(system: str, module: str, definition: Dict[str, Any]) -> Dict[str, Any]:
    """Generate OpenAPI spec from service definition."""
    spec = generate_base_spec(system, module, definition["title"], definition["description"])
    
    # Add endpoints
    for method, path, operation_id, summary in definition.get("endpoints", []):
        if path not in spec["paths"]:
            spec["paths"][path] = {}
        
        spec["paths"][path][method.lower()] = {
            "operationId": operation_id,
            "summary": summary,
            "responses": {
                "200": {"description": "Success"},
                "400": {"description": "Bad request"},
                "404": {"description": "Not found"},
                "500": {"description": "Internal server error"}
            }
        }
        
        # Add request body for POST/PUT/PATCH
        if method in ["POST", "PUT", "PATCH"]:
            resource_name = path.split('/')[-1].rstrip('s') if path.split('/')[-1].endswith('s') else path.split('/')[-1]
            spec["paths"][path][method.lower()]["requestBody"] = {
                "required": True,
                "content": {
                    "application/json": {
                        "schema": {
                            "type": "object",
                            "properties": {}
                        }
                    }
                }
            }
        
        # Add path parameters
        path_params = re.findall(r'\{(\w+)\}', path)
        if path_params:
            spec["paths"][path][method.lower()]["parameters"] = [
                {
                    "name": param,
                    "in": "path",
                    "required": True,
                    "schema": {"type": "string", "format": "uuid"}
                }
                for param in path_params
            ]
    
    return spec

def main():
    """Generate OpenAPI specs for all services."""
    print("Generating OpenAPI specifications for RERP services...")
    
    generated = 0
    
    # Generate specs for defined services
    for (system, module), definition in SERVICE_DEFINITIONS.items():
        spec = generate_spec_from_definition(system, module, definition)
        spec_path = OPENAPI_DIR / system / module / "openapi.yaml"
        spec_path.parent.mkdir(parents=True, exist_ok=True)
        
        with open(spec_path, 'w') as f:
            yaml.dump(spec, f, sort_keys=False, default_flow_style=False, allow_unicode=True)
        
        print(f"✓ Generated: {system}/{module}")
        generated += 1
    
    print(f"\nGenerated {generated} of {len(SERVICE_DEFINITIONS)} defined services")
    print(f"Total services to generate: 71")
    print("\nNext: Expand SERVICE_DEFINITIONS to cover all 71 services")

if __name__ == "__main__":
    main()
