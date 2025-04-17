import { Component } from 'solid-js';
import BlogPostLayout from './BlogPostLayout';
import {
  KeyTakeawayBox,
  InfoBox,
  FeatureList,
  KeyTakeawaysBox,
  SectionCardStack,
  MistakeList,
  DoDontGrid
} from './components';

const IronCondorStrategy: Component = () => {
  return (
    <BlogPostLayout
      header={{
        category: 'Options Trading',
        title: 'Mastering the Iron Condor Options Strategy',
        date: 'November 15, 2024',
        readTime: '8 min read',
        icon: 'fa-solid fa-chart-bar',
        iconBgColor: 'bg-primary'
      }}
      showBackLink={true}
    >
          <KeyTakeawayBox color="primary">
            The Iron Condor is a neutral options strategy that profits from low volatility and 
            range-bound markets. It's perfect for traders who want to collect premium while 
            defining their maximum risk.
          </KeyTakeawayBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">What is an Iron Condor?</h2>
          <p class="text-gray-700 mb-4">
            An Iron Condor is a four-leg options strategy that combines a bear call spread and a bull 
            put spread. It's designed to profit when the underlying stock stays within a specific price 
            range until expiration. The strategy gets its name from the profit/loss diagram, which 
            resembles a condor's wingspan.
          </p>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">How It Works</h2>
          <p class="text-gray-700 mb-4">
            An Iron Condor consists of four options positions:
          </p>
          <ol class="list-decimal pl-6 text-gray-700 space-y-3 mb-6">
            <li><strong>Sell a call option</strong> at a strike price above the current stock price (short call)</li>
            <li><strong>Buy a call option</strong> at a higher strike price (long call) - this caps your upside risk</li>
            <li><strong>Sell a put option</strong> at a strike price below the current stock price (short put)</li>
            <li><strong>Buy a put option</strong> at a lower strike price (long put) - this caps your downside risk</li>
          </ol>

          <InfoBox title="Example Setup">
            <p class="text-gray-700 mb-3">Stock trading at $100:</p>
            <ul class="space-y-2 text-gray-700 text-sm">
              <li>• Sell $105 Call, Buy $110 Call (bear call spread)</li>
              <li>• Sell $95 Put, Buy $90 Put (bull put spread)</li>
              <li>• Net credit received: $2.50 per share</li>
              <li>• Maximum profit: $2.50 (if stock stays between $95-$105 at expiration)</li>
              <li>• Maximum risk: $2.50 (difference between strikes minus credit received)</li>
            </ul>
          </InfoBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">When to Use Iron Condors</h2>
          <DoDontGrid
            doItems={[
              'Low volatility environment',
              'Range-bound, sideways markets',
              'Stock trading in a consolidation pattern',
              'High implied volatility (premiums are expensive)',
              "You're neutral to slightly bearish or bullish"
            ]}
            dontItems={[
              'High volatility expected (earnings, news)',
              'Strong directional trend',
              'Low implied volatility (premiums too cheap)',
              'Uncertain market conditions',
              'Near major support/resistance breaks'
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Setting Up Your Iron Condor</h2>
          <p class="text-gray-700 mb-4">
            PriceWhisperer can help you identify the perfect Iron Condor setups:
          </p>
          <InfoBox>
            <FeatureList
              items={[
                { title: 'Volatility Analysis', description: 'Our system identifies when implied volatility is high enough to make Iron Condors profitable' },
                { title: 'Range Detection', description: 'We analyze support and resistance levels to determine optimal strike selection' },
                { title: 'Probability Calculation', description: 'Each suggested Iron Condor includes the probability of profit based on historical data' },
                { title: 'Risk/Reward Analysis', description: 'Automatic calculation of maximum profit, maximum loss, and break-even points' }
              ]}
            />
          </InfoBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Managing Your Position</h2>
          <SectionCardStack
            items={[
              {
                title: 'Taking Profits',
                description: "Close the position when you've captured 50-75% of maximum profit, or when there are 2-3 weeks left until expiration. Don't be greedy - take profits early to avoid giving them back."
              },
              {
                title: 'Managing Risk',
                description: 'If the stock price moves toward one of your short strikes, consider rolling the position (closing and reopening at different strikes) or closing early to limit losses. Never let an Iron Condor expire if it\'s in danger of being breached.'
              },
              {
                title: 'Adjustments',
                description: 'If the stock breaks through one side, you can "roll" the untested side closer to the current price to collect more premium, or convert to a different strategy like an Iron Butterfly.'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Common Mistakes to Avoid</h2>
          <MistakeList
            items={[
              { title: 'Strikes too narrow', description: "Don't place strikes too close together - you need room for the stock to move" },
              { title: 'Holding too long', description: 'Close positions with 2-3 weeks remaining to avoid gamma risk' },
              { title: 'Ignoring volatility', description: 'Only trade Iron Condors when IV is high enough to justify the risk' },
              { title: 'Not managing risk', description: 'Set stop-losses and exit rules before entering the trade' }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Using PriceWhisperer for Iron Condors</h2>
          <p class="text-gray-700 mb-4">
            PriceWhisperer's Options Strategy Finder automatically identifies Iron Condor opportunities:
          </p>
          <InfoBox>
            <FeatureList
              items={[
                { description: 'Analyzes current volatility to determine if Iron Condors are profitable' },
                { description: 'Identifies stocks in consolidation patterns perfect for Iron Condors' },
                { description: 'Suggests optimal strike selection based on support/resistance levels' },
                { description: 'Calculates probability of profit and risk/reward ratios' },
                { description: 'Provides entry, exit, and adjustment recommendations' }
              ]}
              showBold={false}
            />
          </InfoBox>

          <KeyTakeawaysBox
            items={[
              'Iron Condors profit from range-bound markets and high volatility',
              'Maximum profit is limited to the credit received',
              'Maximum risk is the difference between strikes minus the credit',
              'Close positions early (2-3 weeks before expiration) to lock in profits',
              'Use PriceWhisperer to identify the best Iron Condor setups automatically'
            ]}
          />
    </BlogPostLayout>
  );
};

export default IronCondorStrategy;
