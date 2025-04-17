import { Component, createSignal, onMount, For, createMemo } from 'solid-js';
import { updateSEO } from '../utils/seo';
import { BASE_URL } from '../config/constants';
import TechnologiesWordCloud from './careers/TechnologiesWordCloud';
import JobListing from './careers/JobListing';
import { getJobsByRegionAndDepartment, regions, departmentsWithAll } from '../data/jobs-data';
import { SectionHeader } from './blogs-news/components';
import { HeroWithStats, FeatureCard, CTASection, EmptyState, RegionFilter, DepartmentFilter } from './shared';

const CareersPage: Component = () => {
  const [selectedRegion, setSelectedRegion] = createSignal<string>('');
  const [selectedDepartment, setSelectedDepartment] = createSignal<string>('');
  
  // Only show jobs when both filters are selected
  const filteredJobs = createMemo(() => {
    const region = selectedRegion();
    const department = selectedDepartment();
    
    if (!region || !department) {
      return [];
    }
    
    return getJobsByRegionAndDepartment(region, department);
  });
  
  const hasFiltersSelected = createMemo(() => {
    return selectedRegion() !== '' && selectedDepartment() !== '';
  });

  onMount(() => {
    updateSEO({
      title: 'Careers - Join PriceWhisperer',
      description: 'Join PriceWhisperer and help build the future of AI-powered trading. Explore open positions across North America, Europe, Asia Pacific, and Latin America.',
      keywords: 'careers, jobs, PriceWhisperer jobs, trading platform careers, fintech jobs',
      canonical: `${BASE_URL}/#careers`,
      ogType: 'website',
      ogImage: '/og-image.jpg'
    });
  });

  const handleRegionChange = (region: string) => {
    setSelectedRegion(region);
    
    // Track region filter
    if (typeof window !== 'undefined' && (window as any).gtag) {
      (window as any).gtag('event', 'careers_region_filter', {
        event_category: 'engagement',
        event_label: region,
      });
    }
  };
  
  const handleDepartmentChange = (department: string) => {
    setSelectedDepartment(department);
    
    // Track department filter
    if (typeof window !== 'undefined' && (window as any).gtag) {
      (window as any).gtag('event', 'careers_department_filter', {
        event_category: 'engagement',
        event_label: department,
      });
    }
  };

  const whyWorkHereFeatures = [
    {
      icon: 'fa-solid fa-rocket',
      iconBgColor: 'bg-primary',
      title: 'Cutting-Edge Technology',
      description: 'Work with the latest AI/ML technologies, real-time data processing, and modern tech stack.',
    },
    {
      icon: 'fa-solid fa-globe',
      iconBgColor: 'bg-secondary',
      title: 'Global Impact',
      description: 'Help traders worldwide find profitable opportunities across 25+ global exchanges.',
    },
    {
      icon: 'fa-solid fa-users',
      iconBgColor: 'bg-accent',
      title: 'Great Team',
      description: 'Work with talented engineers, traders, and product experts from around the world.',
    },
  ];

  return (
    <div class="min-h-screen bg-gray-50">
      <main>
        {/* Hero Section */}
        <HeroWithStats
          title="Join the PriceWhisperer Team"
          description="Help us build the future of AI-powered trading. We're looking for talented individuals to join our global team and revolutionize how traders find opportunities."
          stats={[]}
        />

        {/* Technologies Word Cloud */}
        <section class="py-16 bg-white">
          <div class="max-w-7xl mx-auto px-6 lg:px-8">
            <SectionHeader
              title="42 Technologies: The Answer to Everything"
              description="After 7.5 million years, Deep Thought calculated that 42 is the answer to the Ultimate Question. While we haven't solved the universe (yet), we've curated exactly 42 cutting-edge technologies powering PriceWhisperer. Work with the latest innovations in AI, data processing, infrastructure, and modern software development."
            />
            <TechnologiesWordCloud />
          </div>
        </section>

        {/* Job Listings */}
        <section class="py-16 bg-gray-50">
          <div class="max-w-7xl mx-auto px-6 lg:px-8">
            <div class="mb-8">
              <SectionHeader
                title="Open Positions"
                titleSize="default"
              />
              
              {/* Filters */}
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
                {/* Region Filter */}
                <RegionFilter
                  regions={['', ...regions]}
                  selectedRegion={selectedRegion()}
                  onRegionChange={handleRegionChange}
                  label="Select Region *"
                />

                {/* Department Filter */}
                <DepartmentFilter
                  departments={['', ...departmentsWithAll]}
                  selectedDepartment={selectedDepartment()}
                  onDepartmentChange={handleDepartmentChange}
                  label="Select Area of Interest *"
                />
              </div>

              {/* Job Count */}
              {hasFiltersSelected() && (
                <div class="text-gray-600 mb-6">
                  Showing <strong>{filteredJobs().length}</strong> {filteredJobs().length === 1 ? 'position' : 'positions'}
                  {selectedRegion() !== 'All Regions' && selectedRegion() !== '' && ` in ${selectedRegion()}`}
                  {selectedDepartment() !== 'All Departments' && selectedDepartment() !== '' && ` in ${selectedDepartment()}`}
                </div>
              )}
            </div>

            {/* Job Listings */}
            {!hasFiltersSelected() ? (
              <EmptyState
                icon="fa-solid fa-filter"
                title="Select Your Filters"
                description="Please select a Region and Area of Interest to view available positions."
              />
            ) : filteredJobs().length > 0 ? (
              <div class="space-y-6">
                <For each={filteredJobs()}>
                  {(job) => <JobListing job={job} />}
                </For>
              </div>
            ) : (
              <EmptyState
                icon="fa-solid fa-briefcase"
                title="No positions available"
                description={`We don't have any open positions matching your selected filters at the moment. Try selecting different options or check back soon!`}
              />
            )}
          </div>
        </section>

        {/* Why Work Here Section */}
        <section class="py-16 bg-white">
          <div class="max-w-7xl mx-auto px-6 lg:px-8">
            <SectionHeader
              title="Why Work at PriceWhisperer?"
              description="Join a team that's revolutionizing trading with AI and cutting-edge technology."
            />
            
            <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
              <For each={whyWorkHereFeatures}>
                {(feature) => (
                  <FeatureCard
                    icon={feature.icon}
                    iconBgColor={feature.iconBgColor}
                    title={feature.title}
                    description={feature.description}
                    variant="centered"
                  />
                )}
              </For>
            </div>
          </div>
        </section>

        {/* CTA Section */}
        <CTASection
          title="Don't See the Right Role?"
          description="We're always looking for exceptional talent. Submit a general application and we'll keep you in mind for future opportunities."
          primaryButton={{
            text: 'Submit General Application',
            onClick: () => {
              if (typeof window !== 'undefined' && (window as any).gtag) {
                (window as any).gtag('event', 'careers_general_application', {
                  event_category: 'conversion',
                  event_label: 'general_application_click',
                });
              }
              window.location.hash = '#job-application';
            },
          }}
        />
      </main>
    </div>
  );
};

export default CareersPage;

