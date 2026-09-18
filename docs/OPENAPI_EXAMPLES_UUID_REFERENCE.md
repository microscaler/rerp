# OpenAPI Examples UUID Reference

This document tracks all UUIDs used in OpenAPI examples to ensure consistency and proper linking across services.

## Company IDs

- **Acme Corporation**: `550e8400-e29b-41d4-a716-446655440000`
- **TechStart Inc**: `660e8400-e29b-41d4-a716-446655440001`
- **Global Manufacturing Ltd**: `770e8400-e29b-41d4-a716-446655440002`

## General Ledger

### Chart of Accounts
- **Acme - Assets Root**: `a00c0e8400-e29b-41d4-a716-446655440000`
- **Acme - Liabilities Root**: `a00c1e8400-e29b-41d4-a716-446655440001`
- **Acme - Equity Root**: `a00c2e8400-e29b-41d4-a716-446655440002`
- **TechStart - Assets Root**: `b00c0e8400-e29b-41d4-a716-446655440000`
- **TechStart - Liabilities Root**: `b00c1e8400-e29b-41d4-a716-446655440001`
- **Global - Assets Root**: `c00c0e8400-e29b-41d4-a716-446655440000`
- **Global - Multi-Currency Root**: `c00c3e8400-e29b-41d4-a716-446655440003`

### Accounts
- **Acme - Cash Account**: `a0070e8400-e29b-41d4-a716-446655440000`
- **Acme - AR Account**: `a0071e8400-e29b-41d4-a716-446655440001`
- **Acme - Revenue Account**: `a0072e8400-e29b-41d4-a716-446655440002`
- **TechStart - Cash Account**: `b0070e8400-e29b-41d4-a716-446655440000`
- **TechStart - AR Account**: `b0071e8400-e29b-41d4-a716-446655440001`
- **Global - Cash USD**: `c0070e8400-e29b-41d4-a716-446655440000`
- **Global - Cash EUR**: `c0071e8400-e29b-41d4-a716-446655440001`

### Journal Entries
- **Acme - Manual Entry**: `a0080e8400-e29b-41d4-a716-446655440000`
- **Acme - Invoice Entry**: `a0081e8400-e29b-41d4-a716-446655440001`
- **TechStart - Manual Entry**: `b0080e8400-e29b-41d4-a716-446655440000`
- **Global - Multi-Currency Entry**: `c0080e8400-e29b-41d4-a716-446655440000`

## Invoice Service

### Invoices
- **Acme - Customer Invoice**: `a0010e8400-e29b-41d4-a716-446655440000`
- **Acme - Vendor Bill**: `a0011e8400-e29b-41d4-a716-446655440001`
- **TechStart - Customer Invoice**: `b0010e8400-e29b-41d4-a716-446655440000`
- **Global - Multi-Currency Invoice**: `c0010e8400-e29b-41d4-a716-446655440000`

### Invoice Lines
- **Acme Invoice Line 1**: `a0020e8400-e29b-41d4-a716-446655440000`
- **Acme Invoice Line 2**: `a0021e8400-e29b-41d4-a716-446655440001`
- **TechStart Invoice Line 1**: `b0020e8400-e29b-41d4-a716-446655440000`

## Accounts Receivable

### Customers
- **Acme Customer 1**: `111e8400-e29b-41d4-a716-446655440001`
- **Acme Customer 2**: `111e8400-e29b-41d4-a716-446655440002`
- **TechStart Customer 1**: `222e8400-e29b-41d4-a716-446655440001`
- **Global Customer EUR**: `333e8400-e29b-41d4-a716-446655440001`

### Customer Invoices
- **Acme AR Invoice 1**: `a0030e8400-e29b-41d4-a716-446655440000`
- **TechStart AR Invoice 1**: `b0030e8400-e29b-41d4-a716-446655440000`

### AR Payments
- **Acme AR Payment 1**: `a0040e8400-e29b-41d4-a716-446655440000`
- **TechStart AR Payment 1**: `b0040e8400-e29b-41d4-a716-446655440000`

## Accounts Payable

### Vendors
- **Acme Vendor 1**: `411e8400-e29b-41d4-a716-446655440001`
- **TechStart Vendor 1**: `422e8400-e29b-41d4-a716-446655440001`
- **Global Vendor EUR**: `433e8400-e29b-41d4-a716-446655440001`

### Vendor Invoices
- **Acme AP Invoice 1**: `a0050e8400-e29b-41d4-a716-446655440000`
- **TechStart AP Invoice 1**: `b0050e8400-e29b-41d4-a716-446655440000`

### AP Payments
- **Acme AP Payment 1**: `a0060e8400-e29b-41d4-a716-446655440000`
- **TechStart AP Payment 1**: `b0060e8400-e29b-41d4-a716-446655440000`

## UUID Naming Convention

Format: `{company}{entity_type}{sequence}`

- **Company prefix**: `a00` = Acme, `b00` = TechStart, `c00` = Global
- **Entity type**: `1e` = Invoice, `2e` = InvoiceLine, `3e` = CustomerInvoice, `7e` = Account, `8e` = JournalEntry, `c0` = ChartOfAccount
- **Sequence**: Incremental number for multiple instances
