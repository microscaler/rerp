# Accounting Services

## Overview

Comprehensive financial management including general ledger, accounts payable/receivable, financial reporting, and compliance.

## Services

### General Ledger
- **Path**: `accounting/general-ledger/`
- **Description**: Core accounting service managing general ledger and journal entries
- **Documentation**: [General Ledger README](./general-ledger/README.md)
- **API Spec**: [General Ledger OpenAPI](./general-ledger/openapi.yaml)

### Accounts Payable
- **Path**: `accounting/accounts-payable/`
- **Description**: Accounts payable service managing vendor invoices and payments
- **Documentation**: [Accounts Payable README](./accounts-payable/README.md)
- **API Spec**: [Accounts Payable OpenAPI](./accounts-payable/openapi.yaml)

### Accounts Receivable
- **Path**: `accounting/accounts-receivable/`
- **Description**: Accounts receivable service managing customer invoices and collections
- **Documentation**: [Accounts Receivable README](./accounts-receivable/README.md)
- **API Spec**: [Accounts Receivable OpenAPI](./accounts-receivable/openapi.yaml)

### Financial Reports
- **Path**: `accounting/financial-reports/`
- **Description**: Financial reporting service generating P&L and balance sheets
- **Documentation**: [Financial Reports README](./financial-reports/README.md)
- **API Spec**: [Financial Reports OpenAPI](./financial-reports/openapi.yaml)

### Asset
- **Path**: `accounting/asset/`
- **Description**: Fixed asset management service tracking asset acquisition and depreciation
- **Documentation**: [Asset README](./asset/README.md)
- **API Spec**: [Asset OpenAPI](./asset/openapi.yaml)

### Budget
- **Path**: `accounting/budget/`
- **Description**: Budgeting service for budget creation and budget vs actual analysis
- **Documentation**: [Budget README](./budget/README.md)
- **API Spec**: [Budget OpenAPI](./budget/openapi.yaml)

### Invoice
- **Path**: `accounting/invoice/`
- **Description**: Invoice management service handling invoice creation and approval workflows
- **Documentation**: [Invoice README](./invoice/README.md)
- **API Spec**: [Invoice OpenAPI](./invoice/openapi.yaml)

### Edi
- **Path**: `accounting/edi/`
- **Description**: Electronic Data Interchange service supporting PEPPOL and UBL formats
- **Documentation**: [Edi README](./edi/README.md)
- **API Spec**: [Edi OpenAPI](./edi/openapi.yaml)

### Bank Sync
- **Path**: `accounting/bank-sync/`
- **Description**: Bank synchronization service importing bank statements and reconciliation
- **Documentation**: [Bank Sync README](./bank-sync/README.md)
- **API Spec**: [Bank Sync OpenAPI](./bank-sync/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/accounting` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The accounting services services work together to provide complete functionality:

1. **Invoice Flow**: 
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
