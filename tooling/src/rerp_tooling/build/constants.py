"""RERP build constants: package names for accounting microservices."""

from __future__ import annotations

# Service (directory) name -> Cargo [package] name (impl crate binary).
# Maintained by bootstrap; used by build, docker copy-artifacts, and discovery.
PACKAGE_NAMES: dict[str, str] = {
    "general-ledger": "rerp_accounting_general_ledger",
    "invoice": "rerp_accounting_invoice",
    "accounts-receivable": "rerp_accounting_accounts_receivable",
    "accounts-payable": "rerp_accounting_accounts_payable",
    "bank-sync": "rerp_accounting_bank_sync",
    "asset": "rerp_accounting_asset",
    "budget": "rerp_accounting_budget",
    "edi": "rerp_accounting_edi",
    "financial-reports": "rerp_accounting_financial_reports",
    "bff": "rerp_accounting_bff",
}
