export interface FAQItem {
  id: string;
  question: string;
  answer: string;
}

export const faqData: FAQItem[] = [
  {
    "id": "what-is-rerp",
    "question": "What is RERP?",
    "answer": "RERP (Rust Enterprise Resource Planning) is a comprehensive, modular ERP system with 71 integrated services designed for businesses of all sizes. It's built with a cloud-native, microservices architecture that allows you to deploy only the modules you need and scale independently."
  },
  {
    "id": "how-differs",
    "question": "How does RERP differ from other ERPs?",
    "answer": "RERP differs from traditional ERPs in several key ways: (1) Modular architecture - 71 independent services vs monolithic codebase, (2) Open-source - no vendor lock-in, full source code access, (3) Cloud-native - designed for modern infrastructure from the ground up, (4) API-first - OpenAPI specifications enable easy integrations, (5) Independent deployment - each service can be deployed, scaled, and updated separately."
  },
  {
    "id": "modules-available",
    "question": "What modules are available?",
    "answer": "RERP offers 71 services organized into 6 implementation phases: Phase 1 (Core Foundation - 7 services), Phase 2 (Business Operations - 14 services), Phase 3 (Financial & HR - 16 services), Phase 4 (Advanced Operations - 7 services), Phase 5 (Customer-Facing - 10 services), and Phase 6 (Extensions & Analytics - 5 services). Plus additional specialized services for AI, automation, IoT, and more."
  },
  {
    "id": "get-started",
    "question": "How do I get started?",
    "answer": "Getting started with RERP is straightforward: (1) Visit our GitHub repository to explore the codebase, (2) Review the documentation for installation and setup instructions, (3) Choose which modules you need from the 6 implementation phases, (4) Deploy on your infrastructure or use cloud services, (5) Join the community for support and contributions."
  },
  {
    "id": "really-free",
    "question": "Is it really free?",
    "answer": "Yes, RERP is 100% open-source and free. There are no licensing fees, no per-user costs, and no restrictions on usage. The software is dual-licensed under Apache 2.0 and MIT licenses, allowing commercial use. You have full access to the source code and can deploy on your own infrastructure."
  },
  {
    "id": "self-host",
    "question": "Can I self-host?",
    "answer": "Absolutely! RERP is designed for self-hosting. You have complete control over your deployment, data, and infrastructure. The modular architecture allows you to deploy services on-premise, in the cloud, or in a hybrid configuration. All documentation and deployment guides are available in the repository."
  },
  {
    "id": "support-available",
    "question": "What support is available?",
    "answer": "RERP offers community-driven support through GitHub Discussions, comprehensive documentation, issue tracking, and community forums. For enterprise needs, commercial support options (with SLA guarantees, dedicated account managers, and custom development) are planned for the future."
  },
  {
    "id": "how-contribute",
    "question": "How do I contribute?",
    "answer": "Contributions are welcome! You can contribute by: (1) Reporting bugs or suggesting features via GitHub Issues, (2) Contributing code improvements or new modules, (3) Improving documentation, (4) Participating in discussions and helping other users, (5) Sharing your implementation experiences. Check the CONTRIBUTING.md guide in the repository for details."
  }
];
