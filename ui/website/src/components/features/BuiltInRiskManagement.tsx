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

const BuiltInRiskManagement: Component = () => {
  return (
    <FeatureLayout
      header={{
        title: 'Built-in Risk Management',
        subtitle: 'Protect your capital while you grow it',
        icon: 'fa-solid fa-shield-alt',
        iconBgColor: 'bg-indigo-600'
      }}
      showBackLink={true}
    >
          <KeyTakeawayBox color="indigo" title="What You Get" icon="fa-solid fa-shield">
            Automatic position sizing, stop-loss suggestions, and portfolio risk limits built into 
            every trade. Trade with confidence knowing your downside is controlled.
          </KeyTakeawayBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Risk Management Features</h2>
          <FeatureCardGrid
            columns={2}
            items={[
              {
                icon: 'fa-solid fa-calculator',
                iconColor: 'text-indigo-600',
                title: 'Position Sizing',
                description: 'Automatically calculates the right position size based on your account balance, risk tolerance, and stop-loss distance. Never risk more than you can afford to lose.'
              },
              {
                icon: 'fa-solid fa-stop-circle',
                iconColor: 'text-red-600',
                title: 'Stop-Loss Suggestions',
                description: 'Every trade recommendation includes a suggested stop-loss level based on support/resistance, ATR, and volatility. Protect your capital automatically.'
              },
              {
                icon: 'fa-solid fa-chart-pie',
                iconColor: 'text-primary',
                title: 'Portfolio Risk Limits',
                description: 'Set maximum exposure per sector, maximum total portfolio risk, and correlation limits. The system prevents over-concentration automatically.'
              },
              {
                icon: 'fa-solid fa-exclamation-triangle',
                iconColor: 'text-yellow-600',
                title: 'Risk Alerts',
                description: 'Get notified when your portfolio risk exceeds thresholds, when positions need adjustment, or when market conditions change your risk profile.'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Position Sizing Calculator</h2>
          <p class="text-gray-700 mb-4">
            Our position sizing algorithm considers multiple factors:
          </p>
          <InfoBox>
            <FeatureList
              items={[
                { title: 'Account Balance', description: 'Position size scales with your account - larger accounts can take larger positions' },
                { title: 'Risk Per Trade', description: 'Set your maximum risk per trade (e.g., 1% or 2% of account)' },
                { title: 'Stop-Loss Distance', description: 'Wider stops mean smaller position sizes to maintain risk limits' },
                { title: 'Volatility Adjustment', description: 'More volatile stocks get smaller position sizes automatically' },
                { title: 'Portfolio Correlation', description: 'Reduces position size if you already have correlated positions' }
              ]}
            />
          </InfoBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Portfolio-Level Protection</h2>
          <SectionCardStack
            items={[
              {
                title: 'Maximum Drawdown Limits',
                description: 'Set a maximum portfolio drawdown percentage. If your account drops by this amount, the system can automatically reduce position sizes or pause trading.',
                metadata: 'Protect against extended losing streaks',
                metadataIcon: 'fa-solid fa-shield-alt text-indigo-600'
              },
              {
                title: 'Sector Concentration Limits',
                description: 'Prevent over-exposure to a single sector. If tech stocks make up too much of your portfolio, the system won\'t suggest more tech trades.',
                metadata: 'Maintain portfolio diversification automatically',
                metadataIcon: 'fa-solid fa-chart-pie text-primary'
              },
              {
                title: 'Correlation Monitoring',
                description: 'Tracks how your positions move together. If you have multiple highly correlated positions, the system treats them as one large position for risk purposes.',
                metadata: 'Avoid hidden concentration risk',
                metadataIcon: 'fa-solid fa-link text-secondary'
              },
              {
                title: 'Leverage Controls',
                description: 'For margin accounts, set maximum leverage limits. The system ensures you never exceed your comfort level with borrowed capital.',
                metadata: 'Control margin usage automatically',
                metadataIcon: 'fa-solid fa-balance-scale text-accent'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Real-Time Risk Monitoring</h2>
          <p class="text-gray-700 mb-4">
            Your risk is calculated and updated in real-time:
          </p>
          <FeatureCardGrid
            columns={2}
            items={[
              {
                icon: 'fa-solid fa-exclamation-triangle',
                iconColor: 'text-red-600',
                title: 'Current Risk',
                description: 'Real-time calculation of total portfolio risk based on open positions'
              },
              {
                icon: 'fa-solid fa-chart-line',
                iconColor: 'text-yellow-600',
                title: 'Risk Per Trade',
                description: 'How much you\'re risking on each individual position'
              },
              {
                icon: 'fa-solid fa-check-circle',
                iconColor: 'text-blue-600',
                title: 'Available Risk',
                description: 'How much risk capacity you have remaining before hitting limits'
              },
              {
                icon: 'fa-solid fa-chart-bar',
                iconColor: 'text-green-600',
                title: 'Risk-Adjusted Returns',
                description: 'Performance metrics that account for the risk you\'re taking'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Stop-Loss Management</h2>
          <p class="text-gray-700 mb-4">
            Every trade includes intelligent stop-loss suggestions:
          </p>
          <InfoBox>
            <FeatureList
              items={[
                { title: 'Support/Resistance Based', description: 'Stops placed just beyond key technical levels' },
                { title: 'ATR-Based', description: 'Stops sized based on Average True Range for volatility-adjusted risk' },
                { title: 'Trailing Stops', description: 'Automatic stop adjustment as trades move in your favor' },
                { title: 'Breakeven Stops', description: 'Automatic stop adjustment to breakeven after certain profit levels' }
              ]}
            />
          </InfoBox>

          <KeyTakeawaysBox
            items={[
              <>Protect your capital with automatic position sizing and stop-losses</>,
              <>Never risk more than you can afford - the system enforces your limits</>,
              <>Sleep better knowing your downside is controlled</>,
              <>Maintain portfolio diversification automatically</>,
              <>Focus on trading while the system manages risk</>
            ]}
            title="Key Benefits"
          />

    </FeatureLayout>
  );
};

export default BuiltInRiskManagement;
