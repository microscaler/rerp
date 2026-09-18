# OpenAPI Examples PRD - Accounting Suite

## Overview

This PRD defines the requirements for adding comprehensive example data to all OpenAPI specifications in the RERP accounting suite. The goal is to provide UI developers with realistic, interconnected example data that enables rapid UI development and testing without requiring a live backend.

## Background

The BRRTRouter petstore example demonstrates how OpenAPI examples enable:
- **Rapid UI Development**: UI developers can immediately see what data structures look like
- **API Testing**: Examples can be used to populate request bodies and validate responses
- **Documentation**: Examples serve as living documentation showing real-world usage
- **Mocking**: Examples enable frontend teams to build and test UIs independently

## Current State

The accounting OpenAPI specs (`openapi/accounting/`) currently have:
- ✅ Complete schema definitions for all entities
- ✅ Request/response schemas (Create/Update patterns)
- ✅ Path definitions with operations (GET, POST, PUT, DELETE)
- ❌ **No example data** in any paths or schemas

## Requirements

### 1. Example Coverage

**All paths must have examples for:**
- ✅ **Response examples** (200/201 responses) - 3 examples per endpoint
- ✅ **Request body examples** (POST/PUT operations) - 3 examples per endpoint
- ✅ **Error response examples** (400, 404, 500) - At least 1 example per error type

**All schemas should have:**
- ✅ **Example values** in schema `example` fields (for Swagger UI display)
- ✅ **Multiple examples** in path `examples` objects (for programmatic access)

### 2. Example Data Sets

We need **3 distinct example data sets** that represent different scenarios:

#### **Example Set 1: "Acme Corporation" (Primary Company)**
- **Company ID**: `550e8400-e29b-41d4-a716-446655440000`
- **Scenario**: Large enterprise with multiple customers, vendors, and complex transactions
- **Characteristics**:
  - Multiple currencies (USD, EUR, GBP)
  - International customers and vendors
  - Complex invoice structures with many line items
  - Active AR/AP aging reports
  - Multiple bank accounts
  - Comprehensive asset register
  - Multi-period budgets

#### **Example Set 2: "TechStart Inc" (Small Business)**
- **Company ID**: `660e8400-e29b-41d4-a716-446655440001`
- **Scenario**: Small startup with simple accounting needs
- **Characteristics**:
  - Single currency (USD)
  - Limited customer/vendor base
  - Simple invoices (few line items)
  - Basic AR/AP tracking
  - Single bank account
  - Minimal assets
  - Simple budget structure

#### **Example Set 3: "Global Manufacturing Ltd" (Multi-Currency)**
- **Company ID**: `770e8400-e29b-41d4-a716-446655440002`
- **Scenario**: Manufacturing company with complex multi-currency operations
- **Characteristics**:
  - Heavy multi-currency usage (USD, EUR, JPY, CNY)
  - Complex inventory-linked invoices
  - EDI integration examples
  - Asset depreciation schedules
  - Detailed financial reports
  - Advanced budget variance analysis

### 3. Data Linking Requirements

Examples must be **properly linked** across services to enable realistic UI workflows:

#### **Cross-Entity References**
- **Invoice → Customer/Vendor**: Invoice examples must reference valid customer/vendor IDs from AR/AP examples
- **Invoice Line → Invoice**: Invoice line examples must reference valid invoice IDs
- **AR Payment → Customer Invoice**: AR payment examples must reference valid customer invoice IDs
- **AP Payment → Vendor Invoice**: AP payment examples must reference valid vendor invoice IDs
- **Journal Entry → Account**: Journal entry examples must reference valid account IDs from General Ledger
- **Journal Entry Line → Journal Entry**: Journal entry line examples must reference valid journal entry IDs
- **Account Balance → Account**: Account balance examples must reference valid account IDs
- **Bank Transaction → Bank Account**: Bank transaction examples must reference valid bank account IDs
- **Bank Reconciliation → Bank Statement**: Bank reconciliation examples must reference valid bank statement IDs
- **Asset Depreciation → Asset**: Asset depreciation examples must reference valid asset IDs
- **Budget Line Item → Budget**: Budget line item examples must reference valid budget IDs
- **Budget Actual → Budget**: Budget actual examples must reference valid budget and account IDs
- **EDI Document → EDI Format**: EDI document examples must reference valid EDI format IDs
- **Financial Report → Report Template**: Financial report examples must reference valid report template IDs
- **Report Data → Financial Report**: Report data examples must reference valid financial report IDs

#### **Consistent UUIDs Across Examples**
All examples must use **consistent UUIDs** for the same entities across different endpoints:

```yaml
# Example: Customer ID used consistently
customer_id: "111e8400-e29b-41d4-a716-446655440001"  # Used in:
  - Invoice examples (customer_id)
  - AR Customer Invoice examples (customer_id)
  - AR Payment examples (customer_id)
  - AR Aging examples (customer_id)
```

#### **Date Consistency**
Dates must be **logically consistent**:
- Invoice dates must be before due dates
- Payment dates must be after invoice dates
- Journal entry dates must align with invoice dates
- Aging dates must be after invoice dates
- Budget periods must have valid start/end dates
- Asset depreciation periods must align with asset purchase dates

### 4. Example Structure

#### **Response Examples Format**

```yaml
responses:
  '200':
    description: List of invoices
    content:
      application/json:
        schema:
          type: object
          properties:
            items:
              type: array
              items:
                $ref: '#/components/schemas/Invoice'
        examples:
          acmeCorporationExample:
            summary: Acme Corporation - Large enterprise invoice
            value:
              items:
                - id: "a001e8400-e29b-41d4-a716-446655440000"
                  invoice_number: "INV-2024-001"
                  customer_id: "111e8400-e29b-41d4-a716-446655440001"
                  # ... full invoice data
          techStartExample:
            summary: TechStart Inc - Small business invoice
            value:
              items:
                - id: "b001e8400-e29b-41d4-a716-446655440001"
                  invoice_number: "INV-2024-100"
                  customer_id: "222e8400-e29b-41d4-a716-446655440002"
                  # ... full invoice data
          globalManufacturingExample:
            summary: Global Manufacturing - Multi-currency invoice
            value:
              items:
                - id: "c001e8400-e29b-41d4-a716-446655440002"
                  invoice_number: "INV-2024-200"
                  customer_id: "333e8400-e29b-41d4-a716-446655440003"
                  currency_code: "EUR"
                  # ... full invoice data
```

#### **Request Body Examples Format**

```yaml
requestBody:
  required: true
  content:
    application/json:
      schema:
        $ref: '#/components/schemas/CreateInvoiceRequest'
      examples:
        acmeCorporationRequest:
          summary: Create invoice for Acme Corporation customer
          value:
            invoice_number: "INV-2024-001"
            invoice_date: "2024-01-15"
            customer_id: "111e8400-e29b-41d4-a716-446655440001"
            # ... required fields only
        techStartRequest:
          summary: Create invoice for TechStart customer
          value:
            invoice_number: "INV-2024-100"
            invoice_date: "2024-01-20"
            customer_id: "222e8400-e29b-41d4-a716-446655440002"
            # ... required fields only
        globalManufacturingRequest:
          summary: Create multi-currency invoice
          value:
            invoice_number: "INV-2024-200"
            invoice_date: "2024-01-25"
            customer_id: "333e8400-e29b-41d4-a716-446655440003"
            currency_code: "EUR"
            # ... required fields only
```

#### **Schema-Level Examples**

```yaml
components:
  schemas:
    Invoice:
      type: object
      properties:
        id:
          type: string
          format: uuid
          example: "a001e8400-e29b-41d4-a716-446655440000"
        invoice_number:
          type: string
          maxLength: 100
          example: "INV-2024-001"
        # ... other properties with examples
```

### 5. Implementation Priority

#### **Phase 1: Core Services (Week 1)**
1. **General Ledger**
   - Accounts (3 examples)
   - Journal Entries (3 examples)
   - Journal Entry Lines (3 examples)
   - Chart of Accounts (3 examples)
   - Account Balances (3 examples)

2. **Invoice**
   - Invoices (3 examples)
   - Invoice Lines (3 examples)

#### **Phase 2: AR/AP Services (Week 2)**
3. **Accounts Receivable**
   - Customer Invoices (3 examples)
   - AR Payments (3 examples)
   - AR Payment Applications (3 examples)
   - AR Aging (3 examples)

4. **Accounts Payable**
   - Vendor Invoices (3 examples)
   - AP Payments (3 examples)
   - AP Payment Applications (3 examples)
   - AP Aging (3 examples)

#### **Phase 3: Supporting Services (Week 3)**
5. **Bank Sync**
   - Bank Accounts (3 examples)
   - Bank Transactions (3 examples)
   - Bank Statements (3 examples)
   - Bank Reconciliations (3 examples)

6. **Asset Management**
   - Assets (3 examples)
   - Asset Categories (3 examples)
   - Asset Depreciation (3 examples)
   - Asset Transactions (3 examples)

#### **Phase 4: Advanced Services (Week 4)**
7. **Budget Management**
   - Budgets (3 examples)
   - Budget Periods (3 examples)
   - Budget Line Items (3 examples)
   - Budget Versions (3 examples)
   - Budget Actuals (3 examples)

8. **EDI**
   - EDI Documents (3 examples)
   - EDI Formats (3 examples)
   - EDI Mappings (3 examples)
   - EDI Acknowledgments (3 examples)

9. **Financial Reports**
   - Financial Reports (3 examples)
   - Report Templates (3 examples)
   - Report Schedules (3 examples)
   - Report Data (3 examples)

### 6. UUID Naming Convention

To ensure consistency and traceability, use a structured UUID pattern:

```
{prefix}{entity_type}{company_index}{entity_index}
```

**Example:**
- `a001e8400-e29b-41d4-a716-446655440000`
  - `a00` = Acme Corporation (company index 001)
  - `1e` = Invoice entity type (1 = Invoice)
  - `8400-e29b-41d4-a716-446655440000` = Entity instance

**Entity Type Codes:**
- `1e` = Invoice
- `2e` = Invoice Line
- `3e` = Customer Invoice (AR)
- `4e` = AR Payment
- `5e` = Vendor Invoice (AP)
- `6e` = AP Payment
- `7e` = Account
- `8e` = Journal Entry
- `9e` = Journal Entry Line
- `ae` = Bank Account
- `be` = Bank Transaction
- `ce` = Asset
- `de` = Budget
- `ee` = EDI Document
- `fe` = Financial Report

**Company Index Codes:**
- `001` = Acme Corporation
- `002` = TechStart Inc
- `003` = Global Manufacturing Ltd

### 7. Data Quality Requirements

#### **Realistic Values**
- Invoice numbers should follow realistic patterns (e.g., "INV-2024-001", "INV-2024-100")
- Dates should be recent and logical (2024-01-01 to 2024-12-31)
- Amounts should be realistic business values (not all zeros or test values)
- Currency codes should match the company scenario
- Status values should be realistic (mix of DRAFT, POSTED, PAID, etc.)

#### **Complete Data**
- All required fields must be present
- Optional fields should be populated where they add value
- Relationships must be valid (foreign keys reference existing entities)
- Calculated fields must be mathematically correct (totals, balances, etc.)

#### **Variety**
- Examples should show different states (DRAFT, POSTED, PAID, CANCELLED)
- Examples should show different scenarios (single currency, multi-currency, with/without discounts, etc.)
- Examples should show edge cases (zero amounts, negative amounts where applicable, etc.)

### 8. Testing Requirements

#### **Validation Checklist**
- [ ] All paths have 3 response examples
- [ ] All POST/PUT operations have 3 request body examples
- [ ] All UUIDs are valid UUID v4 format
- [ ] All foreign key references point to valid example entities
- [ ] All dates are logically consistent
- [ ] All calculated fields are mathematically correct
- [ ] All enum values are valid
- [ ] All required fields are present
- [ ] Examples match schema definitions
- [ ] Examples are properly linked across services

#### **UI Testing**
- [ ] Examples can be used to populate request forms
- [ ] Examples display correctly in Swagger UI
- [ ] Examples can be used for API testing
- [ ] Examples enable complete UI workflows (create invoice → view invoice → create payment → view aging)

### 9. Documentation Requirements

#### **Example Documentation**
Each example should have:
- **Summary**: Brief description of the scenario (e.g., "Acme Corporation - Large enterprise invoice")
- **Value**: Complete example data matching the schema

#### **Cross-Reference Documentation**
Create a mapping document showing:
- Which entities reference which other entities
- Which UUIDs are used for which entities
- How examples link together across services

### 10. Maintenance Requirements

#### **Version Control**
- Examples should be versioned with the OpenAPI spec
- Changes to examples should be tracked in changelog
- Breaking changes to example structure should be documented

#### **Consistency Checks**
- Automated validation should check UUID consistency
- Automated validation should check foreign key validity
- Automated validation should check date consistency
- Automated validation should check calculated field correctness

## Success Criteria

✅ **All 9 accounting services have complete examples**
✅ **All paths have 3 examples per response/request**
✅ **All examples are properly linked across services**
✅ **UI developers can build a complete accounting UI using only examples**
✅ **Examples pass all validation checks**
✅ **Examples are documented and maintainable**

## Deliverables

1. **Updated OpenAPI Specs** (9 files)
   - `openapi/accounting/general-ledger/openapi.yaml`
   - `openapi/accounting/invoice/openapi.yaml`
   - `openapi/accounting/accounts-receivable/openapi.yaml`
   - `openapi/accounting/accounts-payable/openapi.yaml`
   - `openapi/accounting/bank-sync/openapi.yaml`
   - `openapi/accounting/asset/openapi.yaml`
   - `openapi/accounting/budget/openapi.yaml`
   - `openapi/accounting/edi/openapi.yaml`
   - `openapi/accounting/financial-reports/openapi.yaml`

2. **Example Data Mapping Document**
   - `docs/OPENAPI_EXAMPLES_MAPPING.md` - UUID reference guide

3. **Validation Script**
   - `scripts/validate-openapi-examples.py` - Automated validation

## Timeline

- **Week 1**: Core services (General Ledger, Invoice)
- **Week 2**: AR/AP services
- **Week 3**: Supporting services (Bank Sync, Asset)
- **Week 4**: Advanced services (Budget, EDI, Financial Reports)
- **Week 5**: Validation, documentation, and refinement

## References

- [OpenAPI 3.1 Examples Specification](https://spec.openapis.org/oas/v3.1.0#example-object)
- [BRRTRouter Petstore Example](../BRRTRouter/examples/openapi.yaml)
- [BRRTRouter Sample UI](../BRRTRouter/sample-ui/)
- [RERP Accounting OpenAPI Specs](../openapi/accounting/)
