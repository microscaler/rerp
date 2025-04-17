import { Component, createSignal, onMount } from 'solid-js';
import { updateSEO } from '../utils/seo';
import { BASE_URL } from '../config/constants';
import { executeRecaptcha } from '../utils/recaptcha';
import { saveEmailCapture } from '../utils/supabase';

const FreeTrialForm: Component = () => {
  const [formData, setFormData] = createSignal({
    name: '',
    email: '',
    company: '',
    plan: 'starter',
  });
  const [errors, setErrors] = createSignal<Record<string, string>>({});
  const [isSubmitting, setIsSubmitting] = createSignal(false);
  const [submitStatus, setSubmitStatus] = createSignal<'idle' | 'success' | 'error'>('idle');
  const [errorMessage, setErrorMessage] = createSignal('');

  onMount(() => {
    updateSEO({
      title: 'Start Your Free Trial - PriceWhisperer',
      description: 'Start your 14-day free trial of PriceWhisperer. Get full access to AI-powered trading alerts and options analytics.',
      keywords: 'free trial, PriceWhisperer trial, trading platform trial, start trading',
      canonical: `${BASE_URL}/#free-trial`,
      ogType: 'website',
      ogImage: '/og-image.jpg'
    });
  });

  const validateForm = (): boolean => {
    const newErrors: Record<string, string> = {};
    
    if (!formData().name.trim()) {
      newErrors.name = 'Name is required';
    }
    
    if (!formData().email.trim()) {
      newErrors.email = 'Email is required';
    } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData().email)) {
      newErrors.email = 'Please enter a valid email address';
    }
    
    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleInput = (field: string) => (e: Event) => {
    const target = e.target as HTMLInputElement | HTMLSelectElement;
    setFormData({ ...formData(), [field]: target.value });
    // Clear error for this field when user starts typing
    if (errors()[field]) {
      setErrors({ ...errors(), [field]: '' });
    }
  };

  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    
    if (!validateForm()) {
      return;
    }

    setIsSubmitting(true);
    setSubmitStatus('idle');
    setErrorMessage('');

    try {
      // Execute reCAPTCHA v3
      const recaptchaToken = await executeRecaptcha('submit_free_trial');

      // Track form submission
      if (typeof window !== 'undefined' && (window as any).gtag) {
        (window as any).gtag('event', 'free_trial_form_submit', {
          event_category: 'conversion',
          event_label: formData().plan,
          value: 1
        });
      }

      // Save to Supabase
      await saveEmailCapture({
        email: formData().email,
        source: 'free_trial',
        name: formData().name,
        company: formData().company,
        plan: formData().plan,
        recaptcha_token: recaptchaToken,
      });
      
      setSubmitStatus('success');
      setFormData({ name: '', email: '', company: '', plan: 'starter' });
    } catch (error) {
      console.error('Form submission error:', error);
      setSubmitStatus('error');
      setErrorMessage('Something went wrong. Please try again or contact support.');
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div class="bg-gray-50 flex justify-center pt-24 pb-12 px-4 sm:px-6 lg:px-8">
      <div class="max-w-2xl w-full">
        {/* Header */}
        <div class="text-center mb-8">
          <div class="flex justify-center mb-4">
            <div class="w-16 h-16 bg-primary rounded-full flex items-center justify-center">
              <i class="fa-solid fa-rocket text-white text-2xl"></i>
            </div>
          </div>
          <h1 class="text-4xl font-extrabold text-gray-900 mb-4">Start Your Free 14-Day Trial</h1>
          <p class="text-xl text-gray-600">
            Try the Starter plan free for 14 days • Cancel anytime
          </p>
        </div>

        {/* Form */}
        <div class="bg-white rounded-lg shadow-lg p-8 md:p-12">
          <form onSubmit={handleSubmit} class="space-y-6">
            {submitStatus() === 'success' && (
              <div class="bg-green-50 border border-green-200 text-green-800 px-4 py-3 rounded-lg">
                <div class="flex items-center">
                  <i class="fa-solid fa-check-circle mr-2"></i>
                  <span>Thank you! Your free trial request has been submitted. Check your email for next steps.</span>
                </div>
              </div>
            )}

            {submitStatus() === 'error' && (
              <div class="bg-red-50 border border-red-200 text-red-800 px-4 py-3 rounded-lg">
                <div class="flex items-center">
                  <i class="fa-solid fa-exclamation-circle mr-2"></i>
                  <span>{errorMessage() || 'Failed to submit. Please try again.'}</span>
                </div>
              </div>
            )}

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div>
                <label for="name" class="block text-sm font-medium text-gray-700 mb-2">
                  Full Name <span class="text-red-500">*</span>
                </label>
                <input
                  type="text"
                  id="name"
                  name="name"
                  value={formData().name}
                  onInput={handleInput('name')}
                  class={`w-full px-4 py-3 border rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent ${
                    errors().name ? 'border-red-500' : 'border-gray-300'
                  }`}
                  placeholder="John Doe"
                />
                {errors().name && (
                  <p class="mt-1 text-sm text-red-600">{errors().name}</p>
                )}
              </div>

              <div>
                <label for="email" class="block text-sm font-medium text-gray-700 mb-2">
                  Email Address <span class="text-red-500">*</span>
                </label>
                <input
                  type="email"
                  id="email"
                  name="email"
                  value={formData().email}
                  onInput={handleInput('email')}
                  class={`w-full px-4 py-3 border rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent ${
                    errors().email ? 'border-red-500' : 'border-gray-300'
                  }`}
                  placeholder="your.email@example.com"
                />
                {errors().email && (
                  <p class="mt-1 text-sm text-red-600">{errors().email}</p>
                )}
              </div>
            </div>

            <div>
              <label for="company" class="block text-sm font-medium text-gray-700 mb-2">
                Company (Optional)
              </label>
              <input
                type="text"
                id="company"
                name="company"
                value={formData().company}
                onInput={handleInput('company')}
                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
                placeholder="Your company name"
              />
            </div>

            <div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
              <div class="flex items-start">
                <i class="fa-solid fa-info-circle text-blue-600 mt-1 mr-3"></i>
                <div class="text-sm text-blue-700">
                  <p class="font-semibold mb-1">Free Trial Plan: Starter</p>
                  <p>Your 14-day free trial includes full access to the Starter plan features. We'll never share your data. By starting a trial, you agree to our <a href="#terms-of-service" onClick={(e) => { e.preventDefault(); window.location.hash = '#terms-of-service'; window.scrollTo({ top: 0, behavior: 'instant' }); }} class="text-primary-600 hover:text-primary-500 font-semibold underline">Terms of Service</a> and <a href="#privacy-policy" onClick={(e) => { e.preventDefault(); window.location.hash = '#privacy-policy'; window.scrollTo({ top: 0, behavior: 'instant' }); }} class="text-primary-600 hover:text-primary-500 font-semibold underline">Privacy Policy</a>.</p>
                </div>
              </div>
            </div>

            <div>
              <button
                type="submit"
                disabled={isSubmitting()}
                class="w-full flex items-center justify-center px-6 py-4 border border-transparent text-base font-medium rounded-lg text-white bg-primary hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
              >
                {isSubmitting() ? (
                  <>
                    <i class="fa-solid fa-spinner fa-spin mr-2"></i>
                    Starting Your Trial...
                  </>
                ) : (
                  <>
                    Start Free Trial
                    <i class="fa-solid fa-arrow-right ml-2"></i>
                  </>
                )}
              </button>
            </div>

            <p class="text-center text-sm text-gray-500">
              Already have an account?{' '}
              <a
                href="#sign-in"
                onClick={(e) => {
                  e.preventDefault();
                  window.location.hash = '#sign-in';
                  window.scrollTo({ top: 0, behavior: 'instant' });
                }}
                class="text-primary-600 hover:text-primary-500 font-medium"
              >
                Sign in
              </a>
            </p>
          </form>
        </div>

        {/* Benefits */}
        <div class="mt-12 grid grid-cols-1 md:grid-cols-3 gap-6">
          <div class="text-center">
            <div class="w-12 h-12 bg-primary rounded-lg flex items-center justify-center mx-auto mb-4">
              <i class="fa-solid fa-clock text-white text-xl"></i>
            </div>
            <h3 class="font-semibold text-gray-900 mb-2">14 Days Free</h3>
            <p class="text-sm text-gray-600">Full access to all features for 14 days</p>
          </div>
          <div class="text-center">
            <div class="w-12 h-12 bg-secondary rounded-lg flex items-center justify-center mx-auto mb-4">
              <i class="fa-solid fa-check-circle text-white text-xl"></i>
            </div>
            <h3 class="font-semibold text-gray-900 mb-2">Cancel Anytime</h3>
            <p class="text-sm text-gray-600">No commitment, no questions asked</p>
          </div>
          <div class="text-center">
            <div class="w-12 h-12 bg-accent rounded-lg flex items-center justify-center mx-auto mb-4">
              <i class="fa-solid fa-headset text-white text-xl"></i>
            </div>
            <h3 class="font-semibold text-gray-900 mb-2">24/7 Support</h3>
            <p class="text-sm text-gray-600">Get help whenever you need it</p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default FreeTrialForm;

