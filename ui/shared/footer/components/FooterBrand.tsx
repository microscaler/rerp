import { Component } from 'solid-js';

export interface FooterBrandProps {
  logo?: JSX.Element;
  name: string;
  description: string;
  socialLinks?: Array<{
    icon: string;
    href: string;
    label?: string;
  }>;
}

const FooterBrand: Component<FooterBrandProps> = (props) => {
  return (
    <div class="space-y-4">
      <div class="flex items-center space-x-2">
        {props.logo || (
          <i class="fa-solid fa-chart-line text-primary text-xl"></i>
        )}
        <span class="text-lg font-bold text-white">{props.name}</span>
      </div>
      <p class="text-gray-400">{props.description}</p>
      {props.socialLinks && props.socialLinks.length > 0 && (
        <div class="flex space-x-4">
          {props.socialLinks.map((social) => (
            <a
              href={social.href}
              class="text-gray-400 hover:text-white transition-colors"
              aria-label={social.label || social.icon}
            >
              <i class={social.icon}></i>
            </a>
          ))}
        </div>
      )}
    </div>
  );
};

export default FooterBrand;

