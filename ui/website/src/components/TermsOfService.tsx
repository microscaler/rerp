import { Component, onMount } from 'solid-js';
import { updateSEO } from '../utils/seo';
import { BASE_URL } from '../config/constants';
import LegalPagesSidebar from './legal/LegalPagesSidebar';
import LegalPageLayout from './legal/LegalPageLayout';

const TermsOfService: Component = () => {
  const lastUpdated = new Date().toLocaleDateString('en-GB', { year: 'numeric', month: 'long', day: 'numeric' });

  onMount(() => {
    updateSEO({
      title: 'Terms of Service - PriceWhisperer',
      description: 'Terms of Service for PriceWhisperer trading platform. Read our terms and conditions, user agreements, and legal disclaimers.',
      keywords: 'terms of service, terms and conditions, user agreement, legal, PriceWhisperer',
      canonical: `${BASE_URL}/#terms-of-service`,
      ogType: 'website',
      ogImage: '/og-image.jpg'
    });
  });

  return (
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div class="grid grid-cols-1 lg:grid-cols-12 gap-8">
        {/* Main Content */}
        <div class="lg:col-span-8">
          <LegalPageLayout
            header={{
              title: 'Terms of Service',
              lastUpdated: lastUpdated,
              icon: 'fa-solid fa-file-contract',
              iconBgColor: 'bg-primary'
            }}
            showBackLink={true}
          >
            <div class="space-y-8">
            
            {/* Introduction */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">1. Introduction</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                Welcome to PriceWhisperer ("we," "our," or "us"). These Terms of Service ("Terms") govern your access to and use of the PriceWhisperer platform, including our website, mobile applications, and related services (collectively, the "Service").
              </p>
              <p class="text-gray-700 leading-relaxed mb-4">
                By accessing or using our Service, you agree to be bound by these Terms. If you do not agree to these Terms, you must not access or use the Service.
              </p>
              <p class="text-gray-700 leading-relaxed">
                These Terms constitute a legally binding agreement between you and PriceWhisperer. Please read them carefully.
              </p>
            </section>

            {/* Acceptance of Terms */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">2. Acceptance of Terms</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                By creating an account, accessing, or using the Service, you acknowledge that:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>You have read, understood, and agree to be bound by these Terms</li>
                <li>You are at least 18 years of age and have the legal capacity to enter into this agreement</li>
                <li>You have completed our Financial Trading Education (FTE) program and understand the risks associated with trading</li>
                <li>You understand that trading involves substantial risk of loss and is not suitable for all investors</li>
                <li>You will comply with all applicable laws and regulations in your jurisdiction</li>
              </ul>
            </section>

            {/* Nature of Service */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">3. Nature of Service</h2>
              <div class="bg-yellow-50 border-l-4 border-yellow-400 p-4 mb-4">
                <p class="text-gray-800 font-semibold mb-2">Important: Not Financial Advice</p>
                <p class="text-gray-700 leading-relaxed">
                  PriceWhisperer is a technology platform that provides trading alerts, market analysis tools, and educational resources. We do NOT provide financial advice, investment recommendations, or personalised investment guidance.
                </p>
              </div>
              <p class="text-gray-700 leading-relaxed mb-4">
                Our Service includes:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Automated pattern detection and trading alerts based on technical analysis</li>
                <li>Market data aggregation and analysis tools</li>
                <li>Options strategy suggestions based on market conditions</li>
                <li>Educational content and resources</li>
                <li>Integration with third-party trading platforms for trade execution</li>
              </ul>
              <p class="text-gray-700 leading-relaxed">
                All trading decisions, including whether to act on any alert or suggestion provided by our Service, are made solely by you. We do not execute trades on your behalf or provide investment advice.
              </p>
            </section>

            {/* UK FCA Compliance */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">4. UK Financial Conduct Authority (FCA) Compliance</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                PriceWhisperer operates in compliance with UK FCA regulations. In accordance with FCA rules:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>We are NOT authorised or regulated by the FCA to provide investment advice</li>
                <li>We do NOT provide personalised recommendations or investment advice</li>
                <li>Our Service provides information and alerts only - you make all trading decisions independently</li>
                <li>Any trading positions you take are your sole responsibility</li>
              </ul>
              <p class="text-gray-700 leading-relaxed mb-4">
                By using our Service, you acknowledge that:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>You understand that PriceWhisperer is not providing regulated financial services</li>
                <li>You have not received and will not rely on any investment advice from PriceWhisperer</li>
                <li>You will make your own independent assessment of any trading opportunity</li>
                <li>You will seek independent financial advice if you are unsure about any trading decision</li>
              </ul>
            </section>

            {/* Educational Requirements */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">5. Financial Trading Education (FTE) Requirement</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                Before using our Service to identify or execute trading opportunities, you must:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Complete our comprehensive Financial Trading Education (FTE) program</li>
                <li>Demonstrate understanding of trading risks, including but not limited to:
                  <ul class="list-circle list-inside ml-6 mt-2 space-y-1">
                    <li>Market volatility and price movements</li>
                    <li>Options trading risks (including unlimited loss potential for certain strategies)</li>
                    <li>Leverage and margin requirements</li>
                    <li>Liquidity risks</li>
                    <li>Counterparty risks</li>
                    <li>Regulatory and tax implications</li>
                  </ul>
                </li>
                <li>Understand the specific risks associated with each type of trading platform and instrument</li>
                <li>Confirm that you have the financial resources and risk tolerance to engage in trading activities</li>
              </ul>
              <p class="text-gray-700 leading-relaxed">
                By proceeding to use our Service for trading purposes, you represent and warrant that you have completed the FTE program and understand all associated risks.
              </p>
            </section>

            {/* Risk Disclosure */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">6. Risk Disclosure</h2>
              <div class="bg-red-50 border-l-4 border-red-500 p-4 mb-4">
                <p class="text-red-800 font-semibold mb-2">Trading Risk Warning</p>
                <p class="text-red-700 leading-relaxed mb-2">
                  Trading in financial instruments, including stocks, options, and derivatives, involves substantial risk of loss. You may lose some or all of your invested capital.
                </p>
                <p class="text-red-700 leading-relaxed">
                  Past performance is not indicative of future results. No guarantee of profits is made or implied.
                </p>
              </div>
              <p class="text-gray-700 leading-relaxed mb-4">
                You acknowledge and agree that:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>All trading involves risk, and you may lose money</li>
                <li>Options trading can result in unlimited losses for certain strategies</li>
                <li>Market conditions can change rapidly, affecting the value of positions</li>
                <li>Technical analysis and pattern recognition are not guarantees of future performance</li>
                <li>Alerts and suggestions from our Service are based on historical patterns and may not be accurate</li>
                <li>You should never trade with money you cannot afford to lose</li>
                <li>You should seek independent financial advice before making trading decisions</li>
              </ul>
            </section>

            {/* No Liability for Trading Decisions */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">7. No Liability for Trading Decisions or Positions</h2>
              <div class="bg-red-50 border-l-4 border-red-500 p-4 mb-4">
                <p class="text-red-800 font-semibold mb-2">Important Limitation of Liability</p>
                <p class="text-red-700 leading-relaxed">
                  PriceWhisperer accepts NO responsibility, liability, or accountability for any trading decisions you make or any trading positions you take, whether based on our Service or otherwise.
                </p>
              </div>
              <p class="text-gray-700 leading-relaxed mb-4">
                To the fullest extent permitted by law, you agree that:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>PriceWhisperer, its directors, officers, employees, agents, and affiliates shall not be liable for any losses, damages, or costs arising from:
                  <ul class="list-circle list-inside ml-6 mt-2 space-y-1">
                    <li>Any trading decision you make, whether or not based on our Service</li>
                    <li>Any trading position you take or execute</li>
                    <li>Any losses incurred from trading activities</li>
                    <li>Any errors, delays, or inaccuracies in our alerts, data, or analysis</li>
                    <li>Any technical failures or interruptions to our Service</li>
                    <li>Any market movements or volatility</li>
                  </ul>
                </li>
                <li>You are solely responsible for all trading decisions and positions</li>
                <li>You bear all risk of loss associated with your trading activities</li>
                <li>PriceWhisperer does not guarantee the accuracy, completeness, or timeliness of any information provided</li>
                <li>PriceWhisperer does not warrant that any trading opportunity will be profitable</li>
              </ul>
              <p class="text-gray-700 leading-relaxed mb-4">
                This limitation of liability applies regardless of whether the claim arises in contract, tort (including negligence), breach of statutory duty, or otherwise, and even if PriceWhisperer has been advised of the possibility of such losses.
              </p>
            </section>

            {/* User Responsibilities */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">8. User Responsibilities</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                You are responsible for:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Maintaining the confidentiality of your account credentials</li>
                <li>All activities that occur under your account</li>
                <li>Ensuring your use of the Service complies with all applicable laws and regulations</li>
                <li>Verifying the accuracy of any information before making trading decisions</li>
                <li>Conducting your own due diligence on any trading opportunity</li>
                <li>Managing your own risk and position sizing</li>
                <li>Seeking independent financial, legal, and tax advice as appropriate</li>
                <li>Complying with all terms and conditions of any third-party trading platforms you use</li>
              </ul>
            </section>

            {/* Account Terms */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">9. Account Terms</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                To use certain features of our Service, you must create an account. You agree to:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Provide accurate, current, and complete information during registration</li>
                <li>Maintain and update your information to keep it accurate, current, and complete</li>
                <li>Not share your account credentials with any third party</li>
                <li>Notify us immediately of any unauthorised access to your account</li>
                <li>Be responsible for all activities under your account</li>
              </ul>
              <p class="text-gray-700 leading-relaxed">
                We reserve the right to suspend or terminate your account if you violate these Terms or engage in fraudulent, abusive, or illegal activities.
              </p>
            </section>

            {/* Subscription and Payment */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">10. Subscription and Payment</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                Our Service is offered on a subscription basis. By subscribing, you agree to:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Pay all fees associated with your chosen subscription plan</li>
                <li>Provide accurate payment information</li>
                <li>Authorise us to charge your payment method for subscription fees</li>
                <li>Understand that subscription fees are non-refundable except as required by law or as specified in our refund policy</li>
              </ul>
              <p class="text-gray-700 leading-relaxed mb-4">
                Subscription terms:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Subscriptions automatically renew unless cancelled before the renewal date</li>
                <li>You may cancel your subscription at any time</li>
                <li>Cancellation takes effect at the end of your current billing period</li>
                <li>We reserve the right to change our pricing with 30 days' notice</li>
              </ul>
            </section>

            {/* Intellectual Property */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">11. Intellectual Property</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                All content, features, and functionality of the Service, including but not limited to text, graphics, logos, software, and algorithms, are owned by PriceWhisperer or its licensors and are protected by copyright, trademark, and other intellectual property laws.
              </p>
              <p class="text-gray-700 leading-relaxed">
                You may not copy, modify, distribute, sell, or lease any part of our Service without our prior written consent.
              </p>
            </section>

            {/* Third-Party Services */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">12. Third-Party Services</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                Our Service may integrate with third-party trading platforms, brokers, and other services. You acknowledge that:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>We are not responsible for the availability, accuracy, or functionality of third-party services</li>
                <li>Your use of third-party services is subject to their respective terms and conditions</li>
                <li>We do not endorse or guarantee any third-party service</li>
                <li>Any disputes with third-party services must be resolved directly with that service provider</li>
              </ul>
            </section>

            {/* Disclaimers */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">13. Disclaimers</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                THE SERVICE IS PROVIDED "AS IS" AND "AS AVAILABLE" WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, OR NON-INFRINGEMENT</li>
                <li>WARRANTIES THAT THE SERVICE WILL BE UNINTERRUPTED, SECURE, OR ERROR-FREE</li>
                <li>WARRANTIES REGARDING THE ACCURACY, RELIABILITY, OR TIMELINESS OF ANY INFORMATION PROVIDED</li>
                <li>WARRANTIES THAT ANY TRADING OPPORTUNITY WILL BE PROFITABLE</li>
              </ul>
              <p class="text-gray-700 leading-relaxed">
                We do not warrant that the Service will meet your requirements or that any errors will be corrected.
              </p>
            </section>

            {/* Limitation of Liability */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">14. Limitation of Liability</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                TO THE FULLEST EXTENT PERMITTED BY APPLICABLE LAW, IN NO EVENT SHALL PRICEWHISPERER, ITS DIRECTORS, OFFICERS, EMPLOYEES, AGENTS, OR AFFILIATES BE LIABLE FOR:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>ANY INDIRECT, INCIDENTAL, SPECIAL, CONSEQUENTIAL, OR PUNITIVE DAMAGES</li>
                <li>ANY LOSS OF PROFITS, REVENUE, DATA, OR USE</li>
                <li>ANY TRADING LOSSES OR FINANCIAL LOSSES OF ANY KIND</li>
                <li>ANY DAMAGES ARISING FROM YOUR USE OF OR INABILITY TO USE THE SERVICE</li>
              </ul>
              <p class="text-gray-700 leading-relaxed mb-4">
                Our total liability to you for all claims arising from or related to the Service shall not exceed the amount you paid to us in the 12 months preceding the claim.
              </p>
              <p class="text-gray-700 leading-relaxed">
                Nothing in these Terms excludes or limits our liability for death or personal injury caused by our negligence, fraud, or any other liability that cannot be excluded or limited by law.
              </p>
            </section>

            {/* Indemnification */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">15. Indemnification</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                You agree to indemnify, defend, and hold harmless PriceWhisperer, its directors, officers, employees, agents, and affiliates from and against any and all claims, damages, losses, liabilities, costs, and expenses (including reasonable legal fees) arising from:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Your use of the Service</li>
                <li>Your violation of these Terms</li>
                <li>Your violation of any law or regulation</li>
                <li>Your trading decisions and activities</li>
                <li>Any claims by third parties related to your use of the Service</li>
              </ul>
            </section>

            {/* Termination */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">16. Termination</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                We may suspend or terminate your access to the Service at any time, with or without cause or notice, for any reason including:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Violation of these Terms</li>
                <li>Fraudulent, abusive, or illegal activity</li>
                <li>Non-payment of subscription fees</li>
                <li>Any other reason we deem necessary to protect the Service or other users</li>
              </ul>
              <p class="text-gray-700 leading-relaxed">
                You may terminate your account at any time by cancelling your subscription. Upon termination, your right to use the Service will immediately cease.
              </p>
            </section>

            {/* Governing Law */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">17. Governing Law and Jurisdiction</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                These Terms shall be governed by and construed in accordance with the laws of England and Wales.
              </p>
              <p class="text-gray-700 leading-relaxed">
                Any disputes arising from or related to these Terms or the Service shall be subject to the exclusive jurisdiction of the courts of England and Wales.
              </p>
            </section>

            {/* Changes to Terms */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">18. Changes to Terms</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                We reserve the right to modify these Terms at any time. We will notify you of material changes by:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Posting the updated Terms on our website</li>
                <li>Sending an email to the address associated with your account</li>
                <li>Displaying a notice within the Service</li>
              </ul>
              <p class="text-gray-700 leading-relaxed">
                Your continued use of the Service after such modifications constitutes your acceptance of the updated Terms. If you do not agree to the modified Terms, you must stop using the Service.
              </p>
            </section>

            {/* Contact Information */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">19. Contact Information</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                If you have any questions about these Terms, please contact us at:
              </p>
              <div class="bg-gray-50 p-4 rounded-lg">
                <p class="text-gray-700 mb-2"><strong>Email:</strong> legal@pricewhisperer.ai</p>
                <p class="text-gray-700 mb-2"><strong>Address:</strong> [Your Company Address]</p>
                <p class="text-gray-700"><strong>Phone:</strong> [Your Phone Number]</p>
              </div>
            </section>

            {/* Acknowledgment */}
            <section class="bg-blue-50 border-l-4 border-blue-500 p-6 rounded-lg">
              <h2 class="text-2xl font-bold text-gray-900 mb-4">20. Acknowledgment</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                BY USING THE SERVICE, YOU ACKNOWLEDGE THAT YOU HAVE READ, UNDERSTOOD, AND AGREE TO BE BOUND BY THESE TERMS OF SERVICE.
              </p>
              <p class="text-gray-700 leading-relaxed mb-4">
                YOU FURTHER ACKNOWLEDGE THAT:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 ml-4">
                <li>PriceWhisperer does not provide financial advice</li>
                <li>All trading decisions are made solely by you</li>
                <li>You bear all risk and responsibility for your trading activities</li>
                <li>PriceWhisperer accepts no liability for any trading losses or positions</li>
                <li>You have completed the FTE program and understand all associated risks</li>
              </ul>
            </section>
            </div>
          </LegalPageLayout>
        </div>
        
        {/* Sidebar - Legal Pages */}
        <div class="lg:col-span-4 lg:sticky lg:top-8 lg:self-start">
          <LegalPagesSidebar currentPage="terms-of-service" />
        </div>
      </div>
    </div>
  );
};

export default TermsOfService;

