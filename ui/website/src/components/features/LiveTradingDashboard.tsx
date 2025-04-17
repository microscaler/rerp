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

const LiveTradingDashboard: Component = () => {
  return (
    <FeatureLayout
      header={{
        title: 'Live Trading Dashboard',
        subtitle: 'Monitor everything in real-time',
        icon: 'fa-solid fa-tachometer-alt',
        iconBgColor: 'bg-purple-600'
      }}
      showBackLink={true}
    >
          <KeyTakeawayBox color="purple" title="What You Get" icon="fa-solid fa-desktop">
            A comprehensive, customizable dashboard that shows everything you need to make trading 
            decisions - all updating in real-time.
          </KeyTakeawayBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Dashboard Features</h2>
          <FeatureCardGrid
            columns={2}
            items={[
              {
                icon: 'fa-solid fa-list',
                iconColor: 'text-purple-600',
                title: 'Watchlist Manager',
                description: 'Track your favorite stocks with real-time price updates, volume, and percentage changes. Organize by sector, strategy, or custom categories.'
              },
              {
                icon: 'fa-solid fa-briefcase',
                iconColor: 'text-primary',
                title: 'Portfolio Overview',
                description: 'See all your open positions at a glance with real-time P&L, position sizes, and risk metrics. Track performance across all your trades.'
              },
              {
                icon: 'fa-solid fa-bell',
                iconColor: 'text-secondary',
                title: 'Active Alerts',
                description: 'View all your active alerts in one place. See which stocks are triggering signals and why, with full context for each alert.'
              },
              {
                icon: 'fa-solid fa-chart-candlestick',
                iconColor: 'text-accent',
                title: 'Live Charts',
                description: 'Interactive charts with multiple timeframes, technical indicators, and drawing tools. All updating in real-time as markets move.'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Customizable Widgets</h2>
          <p class="text-gray-700 mb-4">
            Build your perfect dashboard with drag-and-drop widgets:
          </p>
          <SectionCardStack
            items={[
              {
                title: 'Market Overview',
                description: 'Major indices (S&P 500, NASDAQ, Dow), sector performance, and market breadth indicators all in one widget.',
                metadata: 'Real-time updates, color-coded performance',
                metadataIcon: 'fa-solid fa-check-circle text-secondary'
              },
              {
                title: 'Top Movers',
                description: 'See the biggest gainers and losers, highest volume stocks, and most active options in real-time.',
                metadata: 'Filter by exchange, sector, or market cap',
                metadataIcon: 'fa-solid fa-check-circle text-secondary'
              },
              {
                title: 'Options Flow',
                description: 'Track unusual options activity, large block trades, and dark pool prints that might signal big moves.',
                metadata: 'Real-time options data with Greeks',
                metadataIcon: 'fa-solid fa-check-circle text-secondary'
              },
              {
                title: 'Risk Metrics',
                description: 'Portfolio-level risk analysis including total exposure, correlation risk, and position concentration.',
                metadata: 'Real-time risk calculations',
                metadataIcon: 'fa-solid fa-check-circle text-secondary'
              },
              {
                title: 'Performance Analytics',
                description: 'Track your trading performance with win rate, average profit/loss, Sharpe ratio, and other key metrics.',
                metadata: 'Historical and real-time performance tracking',
                metadataIcon: 'fa-solid fa-check-circle text-secondary'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Real-Time Updates</h2>
          <p class="text-gray-700 mb-4">
            Everything updates automatically as markets move:
          </p>
          <InfoBox>
            <FeatureList
              items={[
                { title: 'Sub-Second Latency', description: 'Price updates arrive within milliseconds of market changes' },
                { title: 'Auto-Refresh', description: 'All widgets update automatically - no manual refresh needed' },
                { title: 'Alert Integration', description: 'New alerts appear instantly in your dashboard' },
                { title: 'Live Charts', description: 'Candlestick charts update tick-by-tick during market hours' }
              ]}
            />
          </InfoBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Mobile & Desktop</h2>
          <FeatureCardGrid
            columns={2}
            items={[
              {
                icon: 'fa-solid fa-desktop',
                iconColor: 'text-primary',
                title: 'Desktop Dashboard',
                description: 'Full-featured dashboard with multiple columns, large charts, and comprehensive data views. Perfect for serious analysis.'
              },
              {
                icon: 'fa-solid fa-mobile-alt',
                iconColor: 'text-secondary',
                title: 'Mobile App',
                description: 'Streamlined mobile view with essential widgets. Check your positions and alerts on the go, anywhere.'
              }
            ]}
          />

          <KeyTakeawaysBox
            items={[
              <>See everything you need in one place - no switching between multiple tabs</>,
              <>Customize your view to match your trading style and preferences</>,
              <>Real-time updates mean you're always working with current data</>,
              <>Mobile access lets you monitor markets even when you're away from your desk</>,
              <>Save time by having all your key metrics visible at once</>
            ]}
            title="Key Benefits"
          />

    </FeatureLayout>
  );
};

export default LiveTradingDashboard;
