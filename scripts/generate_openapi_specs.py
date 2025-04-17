#!/usr/bin/env python3
"""
Generate OpenAPI 3.1.0 specifications for all RERP services.

This script generates comprehensive OpenAPI specs with paths and schemas
based on service descriptions from MICROSERVICE_MATRIX_AUDIT.md and README files.
"""

import os
import yaml
from pathlib import Path
from typing import Dict, List, Any
import re

# Base directory
BASE_DIR = Path(__file__).parent.parent
OPENAPI_DIR = BASE_DIR / "openapi"
COMPONENTS_DIR = BASE_DIR / "components"

def to_camel_case(snake_str: str) -> str:
    """Convert snake_case to camelCase."""
    components = snake_str.split('_')
    return components[0] + ''.join(x.title() for x in components[1:])

def to_pascal_case(snake_str: str) -> str:
    """Convert snake_case to PascalCase."""
    components = snake_str.split('_')
    return ''.join(x.title() for x in components)

def sanitize_name(name: str) -> str:
    """Sanitize name for use in OpenAPI."""
    return name.replace('-', '_').replace(' ', '_').lower()

def generate_idam_spec() -> Dict[str, Any]:
    """Generate OpenAPI spec for Identity & Access Management service."""
    return {
        "openapi": "3.1.0",
        "info": {
            "title": "Identity & Access Management (IDAM)",
            "version": "1.0.0",
            "description": "Unified authentication and user management service supporting multiple auth methods (LDAP, OAuth, Passkeys, TOTP) with session management."
        },
        "servers": [
            {"url": "/api/v1/idam", "description": "IDAM API"}
        ],
        "tags": [
            {"name": "authentication", "description": "User authentication operations"},
            {"name": "users", "description": "User management operations"},
            {"name": "sessions", "description": "Session management operations"},
            {"name": "mfa", "description": "Multi-factor authentication operations"},
            {"name": "verification", "description": "Email and phone verification"}
        ],
        "paths": {
            "/auth/login": {
                "post": {
                    "operationId": "login",
                    "tags": ["authentication"],
                    "summary": "Authenticate user",
                    "requestBody": {
                        "required": True,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/LoginRequest"
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Authentication successful",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/LoginResponse"}
                                }
                            }
                        },
                        "401": {"description": "Invalid credentials"},
                        "403": {"description": "Account locked or disabled"}
                    }
                }
            },
            "/auth/logout": {
                "post": {
                    "operationId": "logout",
                    "tags": ["authentication"],
                    "summary": "Logout user",
                    "responses": {
                        "200": {"description": "Logout successful"},
                        "401": {"description": "Unauthorized"}
                    }
                }
            },
            "/auth/refresh": {
                "post": {
                    "operationId": "refreshToken",
                    "tags": ["authentication"],
                    "summary": "Refresh access token",
                    "requestBody": {
                        "required": True,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/RefreshTokenRequest"
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Token refreshed",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/TokenResponse"}
                                }
                            }
                        },
                        "401": {"description": "Invalid refresh token"}
                    }
                }
            },
            "/auth/register": {
                "post": {
                    "operationId": "register",
                    "tags": ["authentication"],
                    "summary": "Register new user",
                    "requestBody": {
                        "required": True,
                        "content": {
                            "application/json": {
                                "schema": {"$ref": "#/components/schemas/RegisterRequest"}
                            }
                        }
                    },
                    "responses": {
                        "201": {
                            "description": "User registered",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/User"}
                                }
                            }
                        },
                        "400": {"description": "Invalid request"},
                        "409": {"description": "User already exists"}
                    }
                }
            },
            "/auth/password/reset": {
                "post": {
                    "operationId": "requestPasswordReset",
                    "tags": ["authentication"],
                    "summary": "Request password reset",
                    "requestBody": {
                        "required": True,
                        "content": {
                            "application/json": {
                                "schema": {"$ref": "#/components/schemas/PasswordResetRequest"}
                            }
                        }
                    },
                    "responses": {
                        "200": {"description": "Reset email sent"},
                        "404": {"description": "User not found"}
                    }
                }
            },
            "/auth/password/reset/confirm": {
                "post": {
                    "operationId": "confirmPasswordReset",
                    "tags": ["authentication"],
                    "summary": "Confirm password reset",
                    "requestBody": {
                        "required": True,
                        "content": {
                            "application/json": {
                                "schema": {"$ref": "#/components/schemas/PasswordResetConfirm"}
                            }
                        }
                    },
                    "responses": {
                        "200": {"description": "Password reset successful"},
                        "400": {"description": "Invalid token"}
                    }
                }
            },
            "/users": {
                "get": {
                    "operationId": "listUsers",
                    "tags": ["users"],
                    "summary": "List users",
                    "parameters": [
                        {"$ref": "#/components/parameters/Page"},
                        {"$ref": "#/components/parameters/Limit"},
                        {"$ref": "#/components/parameters/Search"}
                    ],
                    "responses": {
                        "200": {
                            "description": "List of users",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/UserList"}
                                }
                            }
                        }
                    }
                },
                "post": {
                    "operationId": "createUser",
                    "tags": ["users"],
                    "summary": "Create user",
                    "requestBody": {
                        "required": True,
                        "content": {
                            "application/json": {
                                "schema": {"$ref": "#/components/schemas/CreateUserRequest"}
                            }
                        }
                    },
                    "responses": {
                        "201": {
                            "description": "User created",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/User"}
                                }
                            }
                        }
                    }
                }
            },
            "/users/{userId}": {
                "get": {
                    "operationId": "getUser",
                    "tags": ["users"],
                    "summary": "Get user by ID",
                    "parameters": [{"$ref": "#/components/parameters/UserId"}],
                    "responses": {
                        "200": {
                            "description": "User details",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/User"}
                                }
                            }
                        },
                        "404": {"description": "User not found"}
                    }
                },
                "put": {
                    "operationId": "updateUser",
                    "tags": ["users"],
                    "summary": "Update user",
                    "parameters": [{"$ref": "#/components/parameters/UserId"}],
                    "requestBody": {
                        "required": True,
                        "content": {
                            "application/json": {
                                "schema": {"$ref": "#/components/schemas/UpdateUserRequest"}
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "User updated",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/User"}
                                }
                            }
                        }
                    }
                },
                "delete": {
                    "operationId": "deleteUser",
                    "tags": ["users"],
                    "summary": "Delete user",
                    "parameters": [{"$ref": "#/components/parameters/UserId"}],
                    "responses": {
                        "204": {"description": "User deleted"},
                        "404": {"description": "User not found"}
                    }
                }
            },
            "/users/{userId}/verify/email": {
                "post": {
                    "operationId": "verifyEmail",
                    "tags": ["verification"],
                    "summary": "Verify email address",
                    "parameters": [{"$ref": "#/components/parameters/UserId"}],
                    "requestBody": {
                        "required": True,
                        "content": {
                            "application/json": {
                                "schema": {"$ref": "#/components/schemas/VerifyEmailRequest"}
                            }
                        }
                    },
                    "responses": {
                        "200": {"description": "Email verified"},
                        "400": {"description": "Invalid verification code"}
                    }
                }
            },
            "/users/{userId}/verify/phone": {
                "post": {
                    "operationId": "verifyPhone",
                    "tags": ["verification"],
                    "summary": "Verify phone number",
                    "parameters": [{"$ref": "#/components/parameters/UserId"}],
                    "requestBody": {
                        "required": True,
                        "content": {
                            "application/json": {
                                "schema": {"$ref": "#/components/schemas/VerifyPhoneRequest"}
                            }
                        }
                    },
                    "responses": {
                        "200": {"description": "Phone verified"},
                        "400": {"description": "Invalid verification code"}
                    }
                }
            },
            "/mfa/enable": {
                "post": {
                    "operationId": "enableMfa",
                    "tags": ["mfa"],
                    "summary": "Enable MFA for user",
                    "requestBody": {
                        "required": True,
                        "content": {
                            "application/json": {
                                "schema": {"$ref": "#/components/schemas/EnableMfaRequest"}
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "MFA enabled",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/MfaSetupResponse"}
                                }
                            }
                        }
                    }
                }
            },
            "/mfa/verify": {
                "post": {
                    "operationId": "verifyMfa",
                    "tags": ["mfa"],
                    "summary": "Verify MFA code",
                    "requestBody": {
                        "required": True,
                        "content": {
                            "application/json": {
                                "schema": {"$ref": "#/components/schemas/VerifyMfaRequest"}
                            }
                        }
                    },
                    "responses": {
                        "200": {"description": "MFA verified"},
                        "400": {"description": "Invalid MFA code"}
                    }
                }
            },
            "/sessions": {
                "get": {
                    "operationId": "listSessions",
                    "tags": ["sessions"],
                    "summary": "List active sessions",
                    "responses": {
                        "200": {
                            "description": "List of sessions",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/SessionList"}
                                }
                            }
                        }
                    }
                }
            },
            "/sessions/{sessionId}": {
                "delete": {
                    "operationId": "revokeSession",
                    "tags": ["sessions"],
                    "summary": "Revoke session",
                    "parameters": [{"$ref": "#/components/parameters/SessionId"}],
                    "responses": {
                        "204": {"description": "Session revoked"},
                        "404": {"description": "Session not found"}
                    }
                }
            }
        },
        "components": {
            "parameters": {
                "UserId": {
                    "name": "userId",
                    "in": "path",
                    "required": True,
                    "schema": {"type": "string", "format": "uuid"}
                },
                "SessionId": {
                    "name": "sessionId",
                    "in": "path",
                    "required": True,
                    "schema": {"type": "string", "format": "uuid"}
                },
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
            "schemas": {
                "LoginRequest": {
                    "type": "object",
                    "required": ["email", "password"],
                    "properties": {
                        "email": {"type": "string", "format": "email"},
                        "password": {"type": "string", "format": "password"},
                        "mfaCode": {"type": "string"},
                        "rememberMe": {"type": "boolean", "default": False}
                    }
                },
                "LoginResponse": {
                    "type": "object",
                    "properties": {
                        "accessToken": {"type": "string"},
                        "refreshToken": {"type": "string"},
                        "expiresIn": {"type": "integer"},
                        "user": {"$ref": "#/components/schemas/User"}
                    }
                },
                "RefreshTokenRequest": {
                    "type": "object",
                    "required": ["refreshToken"],
                    "properties": {
                        "refreshToken": {"type": "string"}
                    }
                },
                "TokenResponse": {
                    "type": "object",
                    "properties": {
                        "accessToken": {"type": "string"},
                        "refreshToken": {"type": "string"},
                        "expiresIn": {"type": "integer"}
                    }
                },
                "RegisterRequest": {
                    "type": "object",
                    "required": ["email", "password", "firstName", "lastName"],
                    "properties": {
                        "email": {"type": "string", "format": "email"},
                        "password": {"type": "string", "format": "password", "minLength": 8},
                        "firstName": {"type": "string"},
                        "lastName": {"type": "string"},
                        "phone": {"type": "string"}
                    }
                },
                "PasswordResetRequest": {
                    "type": "object",
                    "required": ["email"],
                    "properties": {
                        "email": {"type": "string", "format": "email"}
                    }
                },
                "PasswordResetConfirm": {
                    "type": "object",
                    "required": ["token", "newPassword"],
                    "properties": {
                        "token": {"type": "string"},
                        "newPassword": {"type": "string", "format": "password", "minLength": 8}
                    }
                },
                "User": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "format": "uuid"},
                        "email": {"type": "string", "format": "email"},
                        "firstName": {"type": "string"},
                        "lastName": {"type": "string"},
                        "phone": {"type": "string"},
                        "emailVerified": {"type": "boolean"},
                        "phoneVerified": {"type": "boolean"},
                        "mfaEnabled": {"type": "boolean"},
                        "active": {"type": "boolean"},
                        "createdAt": {"type": "string", "format": "date-time"},
                        "updatedAt": {"type": "string", "format": "date-time"}
                    }
                },
                "UserList": {
                    "type": "object",
                    "properties": {
                        "items": {
                            "type": "array",
                            "items": {"$ref": "#/components/schemas/User"}
                        },
                        "total": {"type": "integer"},
                        "page": {"type": "integer"},
                        "limit": {"type": "integer"}
                    }
                },
                "CreateUserRequest": {
                    "type": "object",
                    "required": ["email", "firstName", "lastName"],
                    "properties": {
                        "email": {"type": "string", "format": "email"},
                        "firstName": {"type": "string"},
                        "lastName": {"type": "string"},
                        "phone": {"type": "string"},
                        "password": {"type": "string", "format": "password"}
                    }
                },
                "UpdateUserRequest": {
                    "type": "object",
                    "properties": {
                        "firstName": {"type": "string"},
                        "lastName": {"type": "string"},
                        "phone": {"type": "string"},
                        "active": {"type": "boolean"}
                    }
                },
                "VerifyEmailRequest": {
                    "type": "object",
                    "required": ["code"],
                    "properties": {
                        "code": {"type": "string"}
                    }
                },
                "VerifyPhoneRequest": {
                    "type": "object",
                    "required": ["code"],
                    "properties": {
                        "code": {"type": "string"}
                    }
                },
                "EnableMfaRequest": {
                    "type": "object",
                    "required": ["method"],
                    "properties": {
                        "method": {"type": "string", "enum": ["totp", "sms", "email"]}
                    }
                },
                "MfaSetupResponse": {
                    "type": "object",
                    "properties": {
                        "secret": {"type": "string"},
                        "qrCode": {"type": "string"}
                    }
                },
                "VerifyMfaRequest": {
                    "type": "object",
                    "required": ["code"],
                    "properties": {
                        "code": {"type": "string"}
                    }
                },
                "Session": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "format": "uuid"},
                        "userId": {"type": "string", "format": "uuid"},
                        "ipAddress": {"type": "string"},
                        "userAgent": {"type": "string"},
                        "createdAt": {"type": "string", "format": "date-time"},
                        "expiresAt": {"type": "string", "format": "date-time"}
                    }
                },
                "SessionList": {
                    "type": "object",
                    "properties": {
                        "items": {
                            "type": "array",
                            "items": {"$ref": "#/components/schemas/Session"}
                        }
                    }
                }
            }
        }
    }

def generate_base_spec(service_name: str, system: str, module: str, description: str) -> Dict[str, Any]:
    """Generate base OpenAPI spec structure for a service."""
    service_id = f"{system}_{module}".replace('-', '_')
    service_title = f"{service_name} ({system}/{module})"
    
    return {
        "openapi": "3.1.0",
        "info": {
            "title": service_title,
            "version": "1.0.0",
            "description": description
        },
        "servers": [
            {"url": f"/api/v1/{system}/{module}", "description": f"{service_name} API"}
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

def main():
    """Generate OpenAPI specs for all services."""
    print("Generating OpenAPI specifications for RERP services...")
    
    # Generate IDAM spec (most complex, template for others)
    idam_spec = generate_idam_spec()
    idam_path = OPENAPI_DIR / "auth" / "idam" / "openapi.yaml"
    idam_path.parent.mkdir(parents=True, exist_ok=True)
    with open(idam_path, 'w') as f:
        yaml.dump(idam_spec, f, sort_keys=False, default_flow_style=False, allow_unicode=True)
    print(f"✓ Generated: {idam_path.relative_to(BASE_DIR)}")
    
    # TODO: Generate specs for remaining 70 services
    # This is a template - we'll expand this to generate all services
    
    print("\nGeneration complete!")
    print(f"Generated 1 of 71 services")
    print("Next: Expand generator to cover all services")

if __name__ == "__main__":
    main()
