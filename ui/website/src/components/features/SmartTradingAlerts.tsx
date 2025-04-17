import { Component } from 'solid-js';
import FeatureLayout from './FeatureLayout';
import {
  KeyTakeawayBox,
  FeatureCardGrid,
  InfoBox,
  FeatureList,
  KeyTakeawaysBox
} from '../blog/components';

const SmartTradingAlerts: Component = () => {
  return (
    <FeatureLayout
      header={{
        title: 'Smart Trading Alerts',
        subtitle: 'Never miss a trading opportunity',
        icon: 'fa-solid fa-bell',
        iconBgColor: 'bg-primary'
      }}
      showBackLink={true}
    >
          <KeyTakeawayBox color="primary" title="What You Get" icon="fa-solid fa-info-circle">
            Real-time alerts delivered to your phone, email, or dashboard the moment our AI detects 
            unusual market activity worth your attention.
          </KeyTakeawayBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">How It Works</h2>
          <p class="text-gray-700 mb-4">
            Our AI continuously monitors thousands of stocks across global exchanges, analyzing multiple 
            signals simultaneously to identify the most interesting opportunities:
          </p>

          <FeatureCardGrid
            columns={2}
            items={[
              {
                icon: 'fa-solid fa-chart-line',
                iconColor: 'text-primary',
                title: 'Volume Analysis',
                description: 'Detects unusual volume spikes that often precede significant price movements. We compare current volume to historical averages to spot anomalies.'
              },
              {
                icon: 'fa-solid fa-arrow-trend-up',
                iconColor: 'text-secondary',
                title: 'Price Action Signals',
                description: 'Identifies breakouts, reversals, and momentum shifts in real-time. Our system recognizes when price action suggests a high-probability trade setup.'
              },
              {
                icon: 'fa-solid fa-options',
                iconColor: 'text-accent',
                title: 'Options Flow Intelligence',
                description: 'Tracks unusual options activity that smart money uses. Large block trades, unusual call/put ratios, and dark pool activity can signal upcoming moves.'
              },
              {
                icon: 'fa-solid fa-brain',
                iconColor: 'text-purple-600',
                title: 'AI Pattern Recognition',
                description: 'Machine learning models identify complex patterns across multiple timeframes and indicators that human traders might miss.'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Alert Types</h2>
          <FeatureList
            items={[
              { description: <><strong>Breakout Alerts:</strong> Notified when stocks break above key resistance levels or below support, often signaling trend continuation.</> },
              { description: <><strong>Volume Spikes:</strong> Alerts when trading volume exceeds normal levels by 2x, 5x, or more, indicating institutional interest.</> },
              { description: <><strong>Options Activity:</strong> Unusual options flow that suggests informed trading or hedging activity.</> },
              { description: <><strong>Pattern Completion:</strong> Alerts when chart patterns like triangles, flags, or head-and-shoulders complete.</> },
              { description: <><strong>Momentum Shifts:</strong> Early detection of trend reversals based on multiple technical indicators aligning.</> },
              { description: <><strong>News Correlation:</strong> Price movements that correlate with breaking news, earnings, or economic data.</> }
            ]}
            showBold={false}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Customizable Filters</h2>
          <p class="text-gray-700 mb-4">
            Set up alerts that match your trading style:
          </p>
          <InfoBox>
            <FeatureList
              items={[
                { title: 'Market Cap Filters', description: 'Focus on large-cap, mid-cap, or small-cap stocks' },
                { title: 'Sector Selection', description: 'Get alerts only for industries you understand and trade' },
                { title: 'Price Range', description: 'Filter by stock price to match your account size' },
                { title: 'Volatility Preferences', description: 'Choose between high-volatility momentum plays or stable swing trades' },
                { title: 'Time-of-Day Filters', description: 'Get alerts during market hours that match your schedule' }
              ]}
            />
          </InfoBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Delivery Options</h2>
          <FeatureCardGrid
            columns={3}
            items={[
              {
                icon: 'fa-solid fa-mobile-alt',
                iconColor: 'text-primary',
                title: 'Mobile Push',
                description: 'Instant notifications on your phone, even when the app is closed'
              },
              {
                icon: 'fa-solid fa-envelope',
                iconColor: 'text-secondary',
                title: 'Email',
                description: 'Daily digest or real-time alerts delivered to your inbox'
              },
              {
                icon: 'fa-solid fa-browser',
                iconColor: 'text-accent',
                title: 'Dashboard',
                description: 'See all alerts in your live trading dashboard with full context'
              }
            ]}
          />

          <KeyTakeawaysBox
            items={[
              <>Never miss a trading opportunity - alerts arrive within seconds of detection</>,
              <>Save hours of manual chart scanning - let AI do the work</>,
              <>Get context with every alert - entry points, stop-loss levels, and profit targets included</>,
              <>Filter noise from signal - only the most interesting opportunities reach you</>,
              <>Trade with confidence - alerts are based on proven patterns and real market data</>
            ]}
            title="Key Benefits"
          />

    </FeatureLayout>
  );
};

export default SmartTradingAlerts;
