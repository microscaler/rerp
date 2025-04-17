import { Component, createSignal, Show, onMount, createEffect, createMemo, Index } from 'solid-js';
import { getAllTechnologyNames } from '../data/technologies-data';
import { getJobByShortId } from '../data/jobs-data';
import { FormInput, FormCheckbox, Alert, DatePicker } from './shared';
import { executeRecaptcha } from '../utils/recaptcha';
import { updateSEO } from '../utils/seo';
import { BASE_URL } from '../config/constants';

interface WorkExperience {
  company: string;
  jobTitle: string;
  startDate: string;
  endDate: string;
  background: string;
  technicalExperience: string;
}

// Helper function to calculate duration in years and months
const calculateDuration = (startDate: string, endDate: string): string => {
  if (!startDate) return '';
  if (!endDate) return 'Present';
  
  const start = new Date(startDate);
  const end = new Date(endDate);
  
  if (isNaN(start.getTime()) || isNaN(end.getTime())) return '';
  if (end < start) return '';
  
  const years = end.getFullYear() - start.getFullYear();
  const months = end.getMonth() - start.getMonth();
  
  let totalMonths = years * 12 + months;
  if (end.getDate() < start.getDate()) {
    totalMonths--;
  }
  
  const finalYears = Math.floor(totalMonths / 12);
  const finalMonths = totalMonths % 12;
  
  if (finalYears === 0 && finalMonths === 0) {
    return 'Less than 1 month';
  } else if (finalYears === 0) {
    return `${finalMonths} ${finalMonths === 1 ? 'month' : 'months'}`;
  } else if (finalMonths === 0) {
    return `${finalYears} ${finalYears === 1 ? 'year' : 'years'}`;
  } else {
    return `${finalYears} ${finalYears === 1 ? 'year' : 'years'}, ${finalMonths} ${finalMonths === 1 ? 'month' : 'months'}`;
  }
};

// Composable RoleForm component
interface RoleFormProps {
  index: number;
  role: WorkExperience;
  onFieldChange: (index: number, field: keyof WorkExperience, value: string) => void;
  onRemove: (index: number) => void;
  canRemove: boolean;
  today: string;
  validation?: { backgroundTouched: boolean; technicalExperienceTouched: boolean; backgroundError?: string; technicalExperienceError?: string };
  onValidate: (index: number, field: 'background' | 'technicalExperience', value: string) => void;
}

const RoleForm: Component<RoleFormProps> = (props) => {
  return (
    <div class="p-6 bg-gray-50 rounded-lg space-y-4 border border-gray-200">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-lg font-semibold text-gray-900">Role {props.index + 1}</h3>
        <Show when={props.canRemove}>
          <button
            type="button"
            onClick={() => props.onRemove(props.index)}
            class="text-sm text-red-600 hover:text-red-700 hover:bg-red-50 px-3 py-1 rounded-lg transition-colors flex items-center"
          >
            <i class="fa-solid fa-trash mr-1"></i>
            Remove
          </button>
        </Show>
      </div>
      
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <FormInput
          id={`company-${props.index}`}
          label="Company"
          type="text"
          value={props.role.company}
          onInput={(value) => props.onFieldChange(props.index, 'company', value)}
        />
        <FormInput
          id={`jobTitle-${props.index}`}
          label="Job Title"
          type="text"
          value={props.role.jobTitle}
          onInput={(value) => props.onFieldChange(props.index, 'jobTitle', value)}
        />
      </div>
      
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <DatePicker
          id={`startDate-${props.index}`}
          label="Start Date"
          value={props.role.startDate}
          onInput={(value) => props.onFieldChange(props.index, 'startDate', value)}
          max={props.role.endDate || props.today}
          placeholder="Select start date"
        />
        <DatePicker
          id={`endDate-${props.index}`}
          label="End Date"
          value={props.role.endDate}
          onInput={(value) => props.onFieldChange(props.index, 'endDate', value)}
          max={props.today}
          min={props.role.startDate}
          placeholder="Select end date"
        />
        <div class="flex items-end">
          <div class="w-full">
            <label class="block text-sm font-medium text-gray-700 mb-1">
              Duration
            </label>
            <div class="px-3 py-2 border border-gray-300 rounded-lg bg-gray-50 text-gray-700">
              {calculateDuration(props.role.startDate, props.role.endDate) || 'Enter dates to calculate'}
            </div>
          </div>
        </div>
      </div>
      
      <div>
        <label for={`background-${props.index}`} class="block text-sm font-medium text-gray-700 mb-1">
          Background & Experience
        </label>
        <textarea
          id={`background-${props.index}`}
          rows={3}
          placeholder="Describe your role, key achievements, and responsibilities at this company..."
          value={props.role.background}
          onInput={(e) => props.onFieldChange(props.index, 'background', e.currentTarget.value)}
          onBlur={(e) => props.onValidate(props.index, 'background', e.currentTarget.value)}
          class={`w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent ${
            props.validation?.backgroundError ? 'border-red-300' : 'border-gray-300'
          }`}
        />
        {props.validation?.backgroundTouched && props.validation?.backgroundError && (
          <p class="mt-1 text-sm text-red-600">{props.validation.backgroundError}</p>
        )}
      </div>
      
      <div>
        <label for={`technicalExperience-${props.index}`} class="block text-sm font-medium text-gray-700 mb-1">
          Technical Experience
        </label>
        <textarea
          id={`technicalExperience-${props.index}`}
          rows={3}
          placeholder="Describe the technical skills, technologies, and projects you worked on in this role..."
          value={props.role.technicalExperience}
          onInput={(e) => props.onFieldChange(props.index, 'technicalExperience', e.currentTarget.value)}
          onBlur={(e) => props.onValidate(props.index, 'technicalExperience', e.currentTarget.value)}
          class={`w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent ${
            props.validation?.technicalExperienceError ? 'border-red-300' : 'border-gray-300'
          }`}
        />
        {props.validation?.technicalExperienceTouched && props.validation?.technicalExperienceError && (
          <p class="mt-1 text-sm text-red-600">{props.validation.technicalExperienceError}</p>
        )}
      </div>
    </div>
  );
};

const JobApplicationPage: Component = () => {
  // Extract shortId from hash and lookup job
  const extractJobInfo = () => {
    if (typeof window === 'undefined') return { shortId: undefined, job: undefined };
    const hash = window.location.hash;
    if (hash.startsWith('#job-application-')) {
      const shortId = hash.replace('#job-application-', '');
      const job = getJobByShortId(shortId);
      return { shortId, job };
    }
    return { shortId: undefined, job: undefined };
  };

  const [jobInfo, setJobInfo] = createSignal(extractJobInfo());

  createEffect(() => {
    setJobInfo(extractJobInfo());
  });

  onMount(() => {
    const job = jobInfo().job;
    updateSEO({
      title: job ? `Apply for ${job.title} - PriceWhisperer` : 'Job Application - PriceWhisperer',
      description: 'Apply to join the PriceWhisperer team and help build the future of AI-powered trading.',
      keywords: 'job application, careers, PriceWhisperer jobs',
      canonical: `${BASE_URL}/#job-application${jobInfo().shortId ? `-${jobInfo().shortId}` : ''}`,
      ogType: 'website',
      ogImage: '/og-image.jpg'
    });

    // Initialize validation state for the first role
    setRoleValidation(prev => ({
      ...prev,
      0: { backgroundTouched: false, technicalExperienceTouched: false },
    }));

    // Listen for hash changes
    const handleHashChange = () => {
      setJobInfo(extractJobInfo());
    };
    window.addEventListener('hashchange', handleHashChange);
    
    return () => {
      window.removeEventListener('hashchange', handleHashChange);
    };
  });

  const [formData, setFormData] = createSignal({
    firstName: '',
    lastName: '',
    email: '',
    phone: '',
    location: '',
    linkedin: '',
    github: '',
    interest: '',
    otherTechnologies: '',
  });

  const [selectedTechnologies, setSelectedTechnologies] = createSignal<Set<string>>(new Set());
  const [workExperience, setWorkExperience] = createSignal<WorkExperience[]>([
    { company: '', jobTitle: '', startDate: '', endDate: '', background: '', technicalExperience: '' },
  ]);
  
  // Validation state: track touched fields and errors for each role
  const [roleValidation, setRoleValidation] = createSignal<Record<number, { backgroundTouched: boolean; technicalExperienceTouched: boolean; backgroundError?: string; technicalExperienceError?: string }>>({});
  
  // Helper to get role by index
  const getRole = (index: number) => workExperience()[index];
  
  // Helper to count words
  const countWords = (text: string): number => {
    return text.trim().split(/\s+/).filter(word => word.length > 0).length;
  };
  
  // Validate role fields
  const validateRoleField = (index: number, field: 'background' | 'technicalExperience', value: string) => {
    const wordCount = countWords(value);
    const minWords = 50;
    const error = wordCount < minWords ? `Please enter at least ${minWords} words (currently ${wordCount})` : undefined;
    
    setRoleValidation(prev => ({
      ...prev,
      [index]: {
        backgroundTouched: prev[index]?.backgroundTouched || false,
        technicalExperienceTouched: prev[index]?.technicalExperienceTouched || false,
        backgroundError: prev[index]?.backgroundError,
        technicalExperienceError: prev[index]?.technicalExperienceError,
        [`${field}Touched`]: true,
        [`${field}Error`]: error,
      },
    }));
    
    return !error;
  };
  
  // Check if a role is complete (all validations pass)
  const isRoleComplete = (index: number): boolean => {
    const role = getRole(index);
    const validation = roleValidation()[index];
    
    if (!validation) return false;
    
    const backgroundValid = !validation.backgroundError && validation.backgroundTouched;
    const technicalValid = !validation.technicalExperienceError && validation.technicalExperienceTouched;
    
    return backgroundValid && technicalValid;
  };
  
  // Check if we can add a new role (current role must be complete)
  // Must directly access roleValidation() and check the specific index to make it reactive
  const canAddRole = createMemo(() => {
    const currentIndex = workExperience().length - 1;
    const validation = roleValidation(); // Access validation signal to make memo reactive
    const currentValidation = validation[currentIndex];
    
    if (!currentValidation) return false;
    
    const backgroundValid = !currentValidation.backgroundError && currentValidation.backgroundTouched;
    const technicalValid = !currentValidation.technicalExperienceError && currentValidation.technicalExperienceTouched;
    
    return backgroundValid && technicalValid;
  });
  
  // Get today's date for date picker max
  const today = createMemo(() => new Date().toISOString().split('T')[0]);
  const [isSubmitting, setIsSubmitting] = createSignal(false);
  const [submitError, setSubmitError] = createSignal<string | null>(null);
  const [submitSuccess, setSubmitSuccess] = createSignal(false);

  const allTechnologies = getAllTechnologyNames();

  const handleInputChange = (field: string, value: string) => {
    setFormData(prev => ({ ...prev, [field]: value }));
  };

  const handleTechnologyToggle = (tech: string) => {
    setSelectedTechnologies(prev => {
      const newSet = new Set(prev);
      if (newSet.has(tech)) {
        newSet.delete(tech);
      } else {
        newSet.add(tech);
      }
      return newSet;
    });
  };

  const handleWorkExperienceChange = (index: number, field: keyof WorkExperience, value: string) => {
    setWorkExperience(prev => {
      const updated = [...prev];
      updated[index] = { ...updated[index], [field]: value };
      return updated;
    });
  };

  const addWorkExperience = () => {
    const currentIndex = workExperience().length - 1;
    if (!isRoleComplete(currentIndex)) {
      return; // Don't add if current role is not complete
    }
    const newIndex = workExperience().length;
    setWorkExperience(prev => [...prev, { company: '', jobTitle: '', startDate: '', endDate: '', background: '', technicalExperience: '' }]);
    // Initialize validation state for new role
    setRoleValidation(prev => ({
      ...prev,
      [newIndex]: { backgroundTouched: false, technicalExperienceTouched: false },
    }));
  };

  const removeWorkExperience = (index: number) => {
    setWorkExperience(prev => prev.filter((_, i) => i !== index));
  };


  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    setIsSubmitting(true);
    setSubmitError(null);

    try {
      // Execute reCAPTCHA v3 (transparent to user)
      const captchaToken = await executeRecaptcha('submit_job_application');

      // Track form submission
      if (typeof window !== 'undefined' && (window as any).gtag) {
        const job = jobInfo().job;
        (window as any).gtag('event', 'job_application_submit', {
          event_category: 'careers',
          event_label: jobInfo().shortId || 'general',
          job_title: job?.title || 'general',
        });
      }

      // TODO: Replace with actual API call
      // For now, simulate API call
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      // In production, this would be:
      // const response = await fetch('/api/job-applications', {
      //   method: 'POST',
      //   headers: { 'Content-Type': 'application/json' },
      //   body: JSON.stringify({
      //     ...formData(),
      //     technologies: Array.from(selectedTechnologies()),
      //     jobId: jobInfo().shortId,
      //     jobTitle: jobInfo().job?.title,
      //     captchaToken: captchaToken,
      //     description: {
      //       interest: formData().interest,
      //     },
      //   }),
      // });

      setSubmitSuccess(true);
      
      // Redirect to careers page after 3 seconds
      setTimeout(() => {
        window.location.hash = '#careers';
      }, 3000);
    } catch (error) {
      setSubmitError('Failed to submit application. Please try again or contact us directly.');
      console.error('Application submission error:', error);
    } finally {
      setIsSubmitting(false);
    }
  };

  const currentJob = () => jobInfo().job;
  const currentShortId = () => jobInfo().shortId;

  return (
    <div class="min-h-screen bg-gray-50">
      <main class="max-w-4xl mx-auto px-6 lg:px-8 py-12">
        <div class="mb-8">
          <button
            onClick={() => window.location.hash = '#careers'}
            class="text-primary hover:text-blue-700 mb-4 flex items-center"
          >
            <i class="fa-solid fa-arrow-left mr-2"></i>
            Back to Careers
          </button>
          <h1 class="text-4xl font-bold text-gray-900 mb-2">
            {currentJob() ? `Apply for ${currentJob()!.title}` : 'Job Application'}
          </h1>
          <div class="flex items-center gap-4 mb-2">
            <p class="text-gray-600">
              Help us build the future of AI-powered trading. Fill out the form below to apply.
            </p>
            {currentJob() && currentShortId() && (
              <span class="text-sm text-gray-500">
                Job ID: {currentShortId()}
              </span>
            )}
          </div>
        </div>

        <div class="bg-white rounded-lg shadow-lg p-8">
          <form onSubmit={handleSubmit} class="space-y-6">
            <Show when={submitSuccess()}>
              <Alert
                type="success"
                message="Application submitted successfully!"
                description="We'll review your application and get back to you soon. Redirecting to careers page..."
              />
            </Show>

            <Show when={submitError()}>
              <Alert
                type="error"
                message={submitError()!}
              />
            </Show>

            {/* Personal Information */}
            <div class="border-b border-gray-200 pb-6">
              <h2 class="text-xl font-semibold text-gray-900 mb-4">Personal Information</h2>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <FormInput
                  id="firstName"
                  label="First Name"
                  type="text"
                  required
                  value={formData().firstName}
                  onInput={(value) => handleInputChange('firstName', value)}
                />
                <FormInput
                  id="lastName"
                  label="Last Name"
                  type="text"
                  required
                  value={formData().lastName}
                  onInput={(value) => handleInputChange('lastName', value)}
                />
                <FormInput
                  id="email"
                  label="Email"
                  type="email"
                  required
                  value={formData().email}
                  onInput={(value) => handleInputChange('email', value)}
                />
                <FormInput
                  id="phone"
                  label="Phone"
                  type="tel"
                  required
                  value={formData().phone}
                  onInput={(value) => handleInputChange('phone', value)}
                />
                <FormInput
                  id="location"
                  label="Location"
                  type="text"
                  required
                  placeholder="City, Country"
                  value={formData().location}
                  onInput={(value) => handleInputChange('location', value)}
                />
                <FormInput
                  id="linkedin"
                  label="LinkedIn Profile"
                  type="url"
                  placeholder="https://linkedin.com/in/yourprofile"
                  value={formData().linkedin}
                  onInput={(value) => handleInputChange('linkedin', value)}
                />
                <FormInput
                  id="github"
                  label="GitHub Profile"
                  type="url"
                  placeholder="https://github.com/yourusername"
                  value={formData().github}
                  onInput={(value) => handleInputChange('github', value)}
                />
              </div>
            </div>

            {/* Technologies */}
            <div class="border-b border-gray-200 pb-6">
              <h2 class="text-xl font-semibold text-gray-900 mb-4">
                Technologies <span class="text-sm font-normal text-gray-500">(Select all that apply)</span>
              </h2>
              <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3 max-h-64 overflow-y-auto p-2 border border-gray-200 rounded-lg">
                {allTechnologies.map((tech) => (
                  <FormCheckbox
                    id={`tech-${tech}`}
                    label={tech}
                    checked={selectedTechnologies().has(tech)}
                    onChange={(checked) => {
                      if (checked) {
                        setSelectedTechnologies(prev => new Set(prev).add(tech));
                      } else {
                        setSelectedTechnologies(prev => {
                          const newSet = new Set(prev);
                          newSet.delete(tech);
                          return newSet;
                        });
                      }
                    }}
                  />
                ))}
              </div>
              <div class="mt-4">
                <FormInput
                  id="otherTechnologies"
                  label="Other Technologies"
                  type="text"
                  placeholder="List any other technologies you work with (comma-separated)"
                  value={formData().otherTechnologies}
                  onInput={(value) => handleInputChange('otherTechnologies', value)}
                />
              </div>
            </div>

            {/* Work Experience */}
            <div class="border-b border-gray-200 pb-6">
              <div class="flex items-center justify-between mb-4">
                <h2 class="text-xl font-semibold text-gray-900">Work Experience</h2>
                <button
                  type="button"
                  onClick={addWorkExperience}
                  disabled={!canAddRole()}
                  class={`text-sm font-medium flex items-center ${
                    canAddRole()
                      ? 'text-primary hover:text-blue-700'
                      : 'text-gray-400 cursor-not-allowed'
                  }`}
                >
                  <i class="fa-solid fa-plus mr-1"></i>
                  Add Role
                </button>
              </div>
              <div class="mb-4 p-4 bg-blue-50 border border-blue-200 rounded-lg">
                <div class="flex items-start">
                  <i class="fa-solid fa-shield-halved text-blue-600 mt-0.5 mr-3"></i>
                  <div>
                    <p class="text-sm font-semibold text-blue-900 mb-1">
                      Background Verification Notice
                    </p>
                    <p class="text-sm text-blue-800">
                      All work experience entries will be verified through{' '}
                      <a
                        href="https://fadv.com/"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="underline hover:text-blue-900 font-medium"
                      >
                        First Advantage
                      </a>
                      , who will contact your prior employers to confirm the accuracy of your entries. 
                      <span class="font-semibold">
                        Providing false or misleading information will result in immediate exclusion from consideration for this role.
                      </span>
                    </p>
                  </div>
                </div>
              </div>
              <div class="space-y-6">
                <Index each={workExperience()}>
                  {(role, index) => (
                    <RoleForm
                      index={index}
                      role={role()}
                      onFieldChange={handleWorkExperienceChange}
                      onRemove={removeWorkExperience}
                      canRemove={workExperience().length > 1}
                      today={today()}
                      validation={roleValidation()[index]}
                      onValidate={validateRoleField}
                    />
                  )}
                </Index>
              </div>
            </div>

            {/* Description */}
            <div class="space-y-4">
              <h2 class="text-xl font-semibold text-gray-900">Tell us about yourself</h2>
              <div>
                <label for="interest" class="block text-sm font-medium text-gray-700 mb-1">
                  Why PriceWhisperer? <span class="text-red-500 ml-1">*</span>
                </label>
                <textarea
                  id="interest"
                  required
                  rows={6}
                  placeholder="Tell us why you're interested in joining PriceWhisperer and what you hope to contribute..."
                  value={formData().interest}
                  onInput={(e) => handleInputChange('interest', e.currentTarget.value)}
                  class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
                />
              </div>
            </div>

            {/* Submit Button */}
            <div class="flex items-center justify-end space-x-4 pt-4 border-t border-gray-200">
              <button
                type="button"
                onClick={() => window.location.hash = '#careers'}
                class="px-6 py-2 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-50 transition-colors"
                disabled={isSubmitting()}
              >
                Cancel
              </button>
              <button
                type="submit"
                disabled={isSubmitting()}
                class="px-6 py-2 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isSubmitting() ? (
                  <span class="flex items-center">
                    <i class="fa-solid fa-spinner fa-spin mr-2"></i>
                    Submitting...
                  </span>
                ) : (
                  'Submit Application'
                )}
              </button>
            </div>

            {/* reCAPTCHA Notice */}
            <p class="text-xs text-center text-gray-500 mt-4">
              This site is protected by reCAPTCHA and the Google{' '}
              <a href="https://policies.google.com/privacy" target="_blank" rel="noopener noreferrer" class="text-primary hover:underline">
                Privacy Policy
              </a>
              {' '}and{' '}
              <a href="https://policies.google.com/terms" target="_blank" rel="noopener noreferrer" class="text-primary hover:underline">
                Terms of Service
              </a>
              {' '}apply.
            </p>
          </form>
        </div>
      </main>
    </div>
  );
};

export default JobApplicationPage;

