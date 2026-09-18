import { Component } from 'solid-js';

export interface FooterCopyrightProps {
  companyName: string;
  year?: number;
  additionalText?: string;
}

const FooterCopyright: Component<FooterCopyrightProps> = (props) => {
  const currentYear = props.year || new Date().getFullYear();

  return (
    <div class="border-t border-gray-800 mt-8 pt-8 text-center">
      <p class="text-gray-400">
        &copy; {currentYear} {props.companyName}. All rights reserved.
        {props.additionalText && ` ${props.additionalText}`}
      </p>
    </div>
  );
};

export default FooterCopyright;

