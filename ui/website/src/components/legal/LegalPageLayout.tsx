import { Component, JSX } from 'solid-js';

export interface LegalPageHeaderProps {
  title: string;
  lastUpdated: string;
  icon: string;
  iconBgColor?: string;
}

export interface LegalPageLayoutProps {
  header: LegalPageHeaderProps;
  children: JSX.Element;
  showBackLink?: boolean;
}

const LegalPageHeader: Component<LegalPageHeaderProps> = (props) => {
  const iconBgColor = props.iconBgColor || 'bg-primary';
  
  return (
    <div class="mb-8">
      <div class="flex items-center mb-4">
        <div class={`w-12 h-12 ${iconBgColor} rounded-lg flex items-center justify-center mr-4`}>
          <i class={`${props.icon} text-white text-xl`}></i>
        </div>
        <div>
          <h1 class="text-4xl font-extrabold text-gray-900">{props.title}</h1>
          <div class="text-sm text-gray-600 mt-2">
            Last updated: {props.lastUpdated}
          </div>
        </div>
      </div>
    </div>
  );
};

const LegalPageFooter: Component<{ showBackLink?: boolean }> = (props) => {
  if (!props.showBackLink) return null;
  
  return (
    <div class="mt-12 pt-8 border-t border-gray-200">
      <a
        href="#"
        onClick={(e) => {
          e.preventDefault();
          window.location.hash = '';
          window.scrollTo({ top: 0, behavior: 'instant' });
        }}
        class="inline-flex items-center text-primary-600 hover:text-primary-700 font-medium"
      >
        <i class="fa-solid fa-arrow-left mr-2"></i>
        Back to Home
      </a>
    </div>
  );
};

const LegalPageLayout: Component<LegalPageLayoutProps> = (props) => {
  return (
    <article>
      <div class="bg-white rounded-lg shadow-lg p-8 md:p-12">
        <LegalPageHeader {...props.header} />
        
        <div class="prose prose-lg max-w-none">
          {props.children}
        </div>

        <LegalPageFooter showBackLink={props.showBackLink} />
      </div>
    </article>
  );
};

export default LegalPageLayout;

