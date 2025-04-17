import { Component, For } from 'solid-js';
import FooterLink, { FooterLinkProps } from './FooterLink';

export interface FooterLinkSectionProps {
  title: string;
  links: FooterLinkProps[];
}

const FooterLinkSection: Component<FooterLinkSectionProps> = (props) => {
  return (
    <div>
      <h4 class="text-white font-semibold mb-4">{props.title}</h4>
      <ul class="space-y-2">
        <For each={props.links}>
          {(link) => (
            <li>
              <FooterLink {...link} />
            </li>
          )}
        </For>
      </ul>
    </div>
  );
};

export default FooterLinkSection;

