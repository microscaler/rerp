import { Component, JSXElement } from 'solid-js';
import { CaseStudyCard } from './components';
import type { CaseStudyData } from '../../data/case-studies-data';

export interface CaseStudyLayoutProps {
  data: CaseStudyData;
  children?: JSXElement;
  showBackLink?: boolean;
}

const CaseStudyLayout: Component<CaseStudyLayoutProps> = (props) => {
  return (
    <article>
      <CaseStudyCard
        id={props.data.slug}
        title={props.data.title}
        customer={props.data.customer}
        role={props.data.role}
        location={props.data.location}
        image={props.data.image}
        challenge={props.data.challenge}
        solution={props.data.solution}
        results={props.data.results}
        quote={props.data.quote}
      />
      {props.children}
      {props.showBackLink && (
        <div class="mt-12 pt-8 border-t border-gray-200">
          <a
            href="#case-studies"
            onClick={(e) => {
              e.preventDefault();
              window.location.hash = '#case-studies';
              window.scrollTo({ top: 0, behavior: 'instant' });
            }}
            class="inline-flex items-center text-primary-600 hover:text-primary-700 font-medium"
          >
            <i class="fa-solid fa-arrow-left mr-2"></i>
            Back to Case Studies
          </a>
        </div>
      )}
    </article>
  );
};

export default CaseStudyLayout;

