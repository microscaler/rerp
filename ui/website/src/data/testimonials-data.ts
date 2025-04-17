export interface Testimonial {
  id: string;
  quote: string;
  author: string;
  role: string;
  company: string;
  painPoint: string;
}

export const testimonialsData: Testimonial[] = [
  {
    "id": "testimonial-1",
    "quote": "We're stuck with a monolithic ERP that takes months to update. Every update is a risk to our entire system. We need something modular where we can update individual components without risking everything.",
    "author": "Sarah Chen",
    "role": "IT Director",
    "company": "Mid-size Manufacturing",
    "painPoint": "Monolithic updates are too risky"
  },
  {
    "id": "testimonial-2",
    "quote": "Our current ERP forces us to pay for modules we don't use. We're a small business but we're paying enterprise prices for features we'll never need. We need a system where we only deploy what we actually use.",
    "author": "Michael Rodriguez",
    "role": "Operations Manager",
    "company": "Small E-commerce Business",
    "painPoint": "Paying for unused modules"
  },
  {
    "id": "testimonial-3",
    "quote": "Integration is a nightmare. Every time we need to connect to a new service, it's weeks of custom development and expensive consulting. We need an ERP with standardized APIs that actually work.",
    "author": "Jennifer Park",
    "role": "Integration Lead",
    "company": "Growing Tech Company",
    "painPoint": "Difficult integrations"
  }
];

