import { Component } from 'solid-js';
import BlogPostLayout from './BlogPostLayout';
import {
  KeyTakeawayBox,
  InfoBox,
  FeatureList,
  SectionCardStack,
  FeatureCardGrid,
  MistakeList,
  Checklist,
  KeyTakeawaysBox
} from './components';

const RiskManagementBasics: Component = () => {
  return (
    <BlogPostLayout
      header={{
        category: 'Risk Management',
        title: 'Risk Management Fundamentals for Traders',
        date: 'November 10, 2024',
        readTime: '12 min read',
        icon: 'fa-solid fa-shield-alt',
        iconBgColor: 'bg-indigo-600'
      }}
      showBackLink={true}
    >
          <KeyTakeawayBox color="indigo">
            Risk management is the foundation of successful trading. Without it, even the best 
            trading strategies will fail. PriceWhisperer's built-in risk management helps protect 
            your capital automatically.
          </KeyTakeawayBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Why Risk Management Matters</h2>
          <p class="text-gray-700 mb-4">
            The most successful traders aren't necessarily the ones who make the most money - they're 
            the ones who survive the longest. Risk management ensures you stay in the game long enough 
            to let your winning trades compound. Without proper risk management, a few bad trades can 
            wipe out months of gains.
          </p>

          <InfoBox title="The Math of Losses" variant="red">
            <p class="text-gray-700 text-sm mb-3">
              If you lose 50% of your account, you need a 100% gain just to break even. This is why 
              protecting your capital is more important than maximizing profits.
            </p>
            <ul class="space-y-2 text-gray-700 text-sm">
              <li>• 10% loss requires 11% gain to recover</li>
              <li>• 25% loss requires 33% gain to recover</li>
              <li>• 50% loss requires 100% gain to recover</li>
              <li>• 75% loss requires 300% gain to recover</li>
            </ul>
          </InfoBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">The 2% Rule</h2>
          <p class="text-gray-700 mb-4">
            One of the most fundamental risk management rules is to never risk more than 2% of your 
            account on a single trade. This means:
          </p>
          <InfoBox>
            <FeatureList
              items={[
                { title: 'Account Size', description: 'If you have $10,000, never risk more than $200 per trade' },
                { title: 'Position Sizing', description: 'If your stop-loss is $2 away, you can buy 100 shares ($200 risk)' },
                { title: 'Wider Stops', description: 'If your stop-loss is $4 away, you can only buy 50 shares to maintain 2% risk' }
              ]}
              iconColor="text-primary"
              variant="info"
            />
          </InfoBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Position Sizing</h2>
          <p class="text-gray-700 mb-4">
            Position sizing is the process of determining how many shares or contracts to trade based 
            on your risk tolerance. PriceWhisperer calculates this automatically:
          </p>
          <InfoBox title="Position Size Formula">
            <div class="bg-white rounded p-4 mb-3 font-mono text-sm">
              Position Size = (Account Balance × Risk %) / (Entry Price - Stop Loss)
            </div>
            <p class="text-gray-600 text-sm">
              Example: $10,000 account, 2% risk, $100 entry, $95 stop = ($10,000 × 0.02) / ($100 - $95) = 40 shares
            </p>
          </InfoBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Stop-Loss Placement</h2>
          <p class="text-gray-700 mb-4">
            Stop-losses are your safety net. They limit losses when trades go against you. PriceWhisperer 
            suggests stop-losses based on:
          </p>
          <SectionCardStack
            items={[
              {
                title: 'Support/Resistance Levels',
                description: 'Stops placed just beyond key technical levels where price is likely to reverse. If price breaks through support, the trade is likely invalid.'
              },
              {
                title: 'ATR-Based Stops',
                description: 'Average True Range (ATR) measures volatility. Stops sized based on ATR account for normal price fluctuations while protecting against significant moves.'
              },
              {
                title: 'Percentage-Based Stops',
                description: 'For stocks, a common approach is 5-10% stops. For options, stops might be 20-30% due to higher volatility.'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Portfolio Risk Management</h2>
          <p class="text-gray-700 mb-4">
            Beyond individual trades, you need to manage risk across your entire portfolio:
          </p>
          <FeatureCardGrid
            columns={2}
            items={[
              {
                icon: 'fa-solid fa-chart-line-down',
                iconColor: 'text-red-600',
                title: 'Maximum Drawdown',
                description: 'Set a maximum portfolio loss (e.g., 20%). If your account drops by this amount, reduce position sizes or pause trading to reassess.'
              },
              {
                icon: 'fa-solid fa-layer-group',
                iconColor: 'text-primary',
                title: 'Sector Concentration',
                description: 'Don\'t put all your capital in one sector. If tech stocks make up 50% of your portfolio, you\'re over-exposed to tech-specific risks.'
              },
              {
                icon: 'fa-solid fa-link',
                iconColor: 'text-secondary',
                title: 'Correlation Risk',
                description: 'Multiple positions in highly correlated stocks act like one large position. PriceWhisperer tracks correlation to prevent hidden concentration.'
              },
              {
                icon: 'fa-solid fa-shield-alt',
                iconColor: 'text-indigo-600',
                title: 'Total Exposure',
                description: 'Limit total portfolio risk. Even with 2% per trade, 10 positions could mean 20% total risk if they all move against you.'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Using PriceWhisperer's Risk Management</h2>
          <p class="text-gray-700 mb-4">
            PriceWhisperer's built-in risk management features protect your capital automatically:
          </p>
          <InfoBox>
            <FeatureList
              items={[
                { title: 'Automatic Position Sizing', description: 'Every trade recommendation includes the correct position size based on your risk parameters' },
                { title: 'Stop-Loss Suggestions', description: 'Every alert includes a suggested stop-loss based on technical levels and volatility' },
                { title: 'Portfolio Limits', description: 'Set maximum exposure per sector, total portfolio risk, and correlation limits' },
                { title: 'Risk Alerts', description: 'Get notified when your portfolio risk exceeds thresholds or positions need adjustment' },
                { title: 'Real-Time Monitoring', description: 'Track current risk, available risk capacity, and risk-adjusted returns' }
              ]}
            />
          </InfoBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Common Risk Management Mistakes</h2>
          <MistakeList
            items={[
              { title: 'No stop-losses', description: 'Hoping a losing trade will come back is a recipe for disaster' },
              { title: 'Moving stops', description: 'Moving stops further away to avoid losses only increases risk' },
              { title: 'Overtrading', description: 'Too many positions increase total portfolio risk' },
              { title: 'Revenge trading', description: 'Trying to make back losses by increasing position sizes' },
              { title: 'Ignoring correlation', description: 'Multiple positions in similar stocks act like one large position' }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Risk Management Checklist</h2>
          <Checklist
            items={[
              'Never risk more than 2% of account per trade',
              'Always use stop-losses on every trade',
              'Set maximum portfolio drawdown limit (e.g., 20%)',
              'Limit sector concentration (max 25-30% per sector)',
              'Monitor correlation between positions',
              'Review and adjust risk parameters regularly'
            ]}
          />

          <KeyTakeawaysBox
            items={[
              'Risk management is more important than finding winning trades',
              'Never risk more than 2% of your account on a single trade',
              'Always use stop-losses - they\'re your safety net',
              'Manage portfolio-level risk, not just individual trades',
              'PriceWhisperer\'s built-in risk management protects your capital automatically'
            ]}
          />

    </BlogPostLayout>
  );
};

export default RiskManagementBasics;
