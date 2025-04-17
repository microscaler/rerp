import { Component } from 'solid-js';
import FeatureLayout from './FeatureLayout';

const ProjectManagement: Component = () => {
  return (
    <FeatureLayout
      title="Project Management"
      description="Project planning, task tracking, and resource management"
      icon="fa-tasks"
      iconColor="bg-indigo-600"
      services=[
              "Project Core - Project management, task tracking, resource allocation",
              "Timesheet Management - Time tracking, approval, project billing"
]
      useCases=[
              "Service-based project delivery",
              "Time and materials billing",
              "Resource planning and allocation",
              "Project profitability analysis",
              "Client project management"
]
    />
  );
};

export default ProjectManagement;
