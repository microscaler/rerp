import { Component } from 'solid-js';
import { FooterBrand, FooterLinkSection, FooterCopyright } from './components';

const GITHUB_URL = 'https://github.com/microscaler/rerp';

const Footer: Component = () => {
  const links = [
    { text: 'About', href: '#about' },
    { text: 'Contact', href: '#contact' },
    { text: 'GitHub', href: GITHUB_URL, external: true },
  ];

  return (
    <footer class="bg-gray-900 border-t border-gray-800">
      <div class="max-w-7xl mx-auto px-6 lg:px-8 py-12">
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-8">
          <FooterBrand
            name="RERP"
            description="Modular, open-source ERP. Enterprise Resource Planning, reimagined."
            socialLinks={[{ icon: 'fa-brands fa-github', href: GITHUB_URL, label: 'GitHub' }]}
          />
          <FooterLinkSection title="Links" links={links} />
        </div>
        <FooterCopyright companyName="RERP" />
      </div>
    </footer>
  );
};

export default Footer;
