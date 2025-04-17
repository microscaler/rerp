import { Component, createSignal, For, Show } from 'solid-js';
import { getAllTechnologyNames } from '../../data/technologies-data';
import { FormInput, FormCheckbox, Alert } from '../shared';

export interface JobApplicationFormProps {
  jobId?: string;
  jobTitle?: string;
  onClose?: () => void;
}

interface WorkExperience {
  company: string;
  jobTitle: string;
  duration: string;
}

const JobApplicationForm: Component<JobApplicationFormProps> = (props) => {
  const [formData, setFormData] = createSignal({
    firstName: '',
    lastName: '',
    email: '',
    phone: '',
    location: '',
    linkedin: '',
    github: '',
    background: '',
    experience: '',
    interest: '',
    otherTechnologies: '',
  });

  const [selectedTechnologies, setSelectedTechnologies] = createSignal<Set<string>>(new Set());
  const [workExperience, setWorkExperience] = createSignal<WorkExperience[]>([
    { company: '', jobTitle: '', duration: '' },
  ]);
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
    setWorkExperience(prev => [...prev, { company: '', jobTitle: '', duration: '' }]);
  };

  const removeWorkExperience = (index: number) => {
    setWorkExperience(prev => prev.filter((_, i) => i !== index));
  };

  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    setIsSubmitting(true);
    setSubmitError(null);

    // Track form submission
    if (typeof window !== 'undefined' && (window as any).gtag) {
      (window as any).gtag('event', 'job_application_submit', {
        event_category: 'careers',
        event_label: props.jobId || 'general',
        job_title: props.jobTitle || 'general',
      });
    }

    try {
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
      //     workExperience: workExperience().filter(exp => exp.company && exp.jobTitle),
      //     jobId: props.jobId,
      //     jobTitle: props.jobTitle,
      //     description: {
      //       background: formData().background,
      //       experience: formData().experience,
      //       interest: formData().interest,
      //     },
      //   }),
      // });

      setSubmitSuccess(true);
      
      // Reset form after 3 seconds
      setTimeout(() => {
        if (props.onClose) {
          props.onClose();
        }
      }, 3000);
    } catch (error) {
      setSubmitError('Failed to submit application. Please try again or contact us directly.');
      console.error('Application submission error:', error);
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div class="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center p-4 overflow-y-auto">
      <div class="bg-white rounded-lg shadow-xl max-w-4xl w-full max-h-[90vh] overflow-y-auto">
        <div class="sticky top-0 bg-white border-b border-gray-200 px-6 py-4 flex items-center justify-between z-10">
          <h2 class="text-2xl font-bold text-gray-900">
            {props.jobTitle ? `Apply for ${props.jobTitle}` : 'Job Application'}
          </h2>
          <button
            onClick={props.onClose}
            class="text-gray-400 hover:text-gray-600 transition-colors"
            aria-label="Close"
          >
            <i class="fa-solid fa-times text-xl"></i>
          </button>
        </div>

        <form onSubmit={handleSubmit} class="p-6 space-y-6">
          <Show when={submitSuccess()}>
            <Alert
              type="success"
              message="Application submitted successfully!"
              description="We'll review your application and get back to you soon."
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
            <h3 class="text-lg font-semibold text-gray-900 mb-4">Personal Information</h3>
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
            <h3 class="text-lg font-semibold text-gray-900 mb-4">
              Technologies <span class="text-sm font-normal text-gray-500">(Select all that apply)</span>
            </h3>
            <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3 max-h-64 overflow-y-auto p-2 border border-gray-200 rounded-lg">
              <For each={allTechnologies}>
                {(tech) => (
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
                )}
              </For>
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
              <h3 class="text-lg font-semibold text-gray-900">Work Experience</h3>
              <button
                type="button"
                onClick={addWorkExperience}
                class="text-sm text-primary hover:text-blue-700 font-medium flex items-center"
              >
                <i class="fa-solid fa-plus mr-1"></i>
                Add Role
              </button>
            </div>
            <div class="space-y-4">
              <For each={workExperience()}>
                {(exp, index) => (
                  <div class="grid grid-cols-1 md:grid-cols-4 gap-4 p-4 bg-gray-50 rounded-lg relative">
                    <FormInput
                      id={`company-${index()}`}
                      label={`Company ${index() + 1}`}
                      type="text"
                      value={exp.company}
                      onInput={(value) => handleWorkExperienceChange(index(), 'company', value)}
                      class="md:col-span-1"
                    />
                    <FormInput
                      id={`jobTitle-${index()}`}
                      label={`Job Title ${index() + 1}`}
                      type="text"
                      value={exp.jobTitle}
                      onInput={(value) => handleWorkExperienceChange(index(), 'jobTitle', value)}
                      class="md:col-span-1"
                    />
                    <FormInput
                      id={`duration-${index()}`}
                      label={`Duration ${index() + 1}`}
                      type="text"
                      placeholder="e.g., Jan 2020 - Dec 2023"
                      value={exp.duration}
                      onInput={(value) => handleWorkExperienceChange(index(), 'duration', value)}
                      class="md:col-span-1"
                    />
                    <div class="flex items-end md:col-span-1">
                      <Show when={workExperience().length > 1}>
                        <button
                          type="button"
                          onClick={() => removeWorkExperience(index())}
                          class="w-full px-3 py-2 text-sm text-red-600 hover:text-red-700 hover:bg-red-50 rounded-lg transition-colors flex items-center justify-center"
                        >
                          <i class="fa-solid fa-trash mr-1"></i>
                          Remove
                        </button>
                      </Show>
                    </div>
                  </div>
                )}
              </For>
            </div>
          </div>

          {/* Description - Split into multiple fields */}
          <div class="space-y-4">
            <h3 class="text-lg font-semibold text-gray-900">Tell us about yourself</h3>
            <div>
              <label for="background" class="block text-sm font-medium text-gray-700 mb-1">
                Background & Experience <span class="text-red-500 ml-1">*</span>
              </label>
              <textarea
                id="background"
                required
                rows={4}
                placeholder="Describe your professional background, key achievements, and relevant experience..."
                value={formData().background}
                onInput={(e) => handleInputChange('background', e.currentTarget.value)}
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
              />
            </div>
            <div>
              <label for="experience" class="block text-sm font-medium text-gray-700 mb-1">
                Technical Experience <span class="text-red-500 ml-1">*</span>
              </label>
              <textarea
                id="experience"
                required
                rows={4}
                placeholder="Describe your technical skills, projects you've worked on, and technologies you've used..."
                value={formData().experience}
                onInput={(e) => handleInputChange('experience', e.currentTarget.value)}
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
              />
            </div>
            <div>
              <label for="interest" class="block text-sm font-medium text-gray-700 mb-1">
                Why PriceWhisperer? <span class="text-red-500 ml-1">*</span>
              </label>
              <textarea
                id="interest"
                required
                rows={4}
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
              onClick={props.onClose}
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
        </form>
      </div>
    </div>
  );
};

export default JobApplicationForm;
