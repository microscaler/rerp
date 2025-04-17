import { Component, createSignal, onMount, Show } from 'solid-js';
import { updateSEO } from '../utils/seo';
import { BASE_URL } from '../config/constants';
import { InfoBox } from './blog/components';
import { AuthButton } from './auth';
import { 
  sendDualOTP,
  verifyEmailOTPAndTrack,
  verifyPhoneOTPAndTrack,
  checkDualVerificationStatus,
  signInWithOAuth,
  signInWithSAML,
  getSession,
  onAuthStateChange
} from '../utils/supabase-auth';

type AuthMode = 'email-phone' | 'oauth';
type DualVerificationStep = 'input' | 'verify-email' | 'verify-phone' | 'verify-both';

const SignInPage: Component = () => {
  const [authMode, setAuthMode] = createSignal<AuthMode>('email-phone');
  const [dualStep, setDualStep] = createSignal<DualVerificationStep>('input');
  const [isLoading, setIsLoading] = createSignal(false);
  const [error, setError] = createSignal<string | null>(null);
  
  // Dual verification form (email + phone)
  const [email, setEmail] = createSignal('');
  const [phone, setPhone] = createSignal('');
  const [emailOtp, setEmailOtp] = createSignal('');
  const [phoneOtp, setPhoneOtp] = createSignal('');
  const [emailVerified, setEmailVerified] = createSignal(false);
  const [phoneVerified, setPhoneVerified] = createSignal(false);

  onMount(() => {
    updateSEO({
      title: 'Sign In - PriceWhisperer',
      description: 'Sign in to PriceWhisperer using email, phone, Google, Apple, Facebook, or SAML SSO to access your trading dashboard and alerts.',
      keywords: 'sign in, login, PriceWhisperer login, trading platform access',
      canonical: `${BASE_URL}/#sign-in`,
      ogType: 'website',
      ogImage: '/og-image.jpg'
    });

    // Check for existing session
    getSession().then(session => {
      if (session) {
        // Redirect to dashboard or home
        window.location.hash = '#';
        window.location.reload();
      }
    });

    // Listen for auth state changes (OAuth redirects)
    onAuthStateChange((event, session) => {
      if (event === 'SIGNED_IN' && session) {
        window.location.hash = '#';
        window.location.reload();
      }
    });
  });

  const handleDualVerificationSubmit = async (e: Event) => {
    e.preventDefault();
    setIsLoading(true);
    setError(null);

    try {
      if (dualStep() === 'input') {
        // Step 1: Send OTP to both email and phone
        await sendDualOTP(email(), phone());
        setDualStep('verify-both');
        setError(null);
      } else if (dualStep() === 'verify-both') {
        // Step 2: Verify both OTPs
        // User can verify email and phone in any order
        if (!emailVerified() && emailOtp()) {
          // Verify email first
          try {
            await verifyEmailOTP(email(), emailOtp());
            setEmailVerified(true);
            setError(null);
          } catch (err: any) {
            setError(`Email verification failed: ${err.message}`);
            return;
          }
        }
        
        if (!phoneVerified() && phoneOtp()) {
          // Verify phone
          try {
            await verifyPhoneOTP(phone(), phoneOtp());
            setPhoneVerified(true);
            setError(null);
          } catch (err: any) {
            setError(`Phone verification failed: ${err.message}`);
            return;
          }
        }

        // Check if both are verified
        if (emailVerified() && phoneVerified()) {
          // Both verified - redirect
          window.location.hash = '#';
          window.location.reload();
        } else if (emailVerified() || phoneVerified()) {
          // One verified, waiting for the other
          setError(null);
        }
      }
    } catch (err: any) {
      setError(err.message || 'Failed to verify. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  // Helper function to verify email OTP separately
  const handleEmailOtpVerify = async () => {
    if (!emailOtp()) return;
    
    setIsLoading(true);
    setError(null);
    
    try {
      const bothVerified = await verifyEmailOTPAndTrack(email(), emailOtp());
      setEmailVerified(true);
      setError(null);
      
      // If both are now verified, redirect
      if (bothVerified) {
        window.location.hash = '#';
        window.location.reload();
      } else {
        // Check phone verification status
        const status = await checkDualVerificationStatus();
        setPhoneVerified(status.phoneVerified);
        if (status.bothVerified) {
          window.location.hash = '#';
          window.location.reload();
        }
      }
    } catch (err: any) {
      setError(`Email verification failed: ${err.message}`);
    } finally {
      setIsLoading(false);
    }
  };

  // Helper function to verify phone OTP separately
  const handlePhoneOtpVerify = async () => {
    if (!phoneOtp()) return;
    
    setIsLoading(true);
    setError(null);
    
    try {
      const bothVerified = await verifyPhoneOTPAndTrack(phone(), phoneOtp());
      setPhoneVerified(true);
      setError(null);
      
      // If both are now verified, redirect
      if (bothVerified) {
        window.location.hash = '#';
        window.location.reload();
      } else {
        // Check email verification status
        const status = await checkDualVerificationStatus();
        setEmailVerified(status.emailVerified);
        if (status.bothVerified) {
          window.location.hash = '#';
          window.location.reload();
        }
      }
    } catch (err: any) {
      setError(`Phone verification failed: ${err.message}`);
    } finally {
      setIsLoading(false);
    }
  };

  const handleOAuthSignIn = async (provider: 'google' | 'apple' | 'facebook') => {
    setIsLoading(true);
    setError(null);

    try {
      await signInWithOAuth(provider);
      // User will be redirected to OAuth provider
    } catch (err: any) {
      setError(err.message || `Failed to sign in with ${provider}. Please try again.`);
      setIsLoading(false);
    }
  };

  const handleSAMLSignIn = async () => {
    // SAML is inactive for now
    setError('SAML SSO is not yet available. Please use another sign-in method.');
  };

  return (
    <div class="bg-gray-50 flex justify-center pt-24 pb-12 px-4 sm:px-6 lg:px-8">
      <div class="max-w-md w-full space-y-8">
        {/* Header */}
        <div class="text-center mb-8">
          <div class="flex justify-center mb-4">
            <div class="w-16 h-16 bg-primary rounded-full flex items-center justify-center">
              <i class="fa-solid fa-chart-line text-white text-2xl"></i>
            </div>
          </div>
          <h1 class="text-3xl font-bold text-gray-900 mb-2">Sign In</h1>
          <p class="text-xl text-gray-600">
            Access your trading dashboard, alerts, and analytics
          </p>
        </div>

        {/* Auth Mode Tabs */}
        <div class="bg-white rounded-lg shadow-lg p-8 space-y-6">
          <div class="flex border-b border-gray-200">
            <button
              onClick={() => {
                setAuthMode('email-phone');
                setError(null);
                setDualStep('input');
                setEmailVerified(false);
                setPhoneVerified(false);
              }}
              class={`flex-1 py-2 px-4 text-sm font-medium text-center border-b-2 transition-colors ${
                authMode() === 'email-phone'
                  ? 'border-primary text-primary'
                  : 'border-transparent text-gray-500 hover:text-gray-700'
              }`}
            >
              Email & Phone
            </button>
            <button
              onClick={() => {
                setAuthMode('oauth');
                setError(null);
              }}
              class={`flex-1 py-2 px-4 text-sm font-medium text-center border-b-2 transition-colors ${
                authMode() === 'oauth'
                  ? 'border-primary text-primary'
                  : 'border-transparent text-gray-500 hover:text-gray-700'
              }`}
            >
              Social
            </button>
          </div>

          {/* Error Message */}
          <Show when={error()}>
            <div class="bg-red-50 border border-red-200 text-red-800 px-4 py-3 rounded-lg text-sm">
              <div class="flex items-center">
                <i class="fa-solid fa-exclamation-circle mr-2"></i>
                <span>{error()}</span>
              </div>
            </div>
          </Show>

          {/* Dual Verification: Email & Phone */}
          <Show when={authMode() === 'email-phone'}>
            <form onSubmit={handleDualVerificationSubmit} class="space-y-4">
              <Show when={dualStep() === 'input'}>
                <div>
                  <label for="email" class="block text-sm font-medium text-gray-700 mb-2">
                    Email Address
                  </label>
                  <input
                    type="email"
                    id="email"
                    value={email()}
                    onInput={(e) => setEmail(e.currentTarget.value)}
                    required
                    class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                    placeholder="your.email@example.com"
                  />
                </div>
                <div>
                  <label for="phone" class="block text-sm font-medium text-gray-700 mb-2">
                    Phone Number
                  </label>
                  <input
                    type="tel"
                    id="phone"
                    value={phone()}
                    onInput={(e) => setPhone(e.currentTarget.value)}
                    required
                    class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                    placeholder="+1234567890"
                  />
                  <p class="mt-1 text-xs text-gray-500">
                    We'll send verification codes to both your email and phone
                  </p>
                </div>
                <button
                  type="submit"
                  disabled={isLoading()}
                  class="w-full bg-primary text-white py-3 rounded-lg hover:bg-blue-700 font-semibold transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {isLoading() ? (
                    <span class="flex items-center justify-center">
                      <i class="fa-solid fa-spinner fa-spin mr-2"></i>
                      Sending Codes...
                    </span>
                  ) : (
                    'Send Verification Codes'
                  )}
                </button>
              </Show>
              
              <Show when={dualStep() === 'verify-both'}>
                <div class="space-y-6">
                  <div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
                    <p class="text-sm text-blue-700">
                      <i class="fa-solid fa-info-circle mr-2"></i>
                      Verification codes have been sent to both your email and phone. Please verify both to sign in.
                    </p>
                  </div>

                  {/* Email Verification */}
                  <div class={`border-2 rounded-lg p-4 ${emailVerified() ? 'border-green-300 bg-green-50' : 'border-gray-200'}`}>
                    <div class="flex items-center justify-between mb-2">
                      <label for="email-otp" class="block text-sm font-medium text-gray-700">
                        Email Verification Code
                      </label>
                      {emailVerified() && (
                        <span class="text-green-600 text-sm">
                          <i class="fa-solid fa-check-circle mr-1"></i>
                          Verified
                        </span>
                      )}
                    </div>
                    <div class="flex space-x-2">
                      <input
                        type="text"
                        id="email-otp"
                        value={emailOtp()}
                        onInput={(e) => setEmailOtp(e.currentTarget.value)}
                        disabled={emailVerified()}
                        maxLength={6}
                        class="flex-1 px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary text-center text-xl tracking-widest disabled:bg-gray-100"
                        placeholder="000000"
                      />
                      <button
                        type="button"
                        onClick={handleEmailOtpVerify}
                        disabled={isLoading() || emailVerified() || !emailOtp()}
                        class="px-4 py-3 bg-primary text-white rounded-lg hover:bg-blue-700 font-semibold transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                      >
                        {emailVerified() ? (
                          <i class="fa-solid fa-check"></i>
                        ) : (
                          'Verify'
                        )}
                      </button>
                    </div>
                    <p class="mt-1 text-xs text-gray-500">
                      Check your email: {email()}
                    </p>
                  </div>

                  {/* Phone Verification */}
                  <div class={`border-2 rounded-lg p-4 ${phoneVerified() ? 'border-green-300 bg-green-50' : 'border-gray-200'}`}>
                    <div class="flex items-center justify-between mb-2">
                      <label for="phone-otp" class="block text-sm font-medium text-gray-700">
                        Phone Verification Code
                      </label>
                      {phoneVerified() && (
                        <span class="text-green-600 text-sm">
                          <i class="fa-solid fa-check-circle mr-1"></i>
                          Verified
                        </span>
                      )}
                    </div>
                    <div class="flex space-x-2">
                      <input
                        type="text"
                        id="phone-otp"
                        value={phoneOtp()}
                        onInput={(e) => setPhoneOtp(e.currentTarget.value)}
                        disabled={phoneVerified()}
                        maxLength={6}
                        class="flex-1 px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary text-center text-xl tracking-widest disabled:bg-gray-100"
                        placeholder="000000"
                      />
                      <button
                        type="button"
                        onClick={handlePhoneOtpVerify}
                        disabled={isLoading() || phoneVerified() || !phoneOtp()}
                        class="px-4 py-3 bg-primary text-white rounded-lg hover:bg-blue-700 font-semibold transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                      >
                        {phoneVerified() ? (
                          <i class="fa-solid fa-check"></i>
                        ) : (
                          'Verify'
                        )}
                      </button>
                    </div>
                    <p class="mt-1 text-xs text-gray-500">
                      Check your phone: {phone()}
                    </p>
                  </div>

                  {/* Resend Codes */}
                  <div class="flex space-x-3">
                    <button
                      type="button"
                      onClick={() => {
                        setDualStep('input');
                        setEmailOtp('');
                        setPhoneOtp('');
                        setEmailVerified(false);
                        setPhoneVerified(false);
                        setError(null);
                      }}
                      class="flex-1 border border-gray-300 text-gray-700 py-3 rounded-lg hover:bg-gray-50 font-semibold transition-colors"
                    >
                      Change Email/Phone
                    </button>
                    <button
                      type="button"
                      onClick={async () => {
                        setIsLoading(true);
                        setError(null);
                        try {
                          await sendDualOTP(email(), phone());
                          setEmailOtp('');
                          setPhoneOtp('');
                          setEmailVerified(false);
                          setPhoneVerified(false);
                        } catch (err: any) {
                          setError(err.message || 'Failed to resend codes');
                        } finally {
                          setIsLoading(false);
                        }
                      }}
                      disabled={isLoading()}
                      class="flex-1 border border-primary text-primary py-3 rounded-lg hover:bg-blue-50 font-semibold transition-colors disabled:opacity-50"
                    >
                      Resend Codes
                    </button>
                  </div>
                </div>
              </Show>
            </form>
          </Show>

          {/* OAuth Providers */}
          <Show when={authMode() === 'oauth'}>
            <div class="space-y-3">
              <AuthButton 
                provider="google" 
                onClick={() => handleOAuthSignIn('google')}
                disabled={isLoading()}
              />
              <AuthButton 
                provider="apple" 
                onClick={() => handleOAuthSignIn('apple')}
                disabled={isLoading()}
              />
              <AuthButton 
                provider="facebook" 
                onClick={() => handleOAuthSignIn('facebook')}
                disabled={isLoading()}
              />

              {/* Divider */}
              <div class="relative my-6">
                <div class="absolute inset-0 flex items-center">
                  <div class="w-full border-t border-gray-300"></div>
                </div>
                <div class="relative flex justify-center text-sm">
                  <span class="px-2 bg-white text-gray-500">Enterprise customers</span>
                </div>
              </div>

              {/* SAML SSO (Inactive) */}
              <AuthButton 
                provider="saml" 
                onClick={handleSAMLSignIn}
                disabled={true}
              />

              {/* Info Box */}
              <InfoBox variant="blue">
                <div class="flex">
                  <div class="flex-shrink-0">
                    <i class="fa-solid fa-info-circle text-blue-600"></i>
                  </div>
                  <div class="ml-3">
                    <p class="text-sm text-blue-700">
                      <strong>SAML SSO</strong> is available for Enterprise plans. Contact your administrator for access.
                    </p>
                  </div>
                </div>
              </InfoBox>
            </div>
          </Show>
        </div>

        {/* Footer Links */}
        <div class="text-center space-y-2">
          <p class="text-sm text-gray-600">
            Don't have an account?{' '}
            <a
              href="#free-trial"
              onClick={(e) => {
                e.preventDefault();
                window.location.hash = '#free-trial';
                window.scrollTo({ top: 0, behavior: 'instant' });
              }}
              class="font-medium text-primary-600 hover:text-primary-500"
            >
              Start your free trial
            </a>
          </p>
          <p class="text-xs text-gray-500">
            By signing in, you agree to our{' '}
            <a 
              href="#terms-of-service" 
              onClick={(e) => {
                e.preventDefault();
                window.location.hash = '#terms-of-service';
                window.scrollTo({ top: 0, behavior: 'instant' });
              }}
              class="text-primary-600 hover:text-primary-500"
            >
              Terms of Service
            </a>
            {' '}and{' '}
            <a 
              href="#privacy-policy"
              onClick={(e) => {
                e.preventDefault();
                window.location.hash = '#privacy-policy';
                window.scrollTo({ top: 0, behavior: 'instant' });
              }}
              class="text-primary-600 hover:text-primary-500"
            >
              Privacy Policy
            </a>
          </p>
        </div>
      </div>
    </div>
  );
};

export default SignInPage;
