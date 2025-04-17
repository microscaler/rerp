#!/usr/bin/env python3
"""
Generate system-level OpenAPI specs and READMEs for all RERP systems.

Each top-level system directory gets:
1. README.md - System overview and service catalog
2. openapi.yaml - System gateway/orchestration API spec
"""

import os
import yaml
from pathlib import Path
from typing import Dict, List, Any, Optional
import re

BASE_DIR = Path(__file__).parent.parent
OPENAPI_DIR = BASE_DIR / "openapi"

# System descriptions from MICROSERVICE_MATRIX_AUDIT.md
SYSTEM_DESCRIPTIONS = {
    "accounting": {
        "title": "Accounting Services",
        "description": "Comprehensive financial management including general ledger, accounts payable/receivable, financial reporting, and compliance.",
        "services": [
            ("general-ledger", "Core accounting service managing general ledger and journal entries"),
            ("accounts-payable", "Accounts payable service managing vendor invoices and payments"),
            ("accounts-receivable", "Accounts receivable service managing customer invoices and collections"),
            ("financial-reports", "Financial reporting service generating P&L and balance sheets"),
            ("asset", "Fixed asset management service tracking asset acquisition and depreciation"),
            ("budget", "Budgeting service for budget creation and budget vs actual analysis"),
            ("invoice", "Invoice management service handling invoice creation and approval workflows"),
            ("edi", "Electronic Data Interchange service supporting PEPPOL and UBL formats"),
            ("bank-sync", "Bank synchronization service importing bank statements and reconciliation"),
        ]
    },
    "sales": {
        "title": "Sales Services",
        "description": "Complete sales management from quotations to orders, subscriptions, and loyalty programs.",
        "services": [
            ("core", "Unified sales service orchestrating quotations, orders, and sales workflows"),
            ("quotation", "Quotation management service handling quote creation and approval workflows"),
            ("order", "Sales order management service processing orders and fulfillment"),
            ("subscription", "Subscription management service handling recurring billing and renewals"),
            ("loyalty", "Customer loyalty program service managing points and rewards"),
        ]
    },
    "crm": {
        "title": "CRM Services",
        "description": "Customer relationship management with lead tracking, automation, and live chat.",
        "services": [
            ("core", "Core CRM service managing leads, opportunities, contacts, and sales pipeline"),
            ("automation", "Workflow automation service for CRM pipelines"),
            ("livechat", "Real-time live chat service with visitor tracking and agent assignment"),
        ]
    },
    "auth": {
        "title": "Authentication & Authorization",
        "description": "Secure authentication and role-based access control for all RERP services.",
        "services": [
            ("idam", "Unified authentication and user management service supporting multiple auth methods"),
            ("rbac", "Role-based access control service managing permissions, roles, and authorization"),
        ]
    },
    "product": {
        "title": "Product Management",
        "description": "Product catalog, pricing, and tax calculation services.",
        "services": [
            ("catalog", "Comprehensive product catalog managing SKUs, variants, attributes, and hierarchies"),
            ("pricing", "Dynamic pricing engine supporting multiple price lists and volume discounts"),
            ("tax", "Tax calculation service handling complex tax rules and country-specific logic"),
        ]
    },
    "inventory": {
        "title": "Inventory Management",
        "description": "Complete inventory management from stock tracking to warehouse operations and logistics.",
        "services": [
            ("core", "Inventory management service tracking stock levels and movements"),
            ("warehouse", "Warehouse operations service managing multi-warehouse setups"),
            ("logistics", "Logistics and shipping service integrating with carriers"),
            ("dropshipping", "Dropshipping service managing vendor dropship orders"),
        ]
    },
    "hr": {
        "title": "Human Resources",
        "description": "Complete HR management from employee records to payroll, recruitment, and performance management.",
        "services": [
            ("core", "Core HR service managing employee records and organizational structure"),
            ("attendance", "Attendance tracking service managing clock in/out and work hours"),
            ("leave", "Leave management service handling leave requests and approval workflows"),
            ("payroll", "Payroll service processing salary calculations and payslip generation"),
            ("recruitment", "Recruitment service managing job postings and applicant tracking"),
            ("appraisal", "Performance appraisal service managing review cycles and goal setting"),
            ("skills", "Skills management service tracking employee skills and skill gaps"),
        ]
    },
    "purchase": {
        "title": "Procurement",
        "description": "Purchase order management and vendor relationship management.",
        "services": [
            ("core", "Purchase order management service handling PO creation and approval workflows"),
            ("vendor", "Vendor and supplier management service managing vendor profiles and performance"),
        ]
    },
    "manufacturing": {
        "title": "Manufacturing",
        "description": "Manufacturing operations from BOM management to production planning and repair.",
        "services": [
            ("core", "Core manufacturing service orchestrating production orders and BOM management"),
            ("bom", "Bill of Materials service managing product structures and multi-level BOMs"),
            ("production-planning", "Production planning service handling production scheduling and MRP"),
            ("repair", "Repair service managing product repairs and repair orders"),
            ("subcontracting", "Subcontracting service managing outsourced production"),
        ]
    },
    "project": {
        "title": "Project Management",
        "description": "Project and task management with timesheet tracking.",
        "services": [
            ("core", "Project management service handling project creation and task tracking"),
            ("timesheet", "Timesheet service managing time tracking for projects and tasks"),
        ]
    },
    "marketing": {
        "title": "Marketing",
        "description": "Marketing automation, email campaigns, and social media management.",
        "services": [
            ("email", "Email marketing service managing email campaigns and contact lists"),
            ("automation", "Marketing automation service providing workflow automation and lead nurturing"),
            ("social-media", "Social media service managing multi-platform social media posting"),
        ]
    },
    "website": {
        "title": "Website & E-commerce",
        "description": "Website builder, content management, and e-commerce storefront.",
        "services": [
            ("builder", "Website builder service providing drag-and-drop page builder"),
            ("ecommerce", "E-commerce service managing online store and shopping cart"),
            ("cms", "Content management service for creating and managing website content"),
        ]
    },
    "pos": {
        "title": "Point of Sale",
        "description": "Point of sale and payment gateway services.",
        "services": [
            ("core", "Point of Sale service for retail and restaurant operations"),
            ("payment-gateway", "Payment gateway service integrating with payment providers"),
        ]
    },
    "helpdesk": {
        "title": "Helpdesk",
        "description": "Customer support ticket management and knowledge base.",
        "services": [
            ("core", "Helpdesk service managing customer support tickets and SLA tracking"),
            ("knowledge-base", "Knowledge base service providing self-service documentation"),
        ]
    },
    "analytics": {
        "title": "Analytics & BI",
        "description": "Business intelligence, dashboards, and reporting tools.",
        "services": [
            ("dashboards", "Analytics service providing customizable dashboards and KPI tracking"),
            ("reporting", "Reporting service generating standard and custom reports"),
            ("bi", "Business Intelligence service providing advanced analytics and data warehousing"),
        ]
    },
    "infrastructure": {
        "title": "Infrastructure",
        "description": "API gateway and integration platform services.",
        "services": [
            ("gateway", "Unified API gateway providing routing, rate limiting, and authentication"),
            ("integration-platform", "Platform for managing third-party integrations and webhooks"),
        ]
    },
    "marketplace": {
        "title": "Marketplace",
        "description": "App marketplace and integration hub.",
        "services": [
            ("core", "App marketplace service for discovering and installing extensions"),
            ("integration-hub", "Integration hub service providing pre-built connectors"),
        ]
    },
    "localization": {
        "title": "Localization",
        "description": "Country-specific configurations and compliance management.",
        "services": [
            ("core", "Localization service providing country-specific configurations"),
            ("compliance", "Compliance service managing regulatory compliance and audit trails"),
        ]
    },
    "ai": {
        "title": "AI Services",
        "description": "AI capabilities and document processing.",
        "services": [
            ("core", "Core AI service providing AI capabilities across modules"),
            ("document", "Document AI service using machine learning for document extraction"),
        ]
    },
    "automation": {
        "title": "Workflow Automation",
        "description": "Rule-based workflow automation across all modules.",
        "services": [
            ("core", "Workflow automation service providing rule-based automation"),
        ]
    },
    "documents": {
        "title": "Document Management",
        "description": "Document storage, organization, and version control.",
        "services": [
            ("core", "Document management service for storing and organizing documents"),
        ]
    },
    "appointments": {
        "title": "Appointment Scheduling",
        "description": "Appointment scheduling and calendar integration.",
        "services": [
            ("core", "Appointment scheduling service managing appointments and calendar integration"),
        ]
    },
    "approvals": {
        "title": "Approval Workflows",
        "description": "Multi-level approval processes for business transactions.",
        "services": [
            ("core", "Approval workflow service managing multi-level approval processes"),
        ]
    },
    "data": {
        "title": "Data Management",
        "description": "Data cleaning, deduplication, and quality management.",
        "services": [
            ("cleaning", "Data cleaning service providing data deduplication and quality checks"),
        ]
    },
    "esg": {
        "title": "ESG",
        "description": "Environmental, Social, and Governance tracking and reporting.",
        "services": [
            ("core", "ESG service tracking sustainability metrics and ESG reporting"),
        ]
    },
    "iot": {
        "title": "IoT",
        "description": "IoT device integration and data collection.",
        "services": [
            ("core", "IoT service integrating with IoT devices for data collection"),
        ]
    },
    "field-service": {
        "title": "Field Service",
        "description": "Field service management and technician dispatch.",
        "services": [
            ("core", "Field service management service handling work order scheduling"),
        ]
    },
}

def generate_system_readme(system: str, system_info: Dict[str, Any]) -> str:
    """Generate system-level README."""
    title = system_info["title"]
    description = system_info["description"]
    services = system_info["services"]
    
    readme = f"""# {title}

## Overview

{description}

## Services

"""
    
    for module, service_desc in services:
        readme += f"""### {to_title_case(module)}
- **Path**: `{system}/{module}/`
- **Description**: {service_desc}
- **Documentation**: [{to_title_case(module)} README](./{module}/README.md)
- **API Spec**: [{to_title_case(module)} OpenAPI](./{module}/openapi.yaml)

"""
    
    readme += f"""## API Gateway

This system provides a unified API gateway at `/api/v1/{system}` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The {title.lower()} services work together to provide complete functionality:

"""
    
    # Add integration patterns based on system type
    if system == "accounting":
        readme += """1. **Invoice Flow**: 
   - `invoice/` creates invoices
   - `accounts-receivable/` manages customer invoices
   - `accounts-payable/` manages vendor invoices
   - `general-ledger/` records journal entries

2. **Financial Reporting**:
   - `general-ledger/` provides transaction data
   - `financial-reports/` generates P&L, balance sheets
   - `budget/` compares actuals vs budget

3. **Compliance**:
   - `edi/` handles electronic document exchange
   - `bank-sync/` imports bank statements
   - `asset/` tracks fixed assets
"""
    elif system == "sales":
        readme += """1. **Sales Process**:
   - `quotation/` creates quotes
   - `order/` converts quotes to orders
   - `subscription/` manages recurring sales
   - `loyalty/` tracks customer rewards

2. **Order Fulfillment**:
   - `order/` processes sales orders
   - Integrates with `inventory/` for stock management
   - Integrates with `accounting/` for invoicing
"""
    else:
        readme += f"*Integration patterns specific to {title.lower()} will be documented as services are implemented.*\n"
    
    return readme

def generate_system_openapi(system: str, system_info: Dict[str, Any]) -> Dict[str, Any]:
    """Generate system-level OpenAPI spec."""
    title = system_info["title"]
    description = system_info["description"]
    services = system_info["services"]
    
    spec = {
        "openapi": "3.1.0",
        "info": {
            "title": f"{title} API Gateway",
            "version": "1.0.0",
            "description": f"Aggregated API gateway for all {title.lower()} services. Routes requests to appropriate sub-services and provides system-level operations."
        },
        "servers": [
            {"url": f"/api/v1/{system}", "description": f"{title} API Gateway"}
        ],
        "tags": [
            {"name": "system", "description": "System-level operations"},
            {"name": "services", "description": "Service discovery and management"}
        ],
        "paths": {
            "/health": {
                "get": {
                    "operationId": f"{system}Health",
                    "tags": ["system"],
                    "summary": f"{title} system health check",
                    "responses": {
                        "200": {
                            "description": "System health status",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "status": {"type": "string", "enum": ["healthy", "degraded", "unhealthy"]},
                                            "services": {
                                                "type": "array",
                                                "items": {
                                                    "type": "object",
                                                    "properties": {
                                                        "name": {"type": "string"},
                                                        "status": {"type": "string"},
                                                        "uptime": {"type": "number"}
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/services": {
                "get": {
                    "operationId": f"list{to_pascal_case(system)}Services",
                    "tags": ["services"],
                    "summary": f"List all {title.lower()} services",
                    "responses": {
                        "200": {
                            "description": "List of services",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "items": {
                                                "type": "array",
                                                "items": {
                                                    "type": "object",
                                                    "properties": {
                                                        "name": {"type": "string"},
                                                        "path": {"type": "string"},
                                                        "description": {"type": "string"},
                                                        "status": {"type": "string"}
                                                    }
                                                }
                                            },
                                            "total": {"type": "integer"}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/metrics": {
                "get": {
                    "operationId": f"get{to_pascal_case(system)}Metrics",
                    "tags": ["system"],
                    "summary": f"Get {title.lower()} system metrics",
                    "responses": {
                        "200": {
                            "description": "System metrics",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "requests": {"type": "integer"},
                                            "errors": {"type": "integer"},
                                            "latency": {"type": "number"}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
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
                }
            },
            "schemas": {}
        },
        "x-gateway-config": {
            "description": "Gateway routing configuration for sub-services",
            "services": [
                {
                    "name": module,
                    "path": f"/{module}",
                    "basePath": f"/api/v1/{system}/{module}",
                    "spec": f"./{module}/openapi.yaml"
                }
                for module, _ in services
            ]
        }
    }
    
    return spec

def to_pascal_case(snake_str: str) -> str:
    """Convert snake_case or kebab-case to PascalCase."""
    components = re.split(r'[-_]', snake_str)
    return ''.join(x.title() for x in components)

def to_title_case(snake_str: str) -> str:
    """Convert snake_case or kebab-case to Title Case."""
    components = re.split(r'[-_]', snake_str)
    return ' '.join(x.title() for x in components)

def main():
    """Generate system-level specs and READMEs."""
    print("Generating system-level OpenAPI specs and READMEs...")
    
    generated = 0
    for system, system_info in SYSTEM_DESCRIPTIONS.items():
        system_dir = OPENAPI_DIR / system
        system_dir.mkdir(parents=True, exist_ok=True)
        
        # Generate README
        readme_content = generate_system_readme(system, system_info)
        readme_path = system_dir / "README.md"
        with open(readme_path, 'w') as f:
            f.write(readme_content)
        
        # Generate OpenAPI spec
        spec = generate_system_openapi(system, system_info)
        spec_path = system_dir / "openapi.yaml"
        with open(spec_path, 'w') as f:
            yaml.dump(spec, f, sort_keys=False, default_flow_style=False, allow_unicode=True, width=120)
        
        print(f"✓ Generated: {system}/ ({len(system_info['services'])} services)")
        generated += 1
    
    print(f"\n✅ Generation complete!")
    print(f"Generated system-level specs for {generated} systems")

if __name__ == "__main__":
    main()
