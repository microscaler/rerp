import { Component, onMount } from 'solid-js';
import { updateSEO } from '../utils/seo';
import { BASE_URL } from '../config/constants';
import LegalPagesSidebar from './legal/LegalPagesSidebar';
import LegalPageLayout from './legal/LegalPageLayout';

const PrivacyPolicy: Component = () => {
  const lastUpdated = new Date().toLocaleDateString('en-GB', { year: 'numeric', month: 'long', day: 'numeric' });

  onMount(() => {
    updateSEO({
      title: 'Privacy Policy - PriceWhisperer',
      description: 'Privacy Policy for PriceWhisperer. Learn how we collect, use, and protect your personal information and trading data.',
      keywords: 'privacy policy, data protection, GDPR, privacy, data security, PriceWhisperer',
      canonical: `${BASE_URL}/#privacy-policy`,
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
              title: 'Privacy Policy',
              lastUpdated: lastUpdated,
              icon: 'fa-solid fa-shield-halved',
              iconBgColor: 'bg-primary'
            }}
            showBackLink={true}
          >
            <div class="space-y-8">
            
            {/* Introduction */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">1. Introduction</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                PriceWhisperer ("we," "our," or "us") is committed to protecting your privacy and personal information. This Privacy Policy explains how we collect, use, disclose, and safeguard your information when you use our Service.
              </p>
              <p class="text-gray-700 leading-relaxed mb-4">
                This Privacy Policy applies to all users of the PriceWhisperer platform, including our website, mobile applications, and related services (collectively, the "Service").
              </p>
              <p class="text-gray-700 leading-relaxed">
                By using our Service, you consent to the collection and use of information in accordance with this Privacy Policy. If you do not agree with our policies and practices, please do not use our Service.
              </p>
            </section>

            {/* Information We Collect */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">2. Information We Collect</h2>
              
              <h3 class="text-xl font-semibold text-gray-900 mb-3 mt-6">2.1 Personal Information</h3>
              <p class="text-gray-700 leading-relaxed mb-4">
                We collect personal information that you provide directly to us, including:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li><strong>Account Information:</strong> Name, email address, phone number, company name, and password</li>
                <li><strong>Payment Information:</strong> Credit card details, billing address, and payment history (processed through secure third-party payment processors)</li>
                <li><strong>Profile Information:</strong> Trading preferences, risk tolerance, and account settings</li>
                <li><strong>Communication Data:</strong> Correspondence with our support team, feedback, and survey responses</li>
              </ul>

              <h3 class="text-xl font-semibold text-gray-900 mb-3 mt-6">2.2 Trading and Financial Data</h3>
              <p class="text-gray-700 leading-relaxed mb-4">
                When you connect your trading accounts or use our Service, we may collect:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Trading account information (account numbers, broker details)</li>
                <li>Market data and trading activity (positions, orders, transactions)</li>
                <li>Performance metrics and analytics</li>
                <li>Alert preferences and trading strategies</li>
              </ul>
              <div class="bg-blue-50 border-l-4 border-blue-400 p-4 mb-4">
                <p class="text-blue-800 font-semibold mb-2">Data Security</p>
                <p class="text-blue-700 leading-relaxed">
                  All trading and financial data is encrypted in transit and at rest. We use industry-standard security measures to protect your information.
                </p>
              </div>

              <h3 class="text-xl font-semibold text-gray-900 mb-3 mt-6">2.3 Automatically Collected Information</h3>
              <p class="text-gray-700 leading-relaxed mb-4">
                We automatically collect certain information when you use our Service, including:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li><strong>Device Information:</strong> IP address, browser type, operating system, device identifiers</li>
                <li><strong>Usage Data:</strong> Pages visited, features used, time spent, click patterns</li>
                <li><strong>Location Data:</strong> General geographic location (country/region level)</li>
                <li><strong>Cookies and Tracking:</strong> Cookies, web beacons, and similar tracking technologies</li>
              </ul>
            </section>

            {/* How We Use Information */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">3. How We Use Your Information</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                We use the information we collect for the following purposes:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li><strong>Service Provision:</strong> To provide, maintain, and improve our Service, including processing transactions and delivering alerts</li>
                <li><strong>Account Management:</strong> To create and manage your account, authenticate users, and process payments</li>
                <li><strong>Personalization:</strong> To customize your experience, provide relevant content, and tailor alerts to your preferences</li>
                <li><strong>Communication:</strong> To send you service updates, notifications, marketing communications (with your consent), and respond to your inquiries</li>
                <li><strong>Analytics:</strong> To analyze usage patterns, improve our Service, and develop new features</li>
                <li><strong>Security:</strong> To detect, prevent, and address fraud, security breaches, and other illegal activities</li>
                <li><strong>Legal Compliance:</strong> To comply with legal obligations, enforce our Terms of Service, and protect our rights</li>
                <li><strong>Research:</strong> To conduct research and analysis (using anonymized data) to improve trading algorithms and market insights</li>
              </ul>
            </section>

            {/* Information Sharing */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">4. Information Sharing and Disclosure</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                We do not sell your personal information. We may share your information in the following circumstances:
              </p>

              <h3 class="text-xl font-semibold text-gray-900 mb-3 mt-6">4.1 Service Providers</h3>
              <p class="text-gray-700 leading-relaxed mb-4">
                We may share information with third-party service providers who perform services on our behalf, including:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Payment processors</li>
                <li>Cloud hosting and infrastructure providers</li>
                <li>Analytics and data processing services</li>
                <li>Customer support platforms</li>
                <li>Email and communication services</li>
              </ul>
              <p class="text-gray-700 leading-relaxed mb-4">
                These service providers are contractually obligated to protect your information and use it only for the purposes we specify.
              </p>

              <h3 class="text-xl font-semibold text-gray-900 mb-3 mt-6">4.2 Trading Platform Integrations</h3>
              <p class="text-gray-700 leading-relaxed mb-4">
                When you connect your trading accounts, we may share necessary information with your broker or trading platform to facilitate:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Account authentication and authorization</li>
                <li>Trade execution (if you choose to use automated execution features)</li>
                <li>Market data retrieval</li>
              </ul>
              <p class="text-gray-700 leading-relaxed">
                Your use of third-party trading platforms is also subject to their respective privacy policies.
              </p>

              <h3 class="text-xl font-semibold text-gray-900 mb-3 mt-6">4.3 Legal Requirements</h3>
              <p class="text-gray-700 leading-relaxed mb-4">
                We may disclose your information if required by law or in response to:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Court orders, subpoenas, or legal processes</li>
                <li>Government or regulatory requests</li>
                <li>Enforcement of our Terms of Service</li>
                <li>Protection of our rights, property, or safety, or that of our users</li>
              </ul>

              <h3 class="text-xl font-semibold text-gray-900 mb-3 mt-6">4.4 Business Transfers</h3>
              <p class="text-gray-700 leading-relaxed">
                In the event of a merger, acquisition, or sale of assets, your information may be transferred to the acquiring entity, subject to the same privacy protections.
              </p>
            </section>

            {/* Data Security */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">5. Data Security</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                We implement appropriate technical and organizational measures to protect your personal information, including:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>256-bit SSL encryption for data in transit</li>
                <li>Encryption at rest for sensitive data</li>
                <li>Secure authentication and access controls</li>
                <li>Regular security audits and vulnerability assessments</li>
                <li>Employee training on data protection</li>
                <li>Incident response procedures</li>
              </ul>
              <p class="text-gray-700 leading-relaxed mb-4">
                However, no method of transmission over the Internet or electronic storage is 100% secure. While we strive to protect your information, we cannot guarantee absolute security.
              </p>
              <p class="text-gray-700 leading-relaxed">
                You are responsible for maintaining the confidentiality of your account credentials and for all activities that occur under your account.
              </p>
            </section>

            {/* Data Retention */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">6. Data Retention</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                We retain your personal information for as long as necessary to:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Provide our Service to you</li>
                <li>Comply with legal obligations</li>
                <li>Resolve disputes and enforce our agreements</li>
                <li>Maintain security and prevent fraud</li>
              </ul>
              <p class="text-gray-700 leading-relaxed mb-4">
                When you delete your account, we will:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Delete or anonymize your personal information within 30 days</li>
                <li>Retain certain information as required by law (e.g., financial records for tax purposes)</li>
                <li>Retain anonymized, aggregated data for analytics and research purposes</li>
              </ul>
            </section>

            {/* Your Rights (GDPR) */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">7. Your Rights (GDPR and Data Protection)</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                If you are located in the European Economic Area (EEA) or United Kingdom, you have certain rights under the General Data Protection Regulation (GDPR) and UK GDPR:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li><strong>Right of Access:</strong> Request a copy of your personal information</li>
                <li><strong>Right to Rectification:</strong> Correct inaccurate or incomplete information</li>
                <li><strong>Right to Erasure:</strong> Request deletion of your personal information ("right to be forgotten")</li>
                <li><strong>Right to Restrict Processing:</strong> Request limitation of how we process your information</li>
                <li><strong>Right to Data Portability:</strong> Receive your data in a structured, machine-readable format</li>
                <li><strong>Right to Object:</strong> Object to processing of your information for certain purposes</li>
                <li><strong>Right to Withdraw Consent:</strong> Withdraw consent where processing is based on consent</li>
              </ul>
              <p class="text-gray-700 leading-relaxed mb-4">
                To exercise these rights, please contact us at <strong>privacy@pricewhisperer.ai</strong>. We will respond to your request within 30 days.
              </p>
              <p class="text-gray-700 leading-relaxed">
                You also have the right to lodge a complaint with your local data protection authority if you believe we have violated your data protection rights.
              </p>
            </section>

            {/* Cookies */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">8. Cookies and Tracking Technologies</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                We use cookies and similar tracking technologies to:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Remember your preferences and settings</li>
                <li>Authenticate your account</li>
                <li>Analyze usage patterns and improve our Service</li>
                <li>Provide personalized content and advertising</li>
              </ul>
              <p class="text-gray-700 leading-relaxed mb-4">
                Types of cookies we use:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li><strong>Essential Cookies:</strong> Required for the Service to function</li>
                <li><strong>Analytics Cookies:</strong> Help us understand how users interact with our Service</li>
                <li><strong>Functional Cookies:</strong> Remember your preferences and settings</li>
                <li><strong>Advertising Cookies:</strong> Used to deliver relevant advertisements</li>
              </ul>
              <p class="text-gray-700 leading-relaxed">
                You can control cookies through your browser settings. However, disabling certain cookies may affect the functionality of our Service.
              </p>
            </section>

            {/* Children's Privacy */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">9. Children's Privacy</h2>
              <p class="text-gray-700 leading-relaxed">
                Our Service is not intended for individuals under the age of 18. We do not knowingly collect personal information from children. If we become aware that we have collected information from a child under 18, we will take steps to delete such information promptly.
              </p>
            </section>

            {/* International Transfers */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">10. International Data Transfers</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                Your information may be transferred to and processed in countries other than your country of residence. These countries may have different data protection laws than your country.
              </p>
              <p class="text-gray-700 leading-relaxed mb-4">
                When we transfer data outside the EEA or UK, we ensure appropriate safeguards are in place, including:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Standard Contractual Clauses approved by the European Commission</li>
                <li>Adequacy decisions by the European Commission</li>
                <li>Other legally recognized transfer mechanisms</li>
              </ul>
            </section>

            {/* Marketing Communications */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">11. Marketing Communications</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                With your consent, we may send you marketing communications about our Service, new features, and promotional offers. You can opt out of marketing communications at any time by:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Clicking the "unsubscribe" link in any marketing email</li>
                <li>Updating your account preferences</li>
                <li>Contacting us at <strong>privacy@pricewhisperer.ai</strong></li>
              </ul>
              <p class="text-gray-700 leading-relaxed">
                Please note that even if you opt out of marketing communications, we may still send you important service-related messages (e.g., account updates, security notifications).
              </p>
            </section>

            {/* Third-Party Links */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">12. Third-Party Links</h2>
              <p class="text-gray-700 leading-relaxed">
                Our Service may contain links to third-party websites or services. We are not responsible for the privacy practices of these third parties. We encourage you to read their privacy policies before providing any information.
              </p>
            </section>

            {/* Changes to Privacy Policy */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">13. Changes to This Privacy Policy</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                We may update this Privacy Policy from time to time. We will notify you of material changes by:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Posting the updated Privacy Policy on our website</li>
                <li>Sending an email to the address associated with your account</li>
                <li>Displaying a notice within the Service</li>
              </ul>
              <p class="text-gray-700 leading-relaxed">
                Your continued use of the Service after such modifications constitutes your acceptance of the updated Privacy Policy. The "Last updated" date at the top of this page indicates when the Privacy Policy was last revised.
              </p>
            </section>

            {/* Contact Information */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">14. Contact Us</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                If you have any questions, concerns, or requests regarding this Privacy Policy or our data practices, please contact us at:
              </p>
              <div class="bg-gray-50 p-4 rounded-lg">
                <p class="text-gray-700 mb-2"><strong>Email:</strong> privacy@pricewhisperer.ai</p>
                <p class="text-gray-700 mb-2"><strong>Data Protection Officer:</strong> dpo@pricewhisperer.ai</p>
                <p class="text-gray-700 mb-2"><strong>Address:</strong> [Your Company Address]</p>
                <p class="text-gray-700"><strong>Phone:</strong> [Your Phone Number]</p>
              </div>
            </section>
            </div>
          </LegalPageLayout>
        </div>
        
        {/* Sidebar - Legal Pages */}
        <div class="lg:col-span-4 lg:sticky lg:top-8 lg:self-start">
          <LegalPagesSidebar currentPage="privacy-policy" />
        </div>
      </div>
    </div>
  );
};

export default PrivacyPolicy;

