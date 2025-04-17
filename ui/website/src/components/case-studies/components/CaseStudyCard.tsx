import { Component } from 'solid-js';
import CustomerInfo from './CustomerInfo';
import ResultsTable from './ResultsTable';
import CaseStudyQuote from './CaseStudyQuote';
import type { ResultMetric } from './ResultsTable';

export interface CaseStudyCardProps {
  id?: string;
  title: string;
  customer: string;
  role: string;
  location: string;
  image: string; // emoji
  challenge: string;
  solution: string;
  results: ResultMetric[];
  quote: string;
}

const getImageAriaLabel = (image: string): string => {
  if (image === '📈') return 'Chart showing growth';
  if (image === '💰') return 'Money bag representing profit';
  if (image === '⏰') return 'Clock representing time savings';
  return 'Case study illustration';
};

const CaseStudyCard: Component<CaseStudyCardProps> = (props) => {
  return (
    <div
      id={props.id}
      class="bg-gradient-to-br from-gray-50 to-blue-50 rounded-2xl p-8 md:p-12 shadow-lg"
    >
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
        {/* Left Column - Overview */}
        <div class="lg:col-span-1">
          <div
            class="text-6xl mb-4"
            role="img"
            aria-label={getImageAriaLabel(props.image)}
          >
            {props.image}
          </div>
          <h3 class="text-2xl font-bold text-gray-900 mb-4">{props.title}</h3>
          <CustomerInfo
            customer={props.customer}
            role={props.role}
            location={props.location}
          />
        </div>

        {/* Right Column - Details */}
        <div class="lg:col-span-2 space-y-6">
          {/* Challenge */}
          <div>
            <div class="flex items-center space-x-2 mb-2">
              <i class="fa-solid fa-exclamation-triangle text-accent"></i>
              <h4 class="font-semibold text-gray-900">The Challenge</h4>
            </div>
            <p class="text-gray-600 leading-relaxed">{props.challenge}</p>
          </div>

          {/* Solution */}
          <div>
            <div class="flex items-center space-x-2 mb-2">
              <i class="fa-solid fa-lightbulb text-secondary"></i>
              <h4 class="font-semibold text-gray-900">The Solution</h4>
            </div>
            <p class="text-gray-600 leading-relaxed">{props.solution}</p>
          </div>

          {/* Results */}
          <div>
            <div class="flex items-center space-x-2 mb-4">
              <i class="fa-solid fa-chart-line text-primary"></i>
              <h4 class="font-semibold text-gray-900">The Results</h4>
            </div>
            <ResultsTable results={props.results} />
          </div>

          {/* Quote */}
          <CaseStudyQuote
            quote={props.quote}
            customer={props.customer}
            role={props.role}
          />
        </div>
      </div>
    </div>
  );
};

export default CaseStudyCard;

