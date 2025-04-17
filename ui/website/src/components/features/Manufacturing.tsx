import { Component } from 'solid-js';
import FeatureLayout from './FeatureLayout';

const Manufacturing: Component = () => {
  return (
    <FeatureLayout
      title="Manufacturing"
      description="Production planning, BOM management, and manufacturing operations"
      icon="fa-industry"
      iconColor="bg-purple-600"
      services=[
              "Manufacturing Core - Production orders, work centers, production tracking",
              "BOM Management - Bill of materials, BOM costing, multi-level BOMs",
              "Production Planning - Scheduling, capacity planning, resource allocation",
              "Repair Management - Repair orders, tracking, warranty management",
              "Subcontracting - Subcontractor management, quality control"
]
      useCases=[
              "Discrete manufacturing",
              "Make-to-order production",
              "Production scheduling and optimization",
              "Quality control and repair management",
              "Subcontractor coordination"
]
    />
  );
};

export default Manufacturing;
