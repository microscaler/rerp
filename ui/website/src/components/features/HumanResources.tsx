import { Component } from 'solid-js';
import FeatureLayout from './FeatureLayout';

const HumanResources: Component = () => {
  return (
    <FeatureLayout
      title="Human Resources"
      description="Complete HR management from recruitment to performance management"
      icon="fa-users"
      iconColor="bg-orange-600"
      services=[
              "HR Core - Employee records, onboarding, offboarding",
              "Payroll Processing - Salary management, tax deductions, payroll reports",
              "Recruitment - Job postings, applicant tracking, interview management",
              "Attendance Management - Time tracking, attendance records, shift management",
              "Leave Management - Leave requests, balance tracking, leave policies",
              "Performance Appraisals - Performance reviews, goal setting, feedback",
              "Skills Management - Skills inventory, assessment, training management"
]
      useCases=[
              "Employee lifecycle management",
              "Payroll and benefits administration",
              "Recruitment and talent acquisition",
              "Performance management and development",
              "Compliance with labor regulations"
]
    />
  );
};

export default HumanResources;
