import { Component } from 'solid-js';
import { EXTERNAL_URLS } from '../config/constants';
import {
  FooterBrand,
  FooterLinkSection,
  FooterCopyright,
  FooterLinkProps,
} from './components';

const Footer: Component = () => {
  const footerLinks: Record<string, FooterLinkProps[]> = {
    product: [
      { text: 'Features', href: '#features' },
      { text: 'Pricing', href: '#pricing' },
      { text: 'How It Works', href: '#how-it-works' },
      { text: 'Documentation', href: '#' },
    ],
    company: [
      { text: 'About', href: '#about' },
      { text: 'Blog', href: '#blogs' },
      { text: 'Careers', href: '#careers' },
      { text: 'Contact', href: '#contact' },
    ],
    support: [
      { text: 'FAQ', href: '#faq' },
      { text: 'Help Center', href: '#' },
      { text: 'Status', href: '#status' },
      { text: 'Security', href: '#trust-security' },
    ],
    resources: [
      { text: 'Trading Education', href: EXTERNAL_URLS.tradingEducation, external: true },
      { text: 'Blog', href: '#blogs' },
      { text: 'Case Studies', href: '#case-studies' },
      { text: 'Testimonials', href: '#testimonials' },
    ],
    legal: [
      { text: 'Terms of Service', href: '#terms-of-service' },
      { text: 'Privacy Policy', href: '#privacy-policy' },
      { text: 'Refund Policy', href: '#refund-policy' },
    ],
  };

  const socialLinks = [
    { icon: 'fa-brands fa-twitter', href: '#', label: 'Twitter' },
    { icon: 'fa-brands fa-linkedin', href: '#', label: 'LinkedIn' },
    { icon: 'fa-brands fa-github', href: '#', label: 'GitHub' },
  ];

  return (
    <footer id="footer" class="bg-gray-900 border-t border-gray-800">
      <div class="max-w-7xl mx-auto px-6 lg:px-8 py-12">
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-7 gap-8">
          <div class="sm:col-span-2 lg:col-span-2">
            <FooterBrand
              name="PriceWhisperer"
              description="Smart stock market intelligence for modern traders and investors."
              socialLinks={socialLinks}
            />
          </div>
          <div class="lg:col-span-1">
            <FooterLinkSection title="Product" links={footerLinks.product} />
          </div>
          <div class="lg:col-span-1">
            <FooterLinkSection title="Company" links={footerLinks.company} />
          </div>
          <div class="lg:col-span-1">
            <FooterLinkSection title="Support" links={footerLinks.support} />
          </div>
          <div class="lg:col-span-1">
            <FooterLinkSection title="Resources" links={footerLinks.resources} />
          </div>
          <div class="lg:col-span-1">
            <FooterLinkSection title="Legal" links={footerLinks.legal} />
          </div>
        </div>
        <FooterCopyright companyName="PriceWhisperer" />
      </div>
    </footer>
  );
};

export default Footer;

