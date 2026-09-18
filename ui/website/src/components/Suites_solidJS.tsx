import { Component, For } from 'solid-js';

// SVG Icons (Heroicons style)
const InfrastructureIcon = () => (
  <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12 text-white">
    <path stroke-linecap="round" stroke-linejoin="round" d="M13.19 8.688a4.5 4.5 0 011.242 7.244l-4.5 4.5a4.5 4.5 0 01-6.364-6.364l1.757-1.757m13.35-.622l1.757-1.757a4.5 4.5 0 00-6.364-6.364l-4.5 4.5a4.5 4.5 0 001.242 7.244" />
  </svg>
);

const AuthIcon = () => (
  <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12 text-white">
    <path stroke-linecap="round" stroke-linejoin="round" d="M16.5 10.5V6.75a4.5 4.5 0 10-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 002.25-2.25v-6.75a2.25 2.25 0 00-2.25-2.25H6.75a2.25 2.25 0 00-2.25 2.25v6.75a2.25 2.25 0 002.25 2.25z" />
  </svg>
);

const LocalizationIcon = () => (
  <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12 text-white">
    <path stroke-linecap="round" stroke-linejoin="round" d="M12 21a9.004 9.004 0 008.716-6.747M12 21a9.004 9.004 0 01-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 017.843 4.582M12 3a8.997 8.997 0 00-7.843 4.582m15.686 0A11.953 11.953 0 0112 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0121 12c0 .778-.099 1.533-.284 2.253m-2.873 0A8.959 8.959 0 0012 12c0-.778.099-1.533.284-2.253M4.157 7.082A8.959 8.959 0 003 12c0 .778.099 1.533.284 2.253M4.157 7.082L3 4.75m1.157 2.332L6.75 4.75m-3.593 7.503L3 19.25m1.157-2.332L6.75 19.25" />
  </svg>
);

const AccountingIcon = () => (
  <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12 text-white">
    <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 18.75a60.07 60.07 0 0115.797 2.101c.727.198 1.453-.342 1.453-1.096V18.75M3.75 4.5v.75A.75.75 0 013 6h-.75m0 0v-.375c0-.621.504-1.125 1.125-1.125H20.25M2.25 6v9m18-10.5v.75c0 .414.336.75.75.75h.75m-1.5-1.5h.375c.621 0 1.125.504 1.125 1.125v9.75c0 .621-.504 1.125-1.125 1.125h-.375m1.5-1.5H21a.75.75 0 00-.75.75v.75m0 0H3.75m0 0h-.375a1.125 1.125 0 01-1.125-1.125V15m1.5 1.5v-.75A.75.75 0 003 15h-.75M15 10.5a3 3 0 11-6 0 3 3 0 016 0zm3 0h1.125A2.25 2.25 0 0119.5 12.75v-1.5a2.25 2.25 0 00-2.25-2.25H15M9 10.5h.375c.621 0 1.125.504 1.125 1.125v9.75c0 .621-.504 1.125-1.125 1.125H9.375a1.125 1.125 0 01-1.125-1.125v-9.75C8.25 11.004 8.754 10.5 9.375 10.5H9z" />
  </svg>
);

const ProductIcon = () => (
  <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12 text-white">
    <path stroke-linecap="round" stroke-linejoin="round" d="M20.25 7.5l-.625 10.632a2.25 2.25 0 01-2.247 2.118H6.622a2.25 2.25 0 01-2.247-2.118L3.75 7.5M10 11.25h4M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125z" />
  </svg>
);

const PurchaseIcon = () => (
  <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12 text-white">
    <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 18.75a60.07 60.07 0 0115.797 2.101c.727.198 1.453-.342 1.453-1.096V18.75M3.75 4.5v.75A.75.75 0 013 6h-.75m0 0v-.375c0-.621.504-1.125 1.125-1.125H20.25M2.25 6v9m18-10.5v.75c0 .414.336.75.75.75h.75m-1.5-1.5h.375c.621 0 1.125.504 1.125 1.125v9.75c0 .621-.504 1.125-1.125 1.125h-.375m1.5-1.5H21a.75.75 0 00-.75.75v.75m0 0H3.75m0 0h-.375a1.125 1.125 0 01-1.125-1.125V15m1.5 1.5v-.75A.75.75 0 003 15h-.75M15 10.5a3 3 0 11-6 0 3 3 0 016 0zm3 0h1.125A2.25 2.25 0 0119.5 12.75v-1.5a2.25 2.25 0 00-2.25-2.25H15M9 10.5h.375c.621 0 1.125.504 1.125 1.125v9.75c0 .621-.504 1.125-1.125 1.125H9.375a1.125 1.125 0 01-1.125-1.125v-9.75C8.25 11.004 8.754 10.5 9.375 10.5H9z" />
  </svg>
);

const CRMIcon = () => (
  <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12 text-white">
    <path stroke-linecap="round" stroke-linejoin="round" d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-1.135A2.25 2.25 0 0021 16.571v-4.698a2.25 2.25 0 00-.9-1.8l-4.5-3a2.25 2.25 0 00-2.4 0l-4.5 3a2.25 2.25 0 00-.9 1.8v4.698a2.25 2.25 0 002.25 2.25h.384a9.279 9.279 0 001.536-.372M15 19.128v-4.698c0-1.194-.22-2.33-.6-3.372m0 0l-3-4m3 4l3-4M12 19.128v-4.698m0 0c-1.194 0-2.33-.22-3.372-.6m3.372.6c1.194 0 2.33.22 3.372.6" />
  </svg>
);

const SalesIcon = () => (
  <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12 text-white">
    <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 18.75a60.07 60.07 0 0115.797 2.101c.727.198 1.453-.342 1.453-1.096V18.75M3.75 4.5v.75A.75.75 0 013 6h-.75m0 0v-.375c0-.621.504-1.125 1.125-1.125H20.25M2.25 6v9m18-10.5v.75c0 .414.336.75.75.75h.75m-1.5-1.5h.375c.621 0 1.125.504 1.125 1.125v9.75c0 .621-.504 1.125-1.125 1.125h-.375m1.5-1.5H21a.75.75 0 00-.75.75v.75m0 0H3.75m0 0h-.375a1.125 1.125 0 01-1.125-1.125V15m1.5 1.5v-.75A.75.75 0 003 15h-.75M15 10.5a3 3 0 11-6 0 3 3 0 016 0zm3 0h1.125A2.25 2.25 0 0119.5 12.75v-1.5a2.25 2.25 0 00-2.25-2.25H15M9 10.5h.375c.621 0 1.125.504 1.125 1.125v9.75c0 .621-.504 1.125-1.125 1.125H9.375a1.125 1.125 0 01-1.125-1.125v-9.75C8.25 11.004 8.754 10.5 9.375 10.5H9z" />
  </svg>
);

const MarketingIcon = () => (
  <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12 text-white">
    <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 1.5H8.25A2.25 2.25 0 006 3.75v16.5a2.25 2.25 0 002.25 2.25h7.5A2.25 2.25 0 0018 20.25V3.75a2.25 2.25 0 00-2.25-2.25H13.5m-3 0V3h3V1.5m-3 0h3m-3 18.75h3" />
  </svg>
);

const InventoryIcon = () => (
  <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12 text-white">
    <path stroke-linecap="round" stroke-linejoin="round" d="M20.25 7.5l-.625 10.632a2.25 2.25 0 01-2.247 2.118H6.622a2.25 2.25 0 01-2.247-2.118L3.75 7.5M10 11.25h4M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125z" />
  </svg>
);

const HRIcon = () => (
  <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12 text-white">
    <path stroke-linecap="round" stroke-linejoin="round" d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-1.135A2.25 2.25 0 0021 16.571v-4.698a2.25 2.25 0 00-.9-1.8l-4.5-3a2.25 2.25 0 00-2.4 0l-4.5 3a2.25 2.25 0 00-.9 1.8v4.698a2.25 2.25 0 002.25 2.25h.384a9.279 9.279 0 001.536-.372M15 19.128v-4.698c0-1.194-.22-2.33-.6-3.372m0 0l-3-4m3 4l3-4M12 19.128v-4.698m0 0c-1.194 0-2.33-.22-3.372-.6m3.372.6c1.194 0 2.33.22 3.372.6" />
  </svg>
);

const ManufacturingIcon = () => (
  <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12 text-white">
    <path stroke-linecap="round" stroke-linejoin="round" d="M11.42 15.17L17.25 21A2.652 2.652 0 0021 17.25l-5.877-5.877M11.42 15.17l2.496-3.03c.317-.384.74-.626 1.208-.766M11.42 15.17l-4.655-5.653a2.548 2.548 0 00-3.586-3.586l-1.257 1.258m12.623-4.874a2.548 2.548 0 013.586 3.586l-1.257 1.258m0 0L9.497 9.497m7.5-7.5L15.75 4.5m-7.5 7.5l-3.75 3.75" />
  </svg>
);

const WebsiteIcon = () => (
  <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12 text-white">
    <path stroke-linecap="round" stroke-linejoin="round" d="M12 21a9.004 9.004 0 008.716-6.747M12 21a9.004 9.004 0 01-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 017.843 4.582M12 3a8.997 8.997 0 00-7.843 4.582m15.686 0A11.953 11.953 0 0112 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0121 12c0 .778-.099 1.533-.284 2.253m-2.873 0A8.959 8.959 0 0012 12c0-.778.099-1.533.284-2.253M4.157 7.082A8.959 8.959 0 003 12c0 .778.099 1.533.284 2.253M4.157 7.082L3 4.75m1.157 2.332L6.75 4.75m-3.593 7.503L3 19.25m1.157-2.332L6.75 19.25" />
  </svg>
);

const POSIcon = () => (
  <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12 text-white">
    <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 8.25h19.5M2.25 9h19.5m-16.5 5.25h6m-6 2.25h3m-3.75 3h15a2.25 2.25 0 002.25-2.25V6.75A2.25 2.25 0 0019.5 4.5h-15a2.25 2.25 0 00-2.25 2.25v12.75A2.25 2.25 0 004.5 21.75z" />
  </svg>
);

const HelpdeskIcon = () => (
  <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12 text-white">
    <path stroke-linecap="round" stroke-linejoin="round" d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9 5.25h.008v.008H12v-.008z" />
  </svg>
);

const AnalyticsIcon = () => (
  <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12 text-white">
    <path stroke-linecap="round" stroke-linejoin="round" d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z" />
  </svg>
);

const AIIcon = () => (
  <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12 text-white">
    <path stroke-linecap="round" stroke-linejoin="round" d="M9.813 15.904L9 18.75l-.813-2.846a4.5 4.5 0 00-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 003.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 003.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 00-3.09 3.09zM18.259 8.715L18 9.75l-.259-1.035a3.375 3.375 0 00-2.455-2.456L14.25 6l1.036-.259a3.375 3.375 0 002.455-2.456L18 2.25l.259 1.035a3.375 3.375 0 002.456 2.456L21.75 6l-1.035.259a3.375 3.375 0 00-2.456 2.456zM16.894 20.567L16.5 21.75l-.394-1.183a2.25 2.25 0 00-1.423-1.423L13.5 18.75l1.183-.394a2.25 2.25 0 001.423-1.423l.394-1.183.394 1.183a2.25 2.25 0 001.423 1.423l1.183.394-1.183.394a2.25 2.25 0 00-1.423 1.423z" />
  </svg>
);

const AutomationIcon = () => (
  <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12 text-white">
    <path stroke-linecap="round" stroke-linejoin="round" d="M9.75 3.104v5.714a2.25 2.25 0 01-.659 1.591L5 14.5M9.75 3.104c-.251.023-.501.05-.75.082m.75-.082a24.301 24.301 0 014.5 0m0 0v5.714c0 .597.237 1.17.659 1.591L19.8 15.3M14.25 3.104c.251.023.501.05.75.082M19.8 15.3l-1.57.393A9.065 9.065 0 0112 15a9.065 9.065 0 00-6.23-.693L5 14.5m14.8.8l1.402 1.402c1.232 1.232 1.232 3.228 0 4.46s-3.228 1.232-4.46 0l-1.403-1.402m-4.25-4.25l-1.403-1.402c-1.232-1.232-1.232-3.228 0-4.46s3.228-1.232 4.46 0l1.403 1.402" />
  </svg>
);

const ProjectIcon = () => (
  <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12 text-white">
    <path stroke-linecap="round" stroke-linejoin="round" d="M6 6.878V6a2.25 2.25 0 012.25-2.25h7.5A2.25 2.25 0 0118 6v.878m-12 0c.235-.083.487-.128.75-.128h10.5c.263 0 .515.045.75.128m-12 0a2.25 2.25 0 00-1.5 2.122v6.75a2.25 2.25 0 002.25 2.25h7.5a2.25 2.25 0 002.25-2.25V9a2.25 2.25 0 00-1.5-2.122M8.25 19.5v-7.5m0 0l-3-3m3 3l3-3m-3 3h7.5" />
  </svg>
);

const MarketplaceIcon = () => (
  <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12 text-white">
    <path stroke-linecap="round" stroke-linejoin="round" d="M13.5 21v-7.5a.75.75 0 01.75-.75h3a.75.75 0 01.75.75V21m-4.5 0H2.36m11.14 0H18m0 0h3.64m-1.39 0V9.349m-16.5 11.65V9.35m0 0a3.001 3.001 0 003.75-.615A2.993 2.993 0 009.75 9.75c.896 0 1.7-.393 2.25-1.016a2.993 2.993 0 002.25 1.016c.896 0 1.7-.393 2.25-1.016a3.001 3.001 0 003.75.614m-16.5 0a3.004 3.004 0 01-.621-4.72L4.318 3.44A1.5 1.5 0 015.378 3h13.243a1.5 1.5 0 011.06.44l1.19 1.189a3 3 0 01-.621 4.72m-13.5 8.65h3.75a.75.75 0 00.75-.75V13.5a.75.75 0 00-.75-.75H6.75a.75.75 0 00-.75.75v3.75c0 .415.336.75.75.75z" />
  </svg>
);

const FieldServiceIcon = () => (
  <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12 text-white">
    <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 18.75a1.5 1.5 0 01-3 0m3 0a1.5 1.5 0 00-3 0m3 0h6m-9 0H3.375a1.125 1.125 0 01-1.125-1.125V14.25m17.25 4.5a1.5 1.5 0 01-3 0m3 0a1.5 1.5 0 00-3 0m3 0h1.125c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H19.5m-4.5-1.5a1.5 1.5 0 01-3 0m3 0a1.5 1.5 0 00-3 0m3 0h.375c.621 0 1.125.504 1.125 1.125v1.5c0 .621-.504 1.125-1.125 1.125h-.375M15 10.5a3 3 0 11-6 0 3 3 0 016 0zm6 3a2.25 2.25 0 11-4.5 0 2.25 2.25 0 014.5 0zm-13.5 0a2.25 2.25 0 11-4.5 0 2.25 2.25 0 014.5 0z" />
  </svg>
);

interface Suite {
  name: string;
  category: string;
  description: string;
  icon: Component;
}

const suites: Suite[] = [
  // Foundation Suites (3)
  {
    name: 'Infrastructure',
    category: 'Foundation',
    description: 'API Gateway and Integration Platform - the digital foundation that connects everything',
    icon: InfrastructureIcon,
  },
  {
    name: 'Auth',
    category: 'Foundation',
    description: 'Identity and Access Management with enterprise-grade security',
    icon: AuthIcon,
  },
  {
    name: 'Localization',
    category: 'Foundation',
    description: 'Multi-language, multi-currency, and regional compliance infrastructure',
    icon: LocalizationIcon,
  },
  // Core Business Suites (3)
  {
    name: 'Accounting',
    category: 'Core Business',
    description: 'Complete financial management - General Ledger, AP/AR, reporting, and more',
    icon: AccountingIcon,
  },
  {
    name: 'Product',
    category: 'Core Business',
    description: 'Product catalog, dynamic pricing, and tax calculation',
    icon: ProductIcon,
  },
  {
    name: 'Purchase',
    category: 'Core Business',
    description: 'Purchase orders, vendor management, and procurement workflows',
    icon: PurchaseIcon,
  },
  // Sales & Customer Suites (3)
  {
    name: 'CRM',
    category: 'Sales & Customer',
    description: 'Lead management, automation, and live chat for customer relationships',
    icon: CRMIcon,
  },
  {
    name: 'Sales',
    category: 'Sales & Customer',
    description: 'Quotations, orders, subscriptions, and loyalty programs',
    icon: SalesIcon,
  },
  {
    name: 'Marketing',
    category: 'Sales & Customer',
    description: 'Email campaigns, automation, and social media management',
    icon: MarketingIcon,
  },
  // Operations Suites (3)
  {
    name: 'Inventory',
    category: 'Operations',
    description: 'Stock management, warehouse operations, and logistics',
    icon: InventoryIcon,
  },
  {
    name: 'HR',
    category: 'Operations',
    description: 'Employee records, payroll, recruitment, and performance management',
    icon: HRIcon,
  },
  {
    name: 'Manufacturing',
    category: 'Operations',
    description: 'Production planning, BOM management, and quality control',
    icon: ManufacturingIcon,
  },
  // Digital Presence Suites (3)
  {
    name: 'Website',
    category: 'Digital Presence',
    description: 'Website builder, e-commerce platform, and CMS',
    icon: WebsiteIcon,
  },
  {
    name: 'POS',
    category: 'Digital Presence',
    description: 'Point of Sale with payment integration and offline support',
    icon: POSIcon,
  },
  {
    name: 'Helpdesk',
    category: 'Digital Presence',
    description: 'Ticket management, knowledge base, and customer support workflows',
    icon: HelpdeskIcon,
  },
  // Intelligence & Extensions Suites (3)
  {
    name: 'Analytics',
    category: 'Intelligence',
    description: 'Dashboards, reporting engine, and Business Intelligence',
    icon: AnalyticsIcon,
  },
  {
    name: 'AI',
    category: 'Intelligence',
    description: 'Intelligent document processing and predictive analytics',
    icon: AIIcon,
  },
  {
    name: 'Automation',
    category: 'Intelligence',
    description: 'Workflow automation, process automation, and integration automation',
    icon: AutomationIcon,
  },
  // Platform Suites (3)
  {
    name: 'Project',
    category: 'Platform',
    description: 'Project management, timesheet tracking, and resource allocation',
    icon: ProjectIcon,
  },
  {
    name: 'Marketplace',
    category: 'Platform',
    description: 'App marketplace, integration hub, and third-party extension management',
    icon: MarketplaceIcon,
  },
  {
    name: 'Field Service',
    category: 'Platform',
    description: 'Field service management, scheduling, dispatch, and mobile operations',
    icon: FieldServiceIcon,
  },
];

const Suites: Component = () => {
  const foundationSuites = () => suites.filter((s) => s.category === 'Foundation');
  const coreBusinessSuites = () => suites.filter((s) => s.category === 'Core Business');
  const salesCustomerSuites = () => suites.filter((s) => s.category === 'Sales & Customer');
  const operationsSuites = () => suites.filter((s) => s.category === 'Operations');
  const digitalPresenceSuites = () => suites.filter((s) => s.category === 'Digital Presence');
  const intelligenceSuites = () => suites.filter((s) => s.category === 'Intelligence');
  const platformSuites = () => suites.filter((s) => s.category === 'Platform');

  const SuiteCard: Component<{ suite: Suite }> = (props) => (
    <div class="group/tier flex gap-6 rounded-3xl bg-gray-800/50 p-8 ring-1 ring-white/15 xl:p-10">
      <div class="flex size-20 shrink-0 items-center justify-center rounded-lg bg-indigo-500">
        <props.suite.icon />
      </div>
      <div class="flex flex-col">
        <h3 class="text-lg/8 font-semibold text-white">{props.suite.name}</h3>
        <p class="mt-2 text-sm/6 text-gray-300">{props.suite.description}</p>
      </div>
    </div>
  );

  return (
    <section id="suites" class="bg-gray-900 px-6 py-24 sm:py-32 lg:px-8">
      <div class="mx-auto max-w-7xl">
        <div class="mx-auto max-w-2xl lg:text-center">
          <h2 class="text-base font-semibold leading-7 text-indigo-400">RERP Suites</h2>
          <p class="mt-2 text-3xl font-bold tracking-tight text-white sm:text-4xl">
            Modular Suites for Complete Business Operations
          </p>
          <p class="mt-6 text-lg leading-8 text-gray-300">
            RERP is organized into suites—complete business functions that work together seamlessly. Start with what you need, add more suites as your business grows.
          </p>
        </div>

        <div class="mx-auto mt-16 max-w-2xl sm:mt-20 lg:mt-24 lg:max-w-none">
          <h3 class="text-xl font-semibold text-white mb-8">Foundation Suites</h3>
          <div class="grid max-w-xl grid-cols-1 gap-x-8 gap-y-16 lg:max-w-none lg:grid-cols-3 mb-16">
            <For each={foundationSuites()}>
              {(suite) => <SuiteCard suite={suite} />}
            </For>
          </div>

          <h3 class="text-xl font-semibold text-white mb-8">Core Business Suites</h3>
          <div class="grid max-w-xl grid-cols-1 gap-x-8 gap-y-16 lg:max-w-none lg:grid-cols-3 mb-16">
            <For each={coreBusinessSuites()}>
              {(suite) => <SuiteCard suite={suite} />}
            </For>
          </div>

          <h3 class="text-xl font-semibold text-white mb-8">Sales & Customer Suites</h3>
          <div class="grid max-w-xl grid-cols-1 gap-x-8 gap-y-16 lg:max-w-none lg:grid-cols-3 mb-16">
            <For each={salesCustomerSuites()}>
              {(suite) => <SuiteCard suite={suite} />}
            </For>
          </div>

          <h3 class="text-xl font-semibold text-white mb-8">Operations Suites</h3>
          <div class="grid max-w-xl grid-cols-1 gap-x-8 gap-y-16 lg:max-w-none lg:grid-cols-3 mb-16">
            <For each={operationsSuites()}>
              {(suite) => <SuiteCard suite={suite} />}
            </For>
          </div>

          <h3 class="text-xl font-semibold text-white mb-8">Digital Presence Suites</h3>
          <div class="grid max-w-xl grid-cols-1 gap-x-8 gap-y-16 lg:max-w-none lg:grid-cols-3 mb-16">
            <For each={digitalPresenceSuites()}>
              {(suite) => <SuiteCard suite={suite} />}
            </For>
          </div>

          <h3 class="text-xl font-semibold text-white mb-8">Intelligence & Extensions Suites</h3>
          <div class="grid max-w-xl grid-cols-1 gap-x-8 gap-y-16 lg:max-w-none lg:grid-cols-3 mb-16">
            <For each={intelligenceSuites()}>
              {(suite) => <SuiteCard suite={suite} />}
            </For>
          </div>

          <h3 class="text-xl font-semibold text-white mb-8">Platform Suites</h3>
          <div class="grid max-w-xl grid-cols-1 gap-x-8 gap-y-16 lg:max-w-none lg:grid-cols-3">
            <For each={platformSuites()}>
              {(suite) => <SuiteCard suite={suite} />}
            </For>
          </div>
        </div>
      </div>
    </section>
  );
};

export default Suites;
