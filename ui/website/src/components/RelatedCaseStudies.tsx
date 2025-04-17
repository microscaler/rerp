import { Component } from 'solid-js';
import FloatingSidebar from './FloatingSidebar';
import { caseStudiesData, getCaseStudyBySlug } from '../data/case-studies-data';

interface RelatedCaseStudiesProps {
  currentSlug: string;
  title?: string;
}

const RelatedCaseStudies: Component<RelatedCaseStudiesProps> = (props) => {
  // Get all case studies except the current one
  const relatedCaseStudies = () => {
    const current = getCaseStudyBySlug(props.currentSlug);
    if (!current) return [];
    
    return caseStudiesData
      .filter(study => study.slug !== props.currentSlug)
      .slice(0, 5); // Get up to 5 related case studies
  };

  const studies = relatedCaseStudies();

  if (studies.length === 0) {
    return null;
  }

  const handleClick = (slug: string) => {
    // Track related case study click
    if (typeof window !== 'undefined' && (window as any).gtag) {
      (window as any).gtag('event', 'related_case_study_click', {
        event_category: 'engagement',
        event_label: props.currentSlug,
        related_slug: slug
      });
    }
  };

  return (
    <FloatingSidebar title={props.title || 'Related Case Studies'}>
      {studies.map((study) => (
        <a
          href={`#case-study-${study.slug}`}
          onClick={(e) => {
            e.preventDefault();
            window.location.hash = `#case-study-${study.slug}`;
            window.scrollTo({ top: 0, behavior: 'smooth' });
            handleClick(study.slug);
          }}
          class="block bg-gray-50 rounded-lg p-4 hover:shadow-md transition-all border border-gray-200 hover:border-primary hover:bg-primary-50"
        >
          <div class="flex items-start space-x-3">
            <div class="text-2xl flex-shrink-0">{study.image}</div>
            <div class="flex-1 min-w-0">
              <h3 class="text-sm font-semibold text-gray-900 mb-1 hover:text-primary transition-colors line-clamp-2">
                {study.title}
              </h3>
              <p class="text-xs text-gray-600 mb-2 line-clamp-2">{study.excerpt}</p>
              <div class="flex items-center text-primary-600 text-xs font-medium">
                <span>Read case study</span>
                <i class="fa-solid fa-arrow-right ml-2"></i>
              </div>
            </div>
          </div>
        </a>
      ))}
    </FloatingSidebar>
  );
};

export default RelatedCaseStudies;

