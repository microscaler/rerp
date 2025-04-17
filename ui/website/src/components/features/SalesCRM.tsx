import { Component } from 'solid-js';
import FeatureLayout from './FeatureLayout';

const SalesCRM: Component = () => {
  return (
    <FeatureLayout
      title="Sales & CRM"
      description="End-to-end sales management from lead generation to order fulfillment"
      icon="fa-handshake"
      iconColor="bg-secondary"
      services=[
              "Core CRM - Lead management, contact management, opportunity tracking",
              "CRM Automation - Workflow automation, email automation, task automation",
              "Live Chat - Real-time customer communication and support",
              "Sales Core - Sales pipeline, forecasting, analytics",
              "Quotation Management - Quote generation, approval workflows",
              "Order Processing - Order management, fulfillment, tracking",
              "Subscription Management - Recurring billing, subscription analytics",
              "Loyalty Programs - Points management, rewards system"
]
      useCases=[
              "Lead to cash management",
              "Customer relationship management",
              "Sales pipeline optimization",
              "Subscription-based business models",
              "Customer loyalty and retention programs"
]
    />
  );
};

export default SalesCRM;
