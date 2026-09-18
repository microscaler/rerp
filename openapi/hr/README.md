# Human Resources

## Overview

Complete HR management from employee records to payroll, recruitment, and performance management.

## Services

### Core
- **Path**: `hr/core/`
- **Description**: Core HR service managing employee records and organizational structure
- **Documentation**: [Core README](./core/README.md)
- **API Spec**: [Core OpenAPI](./core/openapi.yaml)

### Attendance
- **Path**: `hr/attendance/`
- **Description**: Attendance tracking service managing clock in/out and work hours
- **Documentation**: [Attendance README](./attendance/README.md)
- **API Spec**: [Attendance OpenAPI](./attendance/openapi.yaml)

### Leave
- **Path**: `hr/leave/`
- **Description**: Leave management service handling leave requests and approval workflows
- **Documentation**: [Leave README](./leave/README.md)
- **API Spec**: [Leave OpenAPI](./leave/openapi.yaml)

### Payroll
- **Path**: `hr/payroll/`
- **Description**: Payroll service processing salary calculations and payslip generation
- **Documentation**: [Payroll README](./payroll/README.md)
- **API Spec**: [Payroll OpenAPI](./payroll/openapi.yaml)

### Recruitment
- **Path**: `hr/recruitment/`
- **Description**: Recruitment service managing job postings and applicant tracking
- **Documentation**: [Recruitment README](./recruitment/README.md)
- **API Spec**: [Recruitment OpenAPI](./recruitment/openapi.yaml)

### Appraisal
- **Path**: `hr/appraisal/`
- **Description**: Performance appraisal service managing review cycles and goal setting
- **Documentation**: [Appraisal README](./appraisal/README.md)
- **API Spec**: [Appraisal OpenAPI](./appraisal/openapi.yaml)

### Skills
- **Path**: `hr/skills/`
- **Description**: Skills management service tracking employee skills and skill gaps
- **Documentation**: [Skills README](./skills/README.md)
- **API Spec**: [Skills OpenAPI](./skills/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/hr` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The human resources services work together to provide complete functionality:

*Integration patterns specific to human resources will be documented as services are implemented.*
