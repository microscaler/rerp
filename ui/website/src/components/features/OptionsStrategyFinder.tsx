import { Component } from 'solid-js';
import FeatureLayout from './FeatureLayout';
import {
  KeyTakeawayBox,
  FeatureCardGrid,
  SectionCardStack,
  InfoBox,
  FeatureList,
  KeyTakeawaysBox
} from '../blog/components';

const OptionsStrategyFinder: Component = () => {
  return (
    <FeatureLayout
      header={{
        title: 'Options Strategy Finder',
        subtitle: 'Discover profitable options strategies automatically',
        icon: 'fa-solid fa-chart-bar',
        iconBgColor: 'bg-secondary'
      }}
      showBackLink={true}
    >
          <KeyTakeawayBox color="secondary" title="What You Get" icon="fa-solid fa-lightbulb">
            Our system analyzes market conditions, volatility, and Greeks to automatically suggest 
            the best options strategies for your risk profile and market outlook.
          </KeyTakeawayBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">How It Works</h2>
          <p class="text-gray-700 mb-4">
            Options trading doesn't have to be complicated. Our Strategy Finder evaluates current market 
            conditions and suggests proven strategies that match your risk tolerance:
          </p>

          <FeatureCardGrid
            columns={2}
            items={[
              {
                icon: 'fa-solid fa-chart-line',
                iconColor: 'text-secondary',
                title: 'Market Analysis',
                description: 'Evaluates trend direction, volatility regime, and support/resistance levels to determine the best strategy type for current conditions.'
              },
              {
                icon: 'fa-solid fa-calculator',
                iconColor: 'text-primary',
                title: 'Greeks Analysis',
                description: 'Calculates Delta, Gamma, Theta, and Vega to ensure strategies are properly balanced for your risk profile and time horizon.'
              },
              {
                icon: 'fa-solid fa-wave-square',
                iconColor: 'text-accent',
                title: 'Volatility Assessment',
                description: 'Compares implied volatility to historical volatility to identify whether options are expensive or cheap, guiding strategy selection.'
              },
              {
                icon: 'fa-solid fa-shield-alt',
                iconColor: 'text-indigo-600',
                title: 'Risk Profiling',
                description: 'Matches strategies to your risk tolerance - from conservative income plays to aggressive directional bets.'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Available Strategies</h2>
          <SectionCardStack
            items={[
              {
                title: 'Iron Condors',
                description: 'Perfect for range-bound markets. Collect premium while defining your max risk. Our system finds the optimal strike selection for maximum profit probability.',
                metadata: 'Best for: Low volatility, sideways markets',
                metadataIcon: 'fa-solid fa-check-circle text-secondary'
              },
              {
                title: 'Strangles & Straddles',
                description: 'Profit from big moves in either direction. We identify when volatility is low enough to make these plays profitable.',
                metadata: 'Best for: High volatility expectations, earnings plays',
                metadataIcon: 'fa-solid fa-check-circle text-secondary'
              },
              {
                title: '0-DTE Strategies',
                description: 'Same-day expiration plays for quick profits. Our system identifies the best 0-DTE setups with proper risk management.',
                metadata: 'Best for: Active traders, quick profit opportunities',
                metadataIcon: 'fa-solid fa-check-circle text-secondary'
              },
              {
                title: 'Covered Calls & Cash-Secured Puts',
                description: 'Generate income from stocks you own or want to own. We find the optimal strike prices for maximum premium collection.',
                metadata: 'Best for: Income generation, stock ownership enhancement',
                metadataIcon: 'fa-solid fa-check-circle text-secondary'
              },
              {
                title: 'Vertical Spreads',
                description: 'Defined-risk directional plays. Bull calls, bear puts, and debit/credit spreads optimized for your market outlook.',
                metadata: 'Best for: Directional trades with limited risk',
                metadataIcon: 'fa-solid fa-check-circle text-secondary'
              },
              {
                title: 'Gamma Scalping',
                description: 'Advanced strategy for managing delta-neutral positions. Our system identifies when gamma scalping opportunities are most profitable.',
                metadata: 'Best for: Advanced traders, volatility trading',
                metadataIcon: 'fa-solid fa-check-circle text-secondary'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Strategy Details</h2>
          <p class="text-gray-700 mb-4">
            Each suggested strategy includes:
          </p>
          <InfoBox>
            <FeatureList
              items={[
                { title: 'Entry Instructions', description: 'Exact strikes, expiration dates, and entry prices' },
                { title: 'Risk/Reward Analysis', description: 'Maximum profit, maximum loss, and break-even points' },
                { title: 'Greeks Breakdown', description: 'How Delta, Gamma, Theta, and Vega affect your position' },
                { title: 'Exit Strategy', description: 'When to take profits, adjust, or close the position' },
                { title: 'Probability of Profit', description: 'Historical win rate for similar setups' }
              ]}
            />
          </InfoBox>

          <KeyTakeawaysBox
            items={[
              <>No more guessing which strategy to use - get data-driven recommendations</>,
              <>Understand the risk before you enter - every strategy shows max loss upfront</>,
              <>Learn as you trade - each suggestion includes educational context</>,
              <>Match strategies to market conditions - our system adapts to volatility regimes</>,
              <>Optimize for your account size - strategies are sized appropriately for your capital</>
            ]}
            title="Key Benefits"
          />

    </FeatureLayout>
  );
};

export default OptionsStrategyFinder;
