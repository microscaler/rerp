import { Component } from 'solid-js';

export interface CaseStudyQuoteProps {
  quote: string;
  customer: string;
  role: string;
}

const CaseStudyQuote: Component<CaseStudyQuoteProps> = (props) => {
  return (
    <div class="bg-white rounded-lg p-6 border-l-4 border-primary">
      <i class="fa-solid fa-quote-left text-primary text-2xl mb-3"></i>
      <p class="text-gray-700 italic leading-relaxed mb-3">"{props.quote}"</p>
      <div class="text-sm text-gray-600">— {props.customer}, {props.role}</div>
    </div>
  );
};

export default CaseStudyQuote;

