# RERP OpenAPI Specifications

This directory contains OpenAPI specifications for all RERP microservices, organized by system and module.

## Directory Structure

```
openapi/
├── {system}/
│   └── {module}/
│       ├── README.md          # Consumer-facing service description
│       └── openapi.yaml       # OpenAPI 3.0 specification
```

## Service Organization

### Core Systems (Phase 1)
- **auth/** - Authentication & Authorization
  - `idam/` - Identity and Access Management
  - `rbac/` - Role-Based Access Control
- **infrastructure/** - Infrastructure Services
  - `gateway/` - API Gateway
  - `integration-platform/` - Integration Platform
- **product/** - Product Management
  - `catalog/` - Product Catalog
  - `pricing/` - Dynamic Pricing
  - `tax/` - Tax Calculation

### Business Operations (Phase 2)
- **crm/** - Customer Relationship Management
  - `core/` - Core CRM
  - `automation/` - CRM Automation
  - `livechat/` - Live Chat
- **sales/** - Sales Management
  - `core/` - Sales Orchestration
  - `quotation/` - Quotation Management
  - `order/` - Order Management
  - `subscription/` - Subscription Management
  - `loyalty/` - Loyalty Programs
- **purchase/** - Procurement
  - `core/` - Purchase Orders
  - `vendor/` - Vendor Management
- **inventory/** - Inventory Management
  - `core/` - Inventory Core
  - `warehouse/` - Warehouse Operations
  - `logistics/` - Logistics & Shipping
  - `dropshipping/` - Dropshipping

### Financial & HR (Phase 3)
- **accounting/** - Accounting Services
  - `general-ledger/` - General Ledger
  - `accounts-payable/` - Accounts Payable
  - `accounts-receivable/` - Accounts Receivable
  - `financial-reports/` - Financial Reporting
  - `asset/` - Asset Management
  - `budget/` - Budgeting
  - `invoice/` - Invoice Management
  - `edi/` - Electronic Data Interchange
  - `bank-sync/` - Bank Synchronization
- **hr/** - Human Resources
  - `core/` - HR Core
  - `attendance/` - Attendance Tracking
  - `leave/` - Leave Management
  - `payroll/` - Payroll Processing
  - `recruitment/` - Recruitment
  - `appraisal/` - Performance Appraisal
  - `skills/` - Skills Management

### Advanced Operations (Phase 4)
- **manufacturing/** - Manufacturing
  - `core/` - Manufacturing Core
  - `bom/` - Bill of Materials
  - `production-planning/` - Production Planning
  - `repair/` - Repair Management
  - `subcontracting/` - Subcontracting
- **project/** - Project Management
  - `core/` - Project Management
  - `timesheet/` - Timesheet Tracking

### Customer-Facing (Phase 5)
- **marketing/** - Marketing
  - `email/` - Email Marketing
  - `automation/` - Marketing Automation
  - `social-media/` - Social Media Management
- **website/** - Website & eCommerce
  - `builder/` - Website Builder
  - `ecommerce/` - E-commerce Platform
  - `cms/` - Content Management
- **pos/** - Point of Sale
  - `core/` - POS Core
  - `payment-gateway/` - Payment Gateway
- **helpdesk/** - Customer Support
  - `core/` - Helpdesk Core
  - `knowledge-base/` - Knowledge Base
- **field-service/** - Field Service
  - `core/` - Field Service Management

### Extensions (Phase 6)
- **marketplace/** - App Marketplace
  - `core/` - Marketplace Core
  - `integration-hub/` - Integration Hub
- **analytics/** - Analytics & BI
  - `dashboards/` - Dashboards
  - `reporting/` - Reporting
  - `bi/` - Business Intelligence

### Additional Services
- **localization/** - Localization
  - `core/` - Localization Core
  - `compliance/` - Compliance Management
- **ai/** - AI & Automation
  - `core/` - AI Core
  - `document/` - Document AI
- **automation/** - Workflow Automation
  - `core/` - Automation Core
- **documents/** - Document Management
  - `core/` - Document Management
- **appointments/** - Appointment Scheduling
  - `core/` - Appointment Scheduling
- **approvals/** - Approval Workflows
  - `core/` - Approval Workflows
- **data/** - Data Management
  - `cleaning/` - Data Cleaning
- **esg/** - ESG Reporting
  - `core/` - ESG Core
- **iot/** - IoT Integration
  - `core/` - IoT Core

## Statistics

- **Total Systems**: 27
- **Total Services**: 60+
- **Total Directories**: 99
- **Total Files**: 196 (98 README.md + 98 openapi.yaml)

## Next Steps

1. ✅ **Iteration 1**: Directory structure and empty files created
2. **Iteration 2**: Create consumer-facing README.md files for each service
3. **Iteration 3**: Generate OpenAPI specifications for each service

## Service Naming Convention

- Services are organized by `{system}/{module}` where:
  - `{system}` is the business domain (e.g., `accounting`, `hr`, `sales`)
  - `{module}` is the specific service within that domain (e.g., `general-ledger`, `payroll`, `quotation`)
- The `-service` suffix is omitted from directory names as all items are services
- Examples:
  - `auth-service` → `auth/idam/`
  - `accounting-service` → `accounting/general-ledger/`
  - `product-catalog-service` → `product/catalog/`

## Documentation Standards

Each service directory contains:
- **README.md**: Consumer-facing description focusing on value proposition and capabilities (sales pitch perspective)
- **openapi.yaml**: OpenAPI 3.0 specification for API implementation
