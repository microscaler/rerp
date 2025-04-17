import { Component } from 'solid-js';
import type { CaseStudyData } from '../../../data/case-studies-data';

export interface CaseStudyPreviewCardProps {
  caseStudy: CaseStudyData;
}

const CaseStudyPreviewCard: Component<CaseStudyPreviewCardProps> = (props) => {
  return (
    <a
      href={`#case-study-${props.caseStudy.slug}`}
      onClick={(e) => {
        e.preventDefault();
        window.location.hash = `#case-study-${props.caseStudy.slug}`;
        window.scrollTo({ top: 0, behavior: 'instant' });
      }}
      class="block bg-gradient-to-br from-gray-50 to-blue-50 rounded-xl p-6 hover:shadow-lg transition-all border border-gray-100"
    >
      <div class="flex items-start mb-4">
        <div class="text-4xl mr-4" role="img" aria-label={`Case study illustration for ${props.caseStudy.customer}`}>
          {props.caseStudy.image}
        </div>
        <div class="flex-1">
          <h3 class="text-xl font-bold text-gray-900 mb-2">{props.caseStudy.title}</h3>
          <div class="flex items-center space-x-4 text-sm text-gray-600 mb-3">
            <span class="flex items-center">
              <i class="fa-solid fa-user text-primary mr-2"></i>
              {props.caseStudy.customer}
            </span>
            <span class="flex items-center">
              <i class="fa-solid fa-briefcase text-primary mr-2"></i>
              {props.caseStudy.role}
            </span>
          </div>
        </div>
      </div>
      
      <p class="text-gray-600 leading-relaxed mb-4">{props.caseStudy.excerpt}</p>
      
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-4 text-sm">
          {props.caseStudy.results.slice(0, 2).map((result) => (
            <div class="flex items-center">
              <span class="text-green-600 font-semibold mr-1">{result.improvement}</span>
              <span class="text-gray-600">{result.metric}</span>
            </div>
          ))}
        </div>
        <div class="flex items-center text-primary-600 font-medium text-sm">
          <span>Read full story</span>
          <i class="fa-solid fa-arrow-right ml-2"></i>
        </div>
      </div>
    </a>
  );
};

export default CaseStudyPreviewCard;

