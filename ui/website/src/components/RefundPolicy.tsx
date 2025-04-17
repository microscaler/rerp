import { Component, onMount } from 'solid-js';
import { updateSEO } from '../utils/seo';
import { BASE_URL } from '../config/constants';
import LegalPagesSidebar from './legal/LegalPagesSidebar';
import LegalPageLayout from './legal/LegalPageLayout';

const RefundPolicy: Component = () => {
  const lastUpdated = new Date().toLocaleDateString('en-GB', { year: 'numeric', month: 'long', day: 'numeric' });

  onMount(() => {
    updateSEO({
      title: 'Refund Policy - PriceWhisperer',
      description: 'Refund policy for PriceWhisperer subscriptions. Learn about our refund terms for monthly and annual subscriptions.',
      keywords: 'refund policy, subscription refund, cancellation policy, PriceWhisperer refund',
      canonical: `${BASE_URL}/#refund-policy`,
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
              title: 'Refund Policy',
              lastUpdated: lastUpdated,
              icon: 'fa-solid fa-money-bill-wave',
              iconBgColor: 'bg-primary'
            }}
            showBackLink={true}
          >
            <div class="space-y-8">
            
            {/* Introduction */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">1. Introduction</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                This Refund Policy ("Policy") outlines the terms and conditions under which PriceWhisperer ("we," "our," or "us") provides refunds for subscription fees. By subscribing to our Service, you agree to this Refund Policy.
              </p>
              <p class="text-gray-700 leading-relaxed">
                Please read this Policy carefully before making a purchase. If you have any questions, please contact us at <strong>support@pricewhisperer.ai</strong>.
              </p>
            </section>

            {/* Monthly Subscriptions */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">2. Monthly Subscriptions</h2>
              <div class="bg-red-50 border-l-4 border-red-500 p-4 mb-4">
                <p class="text-red-800 font-semibold mb-2">No Refunds for Monthly Subscriptions</p>
                <p class="text-red-700 leading-relaxed">
                  We do not provide refunds for monthly subscription fees. Monthly subscriptions are non-refundable once payment has been processed.
                </p>
              </div>
              <p class="text-gray-700 leading-relaxed mb-4">
                Monthly subscriptions are billed on a recurring basis. You may cancel your monthly subscription at any time, and your access will continue until the end of your current billing period. No refunds will be issued for the current or any previous billing periods.
              </p>
            </section>

            {/* Billing Schedule */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">3. Billing Schedule</h2>
              <div class="bg-blue-50 border-l-4 border-blue-400 p-4 mb-4">
                <p class="text-blue-800 font-semibold mb-2">Monthly Billing Date</p>
                <p class="text-blue-700 leading-relaxed">
                  Monthly subscriptions are billed on the <strong>28th of each calendar month</strong>, regardless of whether this date falls on a weekday or weekend.
                </p>
              </div>
              <p class="text-gray-700 leading-relaxed mb-4">
                Important billing information:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Billing occurs automatically on the 28th of each month</li>
                <li>The billing date is fixed and does not change based on weekends or holidays</li>
                <li>You will be charged for the next calendar month on the 28th</li>
                <li>If payment fails, your subscription may be suspended until payment is successfully processed</li>
                <li>You will receive email notifications before each billing cycle</li>
              </ul>
              <p class="text-gray-700 leading-relaxed">
                To avoid being charged for the next month, you must cancel your subscription before the 28th of the current month.
              </p>
            </section>

            {/* Annual Subscriptions */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">4. Annual Subscriptions</h2>
              <div class="bg-yellow-50 border-l-4 border-yellow-400 p-4 mb-4">
                <p class="text-yellow-800 font-semibold mb-2">Quarterly-Based Refunds</p>
                <p class="text-yellow-700 leading-relaxed">
                  Annual subscriptions may be eligible for partial refunds based on unused quarters, subject to the terms outlined below.
                </p>
              </div>
              <p class="text-gray-700 leading-relaxed mb-4">
                Refund terms for annual subscriptions:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Refunds are calculated based on <strong>unused quarters</strong> (3-month periods)</li>
                <li>We commit to infrastructure providers on a quarterly basis, which affects our refund policy</li>
                <li>You will receive a refund for complete unused quarters only</li>
                <li>Partial quarters (less than 3 months) are not eligible for refund</li>
                <li>Refund requests must be submitted in writing to <strong>support@pricewhisperer.ai</strong></li>
              </ul>
              <p class="text-gray-700 leading-relaxed mb-4">
                <strong>Example:</strong> If you cancel an annual subscription after 5 months:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>You have used 1 complete quarter (3 months) and part of a second quarter (2 months)</li>
                <li>You will be refunded for 2 complete unused quarters (6 months)</li>
                <li>The partial quarter (2 months) is not refundable</li>
              </ul>
              <p class="text-gray-700 leading-relaxed">
                Refunds for annual subscriptions will be processed within 14 business days of approval and will be issued to the original payment method used for the subscription.
              </p>
            </section>

            {/* Trading Portal Fees */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">5. Trading Portal Fees and Costs</h2>
              <div class="bg-orange-50 border-l-4 border-orange-400 p-4 mb-4">
                <p class="text-orange-800 font-semibold mb-2">Third-Party Trading Portal Fees</p>
                <p class="text-orange-700 leading-relaxed">
                  PriceWhisperer is not responsible for any fees, costs, or charges associated with trading portals or brokers that you connect to your account.
                </p>
              </div>
              <p class="text-gray-700 leading-relaxed mb-4">
                Important information regarding trading portal fees:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>All fees and costs to trading portals must be settled directly on the respective partner account</li>
                <li>PriceWhisperer does not process, collect, or manage fees for third-party trading platforms</li>
                <li>We hold no responsibility for trading portal fees, commissions, or any other charges</li>
                <li>Any disputes regarding trading portal fees must be resolved directly with the trading portal provider</li>
                <li>Refunds from PriceWhisperer do not include or affect any fees charged by trading portals</li>
              </ul>
              <p class="text-gray-700 leading-relaxed">
                Before connecting a trading portal to your PriceWhisperer account, please review the fee structure and terms of service of that trading portal. PriceWhisperer's refund policy applies only to PriceWhisperer subscription fees and does not extend to any third-party services.
              </p>
            </section>

            {/* Cancellation */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">6. Cancellation</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                You may cancel your subscription at any time through your account settings or by contacting our support team. Cancellation terms:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li><strong>Monthly Subscriptions:</strong> Cancellation takes effect at the end of the current billing period. No refunds are provided for the current period.</li>
                <li><strong>Annual Subscriptions:</strong> Cancellation may be eligible for a partial refund based on unused quarters, as described in Section 4.</li>
                <li>You will retain access to the Service until the end of your current billing period</li>
                <li>To avoid being charged for the next billing cycle, cancel before the billing date (28th of the month for monthly subscriptions)</li>
              </ul>
            </section>

            {/* Refund Processing */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">7. Refund Processing</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                If you are eligible for a refund:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Refunds will be processed within 14 business days of approval</li>
                <li>Refunds will be issued to the original payment method used for the subscription</li>
                <li>Processing times may vary depending on your payment provider</li>
                <li>You will receive email confirmation once the refund has been processed</li>
              </ul>
              <p class="text-gray-700 leading-relaxed">
                If you do not receive your refund within the specified timeframe, please contact us at <strong>support@pricewhisperer.ai</strong>.
              </p>
            </section>

            {/* Non-Refundable Items */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">8. Non-Refundable Items</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                The following are not eligible for refunds:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Monthly subscription fees (as stated in Section 2)</li>
                <li>Partial quarters of annual subscriptions (less than 3 months)</li>
                <li>Any fees or costs associated with third-party trading portals or brokers</li>
                <li>Any add-on services or features purchased separately</li>
                <li>Subscriptions that have been suspended or terminated due to violation of our Terms of Service</li>
              </ul>
            </section>

            {/* Chargebacks */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">9. Chargebacks and Disputes</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                If you initiate a chargeback or dispute a charge with your payment provider:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Your account will be immediately suspended pending resolution</li>
                <li>We reserve the right to terminate your account if a chargeback is found to be fraudulent</li>
                <li>We encourage you to contact us directly at <strong>support@pricewhisperer.ai</strong> to resolve any billing issues before initiating a chargeback</li>
                <li>We will provide all necessary documentation to your payment provider to resolve the dispute</li>
              </ul>
            </section>

            {/* Changes to Policy */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">10. Changes to This Refund Policy</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                We reserve the right to modify this Refund Policy at any time. We will notify you of material changes by:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mb-4 ml-4">
                <li>Posting the updated Policy on our website</li>
                <li>Sending an email to the address associated with your account</li>
                <li>Displaying a notice within the Service</li>
              </ul>
              <p class="text-gray-700 leading-relaxed">
                Changes to this Policy will apply to all subscriptions purchased after the effective date of the change. Your continued use of the Service after such modifications constitutes your acceptance of the updated Policy.
              </p>
            </section>

            {/* Contact Information */}
            <section>
              <h2 class="text-2xl font-bold text-gray-900 mb-4">11. Contact Us</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                If you have any questions about this Refund Policy or wish to request a refund, please contact us at:
              </p>
              <div class="bg-gray-50 p-4 rounded-lg">
                <p class="text-gray-700 mb-2"><strong>Email:</strong> support@pricewhisperer.ai</p>
                <p class="text-gray-700 mb-2"><strong>Address:</strong> [Your Company Address]</p>
                <p class="text-gray-700"><strong>Phone:</strong> [Your Phone Number]</p>
              </div>
              <p class="text-gray-700 leading-relaxed mt-4">
                For refund requests, please include your account email address and subscription details in your inquiry.
              </p>
            </section>

            {/* Acknowledgment */}
            <section class="bg-blue-50 border-l-4 border-blue-500 p-6 rounded-lg">
              <h2 class="text-2xl font-bold text-gray-900 mb-4">12. Acknowledgment</h2>
              <p class="text-gray-700 leading-relaxed mb-4">
                BY SUBSCRIBING TO PRICEWHISPERER, YOU ACKNOWLEDGE THAT YOU HAVE READ, UNDERSTOOD, AND AGREE TO BE BOUND BY THIS REFUND POLICY.
              </p>
              <p class="text-gray-700 leading-relaxed">
                You understand that:
              </p>
              <ul class="list-disc list-inside space-y-2 text-gray-700 mt-4 ml-4">
                <li>Monthly subscriptions are non-refundable</li>
                <li>Annual subscriptions may be eligible for partial refunds based on unused quarters only</li>
                <li>Billing occurs on the 28th of each month regardless of weekday or weekend</li>
                <li>PriceWhisperer is not responsible for trading portal fees or costs</li>
                <li>All trading portal fees must be settled directly with the respective trading portal</li>
              </ul>
            </section>
            </div>
          </LegalPageLayout>
        </div>
        
        {/* Sidebar - Legal Pages */}
        <div class="lg:col-span-4 lg:sticky lg:top-8 lg:self-start">
          <LegalPagesSidebar currentPage="refund-policy" />
        </div>
      </div>
    </div>
  );
};

export default RefundPolicy;

