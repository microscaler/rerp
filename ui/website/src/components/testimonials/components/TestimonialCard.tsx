import { Component, For } from 'solid-js';

export interface TestimonialCardProps {
  id?: string;
  name: string;
  role: string;
  location: string;
  rating: number; // 1-5
  quote: string;
  avatar: string; // image path
  results: string; // key result text
  caseStudyLink?: string; // Optional link to case study
}

const TestimonialCard: Component<TestimonialCardProps> = (props) => {
  const cardContent = (
    <>
      <div class="flex items-center mb-4">
        <div class="relative h-16 w-16 mr-4 flex-shrink-0">
          <img
            src={props.avatar}
            alt={`Avatar for ${props.name}`}
            class="h-16 w-16 rounded-full object-cover"
          />
        </div>
        <div class="flex-1">
          <div class="font-semibold text-gray-900">{props.name}</div>
          <div class="text-sm text-gray-600">{props.role}</div>
          <div class="text-xs text-gray-500">{props.location}</div>
        </div>
      </div>
      
      <div class="flex items-center mb-4">
        <For each={Array.from({ length: props.rating })}>
          {() => <i class="fa-solid fa-star text-yellow-400 text-sm"></i>}
        </For>
      </div>
      
      <p class="text-gray-700 leading-relaxed mb-4 italic">
        "{props.quote}"
      </p>
      
      <div class="bg-primary/10 rounded-lg p-3 mt-4">
        <div class="text-xs text-gray-600 mb-1">Key Result:</div>
        <div class="font-semibold text-primary">{props.results}</div>
      </div>
      {props.caseStudyLink && (
        <div class="mt-4 pt-4 border-t border-gray-200">
          <div class="flex items-center text-primary-600 font-medium text-sm">
            <span>Read full case study</span>
            <i class="fa-solid fa-arrow-right ml-2"></i>
          </div>
        </div>
      )}
    </>
  );

  if (props.caseStudyLink) {
    return (
      <a
        id={props.id}
        href={props.caseStudyLink}
        onClick={(e) => {
          e.preventDefault();
          window.location.hash = props.caseStudyLink!;
          window.scrollTo({ top: 0, behavior: 'instant' });
        }}
        class="block bg-gray-50 rounded-xl p-8 hover:shadow-lg transition-all border border-gray-100 cursor-pointer"
      >
        {cardContent}
      </a>
    );
  }

  return (
    <div
      id={props.id}
      class="bg-gray-50 rounded-xl p-8 hover:shadow-lg transition-shadow border border-gray-100"
    >
      {cardContent}
    </div>
  );
};

export default TestimonialCard;

