export interface ModuleCategory {
  name: string;
  services: string[];
}

export interface ModulePhase {
  id: string;
  title: string;
  description: string;
  serviceCount: number;
  categories: ModuleCategory[];
}

export interface SelectionGuide {
  id: string;
  title: string;
  phases: string[];
  description: string;
}

export interface ModulesData {
  phases: ModulePhase[];
  additionalServices: {
    title: string;
    description: string;
    services: string[];
  };
  selectionGuides: SelectionGuide[];
}

export const modulesData: ModulesData = {
  "phases": [
    {
      "id": "phase-1",
      "title": "Core Foundation",
      "description": "Essential for any business - user management, product catalogs, and core infrastructure",
      "serviceCount": 7,
      "categories": [
        {
          "name": "Authentication & Authorization",
          "services": [
            "IDAM",
            "RBAC"
          ]
        },
        {
          "name": "Infrastructure",
          "services": [
            "API Gateway",
            "Integration Platform"
          ]
        },
        {
          "name": "Product Management",
          "services": [
            "Catalog",
            "Pricing",
            "Tax"
          ]
        }
      ]
    },
    {
      "id": "phase-2",
      "title": "Business Operations",
      "description": "Complete sales and operations management from lead to delivery",
      "serviceCount": 14,
      "categories": [
        {
          "name": "CRM",
          "services": [
            "Core CRM",
            "Automation",
            "Live Chat"
          ]
        },
        {
          "name": "Sales",
          "services": [
            "Sales Core",
            "Quotation",
            "Order",
            "Subscription",
            "Loyalty"
          ]
        },
        {
          "name": "Purchase",
          "services": [
            "Purchase Core",
            "Vendor"
          ]
        },
        {
          "name": "Inventory",
          "services": [
            "Inventory Core",
            "Warehouse",
            "Logistics",
            "Dropshipping"
          ]
        }
      ]
    },
    {
      "id": "phase-3",
      "title": "Financial & HR Management",
      "description": "Comprehensive financial management and human resources",
      "serviceCount": 16,
      "categories": [
        {
          "name": "Accounting",
          "services": [
            "General Ledger",
            "AP",
            "AR",
            "Invoice",
            "Asset",
            "Budget",
            "Financial Reports",
            "Bank Sync",
            "EDI"
          ]
        },
        {
          "name": "HR",
          "services": [
            "HR Core",
            "Payroll",
            "Recruitment",
            "Attendance",
            "Leave",
            "Appraisal",
            "Skills"
          ]
        }
      ]
    },
    {
      "id": "phase-4",
      "title": "Advanced Operations",
      "description": "For manufacturing and project-based businesses",
      "serviceCount": 7,
      "categories": [
        {
          "name": "Manufacturing",
          "services": [
            "Manufacturing Core",
            "BOM",
            "Production Planning",
            "Repair",
            "Subcontracting"
          ]
        },
        {
          "name": "Project",
          "services": [
            "Project Core",
            "Timesheet"
          ]
        }
      ]
    },
    {
      "id": "phase-5",
      "title": "Customer-Facing Services",
      "description": "Engage customers through marketing, e-commerce, and support",
      "serviceCount": 10,
      "categories": [
        {
          "name": "Marketing",
          "services": [
            "Email",
            "Automation",
            "Social Media"
          ]
        },
        {
          "name": "Website",
          "services": [
            "CMS",
            "Ecommerce",
            "Builder"
          ]
        },
        {
          "name": "POS",
          "services": [
            "POS Core",
            "Payment Gateway"
          ]
        },
        {
          "name": "Helpdesk",
          "services": [
            "Helpdesk Core",
            "Knowledge Base"
          ]
        },
        {
          "name": "Field Service",
          "services": [
            "Field Service Core"
          ]
        }
      ]
    },
    {
      "id": "phase-6",
      "title": "Extensions & Analytics",
      "description": "Extend functionality and gain insights",
      "serviceCount": 5,
      "categories": [
        {
          "name": "Marketplace",
          "services": [
            "Marketplace Core",
            "Integration Hub"
          ]
        },
        {
          "name": "Analytics",
          "services": [
            "BI",
            "Dashboards",
            "Reporting"
          ]
        }
      ]
    }
  ],
  "additionalServices": {
    "title": "Specialized Services",
    "description": "Advanced capabilities for modern enterprises",
    "services": [
      "AI",
      "Automation",
      "Appointments",
      "Approvals",
      "Data",
      "Documents",
      "ESG",
      "IoT",
      "Localization"
    ]
  },
  "selectionGuides": [
    {
      "id": "small-business",
      "title": "Small Business Starter Pack",
      "phases": [
        "phase-1",
        "phase-2"
      ],
      "description": "Essential modules for small businesses getting started"
    },
    {
      "id": "mid-market",
      "title": "Mid-Market Complete Suite",
      "phases": [
        "phase-1",
        "phase-2",
        "phase-3"
      ],
      "description": "Comprehensive solution for growing businesses"
    },
    {
      "id": "enterprise",
      "title": "Enterprise Full Deployment",
      "phases": [
        "phase-1",
        "phase-2",
        "phase-3",
        "phase-4",
        "phase-5",
        "phase-6"
      ],
      "description": "Complete ERP solution for large enterprises"
    }
  ]
};

export const selectionGuides: SelectionGuide[] = [
  {
    "id": "small-business",
    "title": "Small Business Starter Pack",
    "phases": [
      "phase-1",
      "phase-2"
    ],
    "description": "Essential modules for small businesses getting started"
  },
  {
    "id": "mid-market",
    "title": "Mid-Market Complete Suite",
    "phases": [
      "phase-1",
      "phase-2",
      "phase-3"
    ],
    "description": "Comprehensive solution for growing businesses"
  },
  {
    "id": "enterprise",
    "title": "Enterprise Full Deployment",
    "phases": [
      "phase-1",
      "phase-2",
      "phase-3",
      "phase-4",
      "phase-5",
      "phase-6"
    ],
    "description": "Complete ERP solution for large enterprises"
  }
];

export const additionalServices = {
  "title": "Specialized Services",
  "description": "Advanced capabilities for modern enterprises",
  "services": [
    "AI",
    "Automation",
    "Appointments",
    "Approvals",
    "Data",
    "Documents",
    "ESG",
    "IoT",
    "Localization"
  ]
};

