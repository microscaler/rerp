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

const ExecutionOptions: Component = () => {
  return (
    <FeatureLayout
      header={{
        title: 'Execution Options',
        subtitle: 'Paper trading and live account integration',
        icon: 'fa-solid fa-exchange-alt',
        iconBgColor: 'bg-green-600'
      }}
      showBackLink={true}
    >
          <KeyTakeawayBox color="green" title="What You Get" icon="fa-solid fa-shield-alt">
            Trade with confidence using paper accounts to test strategies risk-free, then seamlessly 
            transition to live trading with integrated broker connections.
          </KeyTakeawayBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Paper Trading - Build Confidence Risk-Free</h2>
          <p class="text-gray-700 mb-4">
            Before risking real capital, test PriceWhisperer's strategies and alerts in a completely 
            risk-free paper trading environment. Our paper trading system simulates real market conditions 
            so you can:
          </p>

          <FeatureCardGrid
            columns={2}
            items={[
              {
                icon: 'fa-solid fa-flask',
                iconColor: 'text-green-600',
                title: 'Test Strategies',
                description: 'Try different trading strategies, test alert accuracy, and see how our system performs without risking a single dollar.'
              },
              {
                icon: 'fa-solid fa-chart-line',
                iconColor: 'text-primary',
                title: 'Real Market Data',
                description: 'Paper trades use real-time market data, so you experience the same conditions as live trading - just without the risk.'
              },
              {
                icon: 'fa-solid fa-user-graduate',
                iconColor: 'text-secondary',
                title: 'Learn the System',
                description: 'Get comfortable with PriceWhisperer\'s interface, understand how alerts work, and learn to interpret signals before going live.'
              },
              {
                icon: 'fa-solid fa-chart-bar',
                iconColor: 'text-accent',
                title: 'Track Performance',
                description: 'Monitor your paper trading performance with the same analytics and reporting tools available for live accounts.'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Live Account Integration</h2>
          <p class="text-gray-700 mb-4">
            When you're ready to trade with real capital, PriceWhisperer integrates seamlessly with 
            leading brokers for automated execution:
          </p>

          <SectionCardStack
            items={[
              {
                title: 'Interactive Brokers (IBKR)',
                description: (
                  <>
                    <p class="mb-3">Full integration with Interactive Brokers' API for automated order execution. Trade stocks, options, futures, and more across global markets.</p>
                    <ul class="list-disc pl-6 text-gray-700 text-sm space-y-1">
                      <li>Real-time market data and order execution</li>
                      <li>Support for stocks, options, futures, and forex</li>
                      <li>Advanced order types (limit, stop, bracket orders)</li>
                      <li>Portfolio and position management</li>
                      <li>Risk management and compliance checks</li>
                    </ul>
                  </>
                ),
                icon: 'fa-solid fa-building',
                iconColor: 'text-primary'
              },
              {
                title: 'Alpaca Markets',
                description: (
                  <>
                    <p class="mb-3">Commission-free stock and options trading with modern API integration. Perfect for algorithmic trading and automated strategies.</p>
                    <ul class="list-disc pl-6 text-gray-700 text-sm space-y-1">
                      <li>Commission-free trading</li>
                      <li>Paper trading environment included</li>
                      <li>Real-time and historical market data</li>
                      <li>REST and WebSocket API support</li>
                      <li>Fractional shares support</li>
                    </ul>
                  </>
                ),
                icon: 'fa-solid fa-chart-candlestick',
                iconColor: 'text-secondary'
              },
              {
                title: 'Other Broker Integrations',
                description: (
                  <>
                    <p class="mb-3">We're continuously adding support for additional brokers. Contact us to request integration with your preferred broker.</p>
                    <ul class="list-disc pl-6 text-gray-700 text-sm space-y-1">
                      <li>TD Ameritrade / Charles Schwab</li>
                      <li>E*TRADE</li>
                      <li>Fidelity</li>
                      <li>Binance (crypto)</li>
                      <li>Custom broker API integration available</li>
                    </ul>
                  </>
                ),
                icon: 'fa-solid fa-globe',
                iconColor: 'text-accent'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">How It Works</h2>
          <InfoBox>
            <ol class="list-decimal pl-6 text-gray-700 space-y-3">
              <li><strong>Start with Paper Trading:</strong> Create a paper account and test PriceWhisperer's alerts and strategies with virtual capital. No risk, all the learning.</li>
              <li><strong>Build Confidence:</strong> See how our system performs, understand the alerts, and get comfortable with the platform before risking real money.</li>
              <li><strong>Connect Your Broker:</strong> When ready, connect your Interactive Brokers, Alpaca, or other supported broker account securely via API.</li>
              <li><strong>Go Live:</strong> Enable live trading and let PriceWhisperer execute trades automatically based on your configured strategies and risk parameters.</li>
              <li><strong>Monitor & Adjust:</strong> Track performance, adjust strategies, and refine your approach using real-time analytics and reporting.</li>
            </ol>
          </InfoBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Security & Safety</h2>
          <FeatureCardGrid
            columns={2}
            items={[
              {
                icon: 'fa-solid fa-lock',
                iconColor: 'text-green-600',
                title: 'API Key Security',
                description: 'Broker API keys are encrypted and stored securely. You maintain full control and can revoke access at any time.'
              },
              {
                icon: 'fa-solid fa-shield-alt',
                iconColor: 'text-blue-600',
                title: 'Risk Limits',
                description: 'Set maximum position sizes, daily loss limits, and other risk controls that are enforced before any trade is executed.'
              },
              {
                icon: 'fa-solid fa-check-circle',
                iconColor: 'text-purple-600',
                title: 'Trade Confirmation',
                description: 'Review and approve trades before execution, or enable fully automated trading based on your preferences.'
              },
              {
                icon: 'fa-solid fa-clipboard-list',
                iconColor: 'text-orange-600',
                title: 'Audit Trail',
                description: 'Complete log of all trades, executions, and system actions for compliance and performance analysis.'
              }
            ]}
          />

          <KeyTakeawaysBox
            items={[
              <>Build confidence with risk-free paper trading before going live</>,
              <>Seamless transition from paper to live trading with the same interface</>,
              <>Integrated with major brokers like Interactive Brokers and Alpaca</>,
              <>Automated execution based on your configured strategies and risk limits</>,
              <>Full control over when and how trades are executed</>,
              <>Complete audit trail for compliance and performance tracking</>
            ]}
            title="Key Benefits"
          />

    </FeatureLayout>
  );
};

export default ExecutionOptions;
