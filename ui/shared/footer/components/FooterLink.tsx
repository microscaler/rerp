import { Component, JSX } from 'solid-js';

export interface FooterLinkProps {
  text: string;
  href: string;
  external?: boolean;
  onClick?: (e: MouseEvent) => void;
}

const FooterLink: Component<FooterLinkProps> = (props) => {
  const handleClick: JSX.EventHandler<HTMLAnchorElement, MouseEvent> = (e) => {
    if (props.onClick) {
      props.onClick(e);
    } else if (!props.external && props.href.startsWith('#')) {
      // Handle internal hash navigation
      e.preventDefault();
      window.location.hash = props.href;
      window.scrollTo({ top: 0, behavior: 'instant' });
    }
  };

  if (props.external) {
    return (
      <a
        href={props.href}
        target="_blank"
        rel="noopener noreferrer"
        class="text-gray-400 hover:text-white transition-colors whitespace-nowrap inline-flex items-center"
        onClick={handleClick}
      >
        <span>{props.text}</span>
        <i class="fa-solid fa-external-link-alt ml-1 text-xs"></i>
      </a>
    );
  }

  return (
    <a
      href={props.href}
      class="text-gray-400 hover:text-white transition-colors"
      onClick={handleClick}
    >
      {props.text}
    </a>
  );
};

export default FooterLink;

