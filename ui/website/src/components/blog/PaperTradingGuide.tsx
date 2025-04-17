import { Component } from 'solid-js';
import BlogPostLayout from './BlogPostLayout';
import {
  KeyTakeawayBox,
  FeatureCardGrid,
  InfoBox,
  SectionCardStack,
  MistakeList,
  SuccessList,
  FeatureList,
  KeyTakeawaysBox
} from './components';

const PaperTradingGuide: Component = () => {
  return (
    <BlogPostLayout
      header={{
        category: 'Trading Basics',
        title: 'How to Use Paper Trading to Build Confidence',
        date: 'November 5, 2024',
        readTime: '6 min read',
        icon: 'fa-solid fa-flask',
        iconBgColor: 'bg-green-600'
      }}
      showBackLink={true}
    >
          <KeyTakeawayBox color="green">
            Paper trading lets you test PriceWhisperer's strategies and alerts with virtual capital. 
            Build confidence and learn the system before risking real money.
          </KeyTakeawayBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">What is Paper Trading?</h2>
          <p class="text-gray-700 mb-4">
            Paper trading (also called simulated or virtual trading) is practice trading with fake money. 
            You make real trading decisions using real market data, but no actual capital is at risk. 
            It's like a flight simulator for traders - you get the experience without the danger.
          </p>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Why Paper Trade with PriceWhisperer?</h2>
          <FeatureCardGrid
            columns={2}
            items={[
              {
                icon: 'fa-solid fa-shield-alt',
                iconColor: 'text-green-600',
                title: 'Zero Risk',
                description: 'Test strategies, learn the platform, and make mistakes without losing real money. Perfect for building confidence.'
              },
              {
                icon: 'fa-solid fa-chart-line',
                iconColor: 'text-primary',
                title: 'Real Market Data',
                description: 'Paper trades use live market data, so you experience the same conditions as real trading. No fake scenarios.'
              },
              {
                icon: 'fa-solid fa-user-graduate',
                iconColor: 'text-secondary',
                title: 'Learn the System',
                description: 'Get comfortable with PriceWhisperer\'s alerts, dashboard, and features before going live. Understand how everything works.'
              },
              {
                icon: 'fa-solid fa-chart-bar',
                iconColor: 'text-accent',
                title: 'Test Strategies',
                description: 'Try different trading approaches, see what works for your style, and refine your strategy before risking capital.'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Getting Started with Paper Trading</h2>
          <InfoBox>
            <ol class="list-decimal pl-6 text-gray-700 space-y-3">
              <li><strong>Create a Paper Account:</strong> Set up a paper trading account in PriceWhisperer with virtual capital (e.g., $10,000)</li>
              <li><strong>Configure Alerts:</strong> Set up the same alerts you would use for live trading - test how they work</li>
              <li><strong>Start Trading:</strong> When alerts fire, execute paper trades just like you would with real money</li>
              <li><strong>Track Performance:</strong> Monitor your paper trading results using the same analytics as live accounts</li>
              <li><strong>Learn and Adjust:</strong> Review what worked, what didn't, and refine your approach</li>
            </ol>
          </InfoBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">What to Practice</h2>
          <SectionCardStack
            items={[
              {
                title: 'Understanding Alerts',
                description: 'Learn to interpret PriceWhisperer\'s alerts. Understand what each alert means, when to act on it, and when to ignore it. Practice reading entry points, stop-losses, and profit targets.'
              },
              {
                title: 'Position Sizing',
                description: 'Practice sizing positions correctly. Use PriceWhisperer\'s position sizing calculator to understand how much to trade based on your risk tolerance.'
              },
              {
                title: 'Entry and Exit Timing',
                description: 'Practice entering trades at the right time and exiting at profit targets or stop-losses. Learn to stick to your plan even when emotions want you to do otherwise.'
              },
              {
                title: 'Risk Management',
                description: 'Practice using stop-losses, managing position sizes, and following risk management rules. See how these rules protect your capital in different market conditions.'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Common Paper Trading Mistakes</h2>
          <MistakeList
            items={[
              { title: 'Not taking it seriously', description: 'Treat paper trading like real trading - follow your rules, use proper position sizing' },
              { title: 'Overtrading', description: 'Just because it\'s free doesn\'t mean you should trade constantly - quality over quantity' },
              { title: 'Ignoring losses', description: 'Paper losses still teach lessons - analyze what went wrong' },
              { title: 'Not tracking performance', description: 'Use the same metrics you would for live trading to evaluate your paper results' }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">When to Go Live</h2>
          <p class="text-gray-700 mb-4">
            You're ready to transition to live trading when:
          </p>
          <SuccessList
            items={[
              "You've paper traded for at least 1-2 months consistently",
              "You're consistently profitable in paper trading (not just lucky)",
              "You understand PriceWhisperer's alerts and how to act on them",
              "You're comfortable with position sizing and risk management",
              "You can stick to your trading plan even when trades go against you"
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">PriceWhisperer Paper Trading Features</h2>
          <InfoBox>
            <FeatureList
              items={[
                { title: 'Real-Time Market Data', description: 'Paper trades use live market prices, so you experience real market conditions' },
                { title: 'Full Feature Access', description: 'All PriceWhisperer features work in paper trading - alerts, strategies, analytics' },
                { title: 'Performance Tracking', description: 'Same analytics and reporting as live accounts - track win rate, P&L, risk metrics' },
                { title: 'Seamless Transition', description: 'When ready, switch to live trading with the same interface and features' }
              ]}
            />
          </InfoBox>

          <KeyTakeawaysBox
            items={[
              'Paper trading lets you learn PriceWhisperer risk-free',
              'Use real market data to experience actual trading conditions',
              'Practice understanding alerts, position sizing, and risk management',
              'Track your performance to evaluate your readiness for live trading',
              'Only go live when you\'re consistently profitable in paper trading'
            ]}
          />

    </BlogPostLayout>
  );
};

export default PaperTradingGuide;
