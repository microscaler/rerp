import { Component } from 'solid-js';
import FeatureLayout from './FeatureLayout';

const InventoryLogistics: Component = () => {
  return (
    <FeatureLayout
      title="Inventory & Logistics"
      description="Comprehensive inventory management with warehouse operations and logistics"
      icon="fa-warehouse"
      iconColor="bg-accent"
      services=[
              "Inventory Core - Stock management, movements, valuation",
              "Warehouse Operations - Location management, picking, packing",
              "Logistics Management - Shipping, carrier integration, delivery tracking",
              "Dropshipping - Order management, supplier integration, inventory sync"
]
      useCases=[
              "Multi-warehouse inventory management",
              "Supply chain optimization",
              "E-commerce fulfillment",
              "Dropshipping business models",
              "Logistics coordination"
]
    />
  );
};

export default InventoryLogistics;
