import { Component } from 'solid-js';

export interface Job {
  id: string;
  shortId: string; // Short unique ID for URLs (e.g., "54ygaf5tg")
  title: string;
  department: string;
  location: string;
  region: string;
  type: 'Full-time' | 'Part-time' | 'Contract' | 'Remote';
  description: string;
  requirements: string[];
  postedDate: string;
}

interface JobListingProps {
  job: Job;
}

const JobListing: Component<JobListingProps> = (props) => {
  const handleApply = () => {
    // Track job application click
    if (typeof window !== 'undefined' && (window as any).gtag) {
      (window as any).gtag('event', 'job_application_click', {
        event_category: 'careers',
        event_label: props.job.id,
        job_title: props.job.title,
        job_location: props.job.location,
      });
    }
    // Navigate to job application page with short unique ID
    window.location.hash = `#job-application-${props.job.shortId}`;
  };

  return (
    <div class="bg-white rounded-lg border border-gray-200 p-6 hover:shadow-lg transition-shadow">
      <div class="flex flex-col md:flex-row md:items-start md:justify-between mb-4">
        <div class="flex-1">
          <h3 class="text-xl font-bold text-gray-900 mb-2">{props.job.title}</h3>
          <div class="flex flex-wrap items-center gap-3 text-sm text-gray-600 mb-3">
            <div class="flex items-center">
              <i class="fa-solid fa-building mr-2 text-primary"></i>
              <span>{props.job.department}</span>
            </div>
            <div class="flex items-center">
              <i class="fa-solid fa-location-dot mr-2 text-primary"></i>
              <span>{props.job.location}</span>
            </div>
            <div class="flex items-center">
              <i class="fa-solid fa-clock mr-2 text-primary"></i>
              <span>{props.job.type}</span>
            </div>
          </div>
        </div>
        <button
          onClick={handleApply}
          class="bg-primary text-white px-6 py-2 rounded-lg hover:bg-blue-700 font-semibold transition-colors whitespace-nowrap"
        >
          Apply Now
        </button>
      </div>
      
      <p class="text-gray-700 mb-4 leading-relaxed">{props.job.description}</p>
      
      <div class="mb-4">
        <h4 class="font-semibold text-gray-900 mb-2">Key Requirements:</h4>
        <ul class="list-disc list-inside space-y-1 text-gray-600 text-sm">
          {props.job.requirements.slice(0, 3).map((req) => (
            <li>{req}</li>
          ))}
        </ul>
      </div>
      
      <div class="text-xs text-gray-500">
        Posted: {props.job.postedDate} | Job ID: {props.job.shortId}
      </div>
    </div>
  );
};

export default JobListing;

