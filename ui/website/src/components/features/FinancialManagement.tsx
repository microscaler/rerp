import { Component } from 'solid-js';
import FeatureLayout from './FeatureLayout';

const FinancialManagement: Component = () => {
  return (
    <FeatureLayout
      title="Financial Management"
      description="Complete accounting suite for comprehensive financial management"
      icon="fa-chart-line"
      iconColor="bg-primary"
      services=[
              "General Ledger - Chart of accounts, journal entries, account reconciliation",
              "Accounts Payable - Vendor invoices, payment processing, AP aging",
              "Accounts Receivable - Customer invoices, payment collection, AR aging",
              "Invoice Management - Invoice generation, templates, automated billing",
              "Asset Management - Fixed assets, depreciation, asset tracking",
              "Budget Planning - Budget creation, tracking, variance analysis",
              "Financial Reports - P&L, balance sheets, cash flow statements",
              "Bank Synchronization - Bank reconciliation, transaction import",
              "EDI Processing - Electronic data interchange for business documents"
]
      useCases=[
              "Complete financial accounting and reporting",
              "Automated invoice processing and payment management",
              "Budget planning and financial forecasting",
              "Compliance with accounting standards",
              "Multi-currency financial management"
]
    />
  );
};

export default FinancialManagement;
