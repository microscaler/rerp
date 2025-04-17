import { Component } from 'solid-js';
import { EXTERNAL_URLS } from '@shared/config/constants';
import FeatureLayout from './FeatureLayout';
import {
  KeyTakeawayBox,
  FeatureCardGrid,
  SectionCardStack,
  InfoBox,
  FeatureList,
  KeyTakeawaysBox
} from '../blog/components';

const FinancialTradingEducation: Component = () => {
  return (
    <FeatureLayout
      header={{
        title: 'Financial Trading Education',
        subtitle: 'The most comprehensive training in finance available',
        icon: 'fa-solid fa-graduation-cap',
        iconBgColor: 'bg-blue-600'
      }}
      showBackLink={true}
    >
          <KeyTakeawayBox color="blue" title="What You Get" icon="fa-solid fa-book">
            Access to our comprehensive Financial Trading Education program - a complete curriculum 
            covering everything from fundamentals to expert-level trading strategies.
          </KeyTakeawayBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Why Our Education Program is Second to None</h2>
          <p class="text-gray-700 mb-4">
            Our Financial Trading Education program is the most comprehensive training in finance available. 
            Unlike generic courses or fragmented tutorials, we provide a structured, progressive curriculum 
            that takes you from complete beginner to expert-level trader.
          </p>

          <FeatureCardGrid
            columns={2}
            items={[
              {
                icon: 'fa-solid fa-list-ol',
                iconColor: 'text-blue-600',
                title: '50 Comprehensive Modules',
                description: 'Structured across 11 progressive parts, covering every aspect of financial markets from banking fundamentals to advanced quantitative finance.'
              },
              {
                icon: 'fa-solid fa-chart-line',
                iconColor: 'text-primary',
                title: 'Progressive Learning Path',
                description: 'Five distinct levels (101-501) that build systematically from beginner to expert, ensuring you master each concept before moving forward.'
              },
              {
                icon: 'fa-solid fa-globe',
                iconColor: 'text-secondary',
                title: 'Complete Market Coverage',
                description: 'Equity markets, fixed income, forex, commodities, options, derivatives, risk management, and advanced quantitative finance - all in one place.'
              },
              {
                icon: 'fa-solid fa-certificate',
                iconColor: 'text-accent',
                title: 'Industry-Standard Content',
                description: 'Content designed for professionals working in financial institutions, ensuring you learn the same concepts used by traders at major banks and hedge funds.'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">What's Included</h2>
          <SectionCardStack
            items={[
              {
                title: 'Part I: Foundations (101-201 Level)',
                description: (
                  <ul class="list-disc pl-6 text-gray-700 text-sm space-y-1">
                    <li>Introduction to Financial Markets</li>
                    <li>Banking Fundamentals</li>
                    <li>Economics Foundations</li>
                    <li>Mathematics for Finance</li>
                  </ul>
                )
              },
              {
                title: 'Part II: Equity Markets (201-301 Level)',
                description: (
                  <ul class="list-disc pl-6 text-gray-700 text-sm space-y-1">
                    <li>Stocks and Shares Fundamentals</li>
                    <li>Equity Valuation</li>
                    <li>Equity Trading</li>
                    <li>Technical Analysis for Equities</li>
                  </ul>
                )
              },
              {
                title: 'Options Trading (301-401 Level)',
                description: (
                  <ul class="list-disc pl-6 text-gray-700 text-sm space-y-1">
                    <li>Options Fundamentals</li>
                    <li>Options Mathematics</li>
                    <li>The Greeks (Delta, Gamma, Theta, Vega)</li>
                    <li>Options Strategies</li>
                    <li>Volatility Trading</li>
                    <li>Advanced Options Concepts</li>
                  </ul>
                )
              },
              {
                title: 'Risk Management (401 Level)',
                description: (
                  <ul class="list-disc pl-6 text-gray-700 text-sm space-y-1">
                    <li>Risk Management Fundamentals</li>
                    <li>Portfolio Risk</li>
                    <li>Credit Risk</li>
                    <li>Market Risk</li>
                    <li>Operational Risk</li>
                  </ul>
                )
              },
              {
                title: 'Advanced Topics (501 Level)',
                description: (
                  <ul class="list-disc pl-6 text-gray-700 text-sm space-y-1">
                    <li>Quantitative Finance</li>
                    <li>Market Making</li>
                    <li>High-Frequency Trading</li>
                    <li>Alternative Investments</li>
                    <li>Behavioral Finance</li>
                    <li>Cross-Asset Trading</li>
                  </ul>
                )
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">What Makes It Different</h2>
          <InfoBox>
            <FeatureList
              items={[
                { title: 'Comprehensive Coverage', description: '50 modules covering every aspect of financial markets - no gaps, no shortcuts' },
                { title: 'Progressive Structure', description: 'Each level builds on the previous, ensuring you understand fundamentals before advanced concepts' },
                { title: 'Practical Focus', description: 'Real-world examples, case studies, and practical applications throughout' },
                { title: 'Industry Standard', description: 'Content aligned with what professionals use at major financial institutions' },
                { title: 'Self-Paced Learning', description: 'Study at your own speed, revisit concepts as needed' },
                { title: 'Always Updated', description: 'Content reflects current market practices and regulations' }
              ]}
            />
          </InfoBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Who It's For</h2>
          <FeatureCardGrid
            columns={2}
            items={[
              {
                icon: 'fa-solid fa-user-graduate',
                iconColor: 'text-blue-600',
                title: 'Beginners',
                description: 'Start with Level 101 fundamentals. No prior knowledge required - we teach you everything from the ground up.'
              },
              {
                icon: 'fa-solid fa-chart-line',
                iconColor: 'text-green-600',
                title: 'Intermediate Traders',
                description: 'Jump in at Level 201-301 to deepen your understanding of specific markets and strategies.'
              },
              {
                icon: 'fa-solid fa-trophy',
                iconColor: 'text-purple-600',
                title: 'Advanced Practitioners',
                description: 'Level 401-501 covers advanced topics like quantitative finance, HFT, and cross-asset trading.'
              },
              {
                icon: 'fa-solid fa-briefcase',
                iconColor: 'text-orange-600',
                title: 'Professionals',
                description: 'Perfect for developers, analysts, and professionals working on trading systems who need comprehensive market knowledge.'
              }
            ]}
          />

          <div class="bg-blue-50 rounded-lg p-6 mt-8">
            <h3 class="text-xl font-semibold text-gray-900 mb-4">Get Started Today</h3>
            <p class="text-gray-700 mb-4">
              Access our complete Financial Trading Education program and start your journey from 
              fundamentals to expert-level trading.
            </p>
            <a
              href={EXTERNAL_URLS.tradingEducation}
              target="_blank"
              rel="noopener noreferrer"
              class="inline-flex items-center px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 font-semibold"
            >
              Explore Financial Trading Education
              <i class="fa-solid fa-external-link-alt ml-2"></i>
            </a>
          </div>

    </FeatureLayout>
  );
};

export default FinancialTradingEducation;
