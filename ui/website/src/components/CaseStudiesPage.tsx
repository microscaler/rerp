import { Component, For } from 'solid-js';
import { SectionHeader } from './blogs-news/components';
import { CaseStudyPreviewCard } from './case-studies/components';
import { caseStudiesData } from '../data/case-studies-data';

const CaseStudiesPage: Component = () => {
  return (
    <div class="bg-gray-50">
      <section class="py-20 bg-white">
        <div class="max-w-7xl mx-auto px-6 lg:px-8">
          <SectionHeader
            title="Real Results from Real Traders"
            description="See how PriceWhisperer is helping traders find more opportunities, save time, and improve their results."
          />

          <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
            <For each={caseStudiesData}>
              {(caseStudy) => (
                <CaseStudyPreviewCard caseStudy={caseStudy} />
              )}
            </For>
          </div>
        </div>
      </section>
    </div>
  );
};

export default CaseStudiesPage;

