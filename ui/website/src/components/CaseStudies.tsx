import { Component, For } from 'solid-js';
import { CaseStudyPreviewCard } from './case-studies/components';
import { SectionHeader } from './blogs-news/components';
import { caseStudiesData } from '../data/case-studies-data';

const CaseStudies: Component = () => {
  // Show only first 3 case studies as previews on homepage
  const previewCaseStudies = caseStudiesData.slice(0, 3);

  return (
    <section id="case-studies" class="py-20 bg-white">
      <div class="max-w-7xl mx-auto px-6 lg:px-8">
        <SectionHeader
          title="Real Results from Real Traders"
          description="See how PriceWhisperer is helping traders find more opportunities, save time, and improve their results."
        />

        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
          <For each={previewCaseStudies}>
            {(caseStudy) => (
              <CaseStudyPreviewCard caseStudy={caseStudy} />
            )}
          </For>
        </div>

        {/* View All Link */}
        <div class="text-center mt-12">
          <a
            href="#case-studies"
            onClick={(e) => {
              e.preventDefault();
              window.location.hash = '#case-studies';
              window.scrollTo({ top: 0, behavior: 'instant' });
            }}
            class="inline-flex items-center text-primary-600 hover:text-primary-700 font-semibold text-lg"
          >
            <span>View All Case Studies</span>
            <i class="fa-solid fa-arrow-right ml-2"></i>
          </a>
        </div>
      </div>
    </section>
  );
};

export default CaseStudies;

