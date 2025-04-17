import { Component } from 'solid-js';
import FeatureLayout from './FeatureLayout';

const AnalyticsBI: Component = () => {
  return (
    <FeatureLayout
      title="Analytics & BI"
      description="Business intelligence, dashboards, and comprehensive analytics"
      icon="fa-chart-bar"
      iconColor="bg-green-600"
      services=[
              "Business Intelligence - Data warehousing, OLAP cubes, advanced analytics",
              "Dashboards - Custom dashboards, widget library, real-time updates",
              "Reporting - Report builder, scheduled reports, distribution"
]
      useCases=[
              "Business intelligence and analytics",
              "Custom reporting and dashboards",
              "Data-driven decision making",
              "Performance monitoring and KPIs",
              "Cross-module data analysis"
]
    />
  );
};

export default AnalyticsBI;
