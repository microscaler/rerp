import { Component } from 'solid-js';
import FeatureLayout from './FeatureLayout';
import {
  KeyTakeawayBox,
  FeatureCardGrid,
  InfoBox,
  FeatureList,
  KeyTakeawaysBox
} from '../blog/components';

const GlobalMarketCoverage: Component = () => {
  return (
    <FeatureLayout
      header={{
        title: 'Global Market Coverage',
        subtitle: 'Track opportunities across 20+ exchanges worldwide',
        icon: 'fa-solid fa-globe',
        iconBgColor: 'bg-accent'
      }}
      showBackLink={true}
    >
          <KeyTakeawayBox color="accent" title="What You Get" icon="fa-solid fa-globe-americas">
            Unified access to stocks, options, and market data from major exchanges around the world. 
            Never miss an opportunity because it's on a different exchange.
          </KeyTakeawayBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Supported Exchanges</h2>
          <p class="text-gray-700 mb-6">
            We monitor and provide real-time data from the world's major stock exchanges:
          </p>

          <FeatureCardGrid
            columns={2}
            items={[
              {
                icon: 'fa-solid fa-flag-usa',
                iconColor: 'text-primary',
                title: 'North America',
                description: (
                  <FeatureList
                    items={[
                      { description: <><strong>NYSE</strong> - New York Stock Exchange</> },
                      { description: <><strong>NASDAQ</strong> - National Association of Securities Dealers</> },
                      { description: <><strong>TSX</strong> - Toronto Stock Exchange</> },
                      { description: <><strong>AMEX</strong> - American Stock Exchange</> }
                    ]}
                    showBold={false}
                  />
                )
              },
              {
                icon: 'fa-solid fa-flag',
                iconColor: 'text-secondary',
                title: 'Europe',
                description: (
                  <FeatureList
                    items={[
                      { description: <><strong>LSE</strong> - London Stock Exchange</> },
                      { description: <><strong>Euronext</strong> - Pan-European Exchange</> },
                      { description: <><strong>XETR</strong> - Xetra (Frankfurt)</> },
                      { description: <><strong>SWX</strong> - SIX Swiss Exchange</> }
                    ]}
                    showBold={false}
                  />
                )
              },
              {
                icon: 'fa-solid fa-flag',
                iconColor: 'text-accent',
                title: 'Asia Pacific',
                description: (
                  <FeatureList
                    items={[
                      { description: <><strong>TSE</strong> - Tokyo Stock Exchange</> },
                      { description: <><strong>HKEX</strong> - Hong Kong Exchanges</> },
                      { description: <><strong>ASX</strong> - Australian Securities Exchange</> },
                      { description: <><strong>SSE</strong> - Shanghai Stock Exchange</> }
                    ]}
                    showBold={false}
                  />
                )
              },
              {
                icon: 'fa-solid fa-globe',
                iconColor: 'text-primary',
                title: 'Additional Markets',
                description: (
                  <FeatureList
                    items={[
                      { description: <>Brazil (B3), Mexico (BMV), India (NSE, BSE)</> },
                      { description: <>South Korea (KRX), Singapore (SGX)</> },
                      { description: <>And 10+ more exchanges worldwide</> }
                    ]}
                    showBold={false}
                  />
                )
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Unified Data Feed</h2>
          <p class="text-gray-700 mb-4">
            No matter which exchange a stock trades on, you get:
          </p>
          <InfoBox>
            <FeatureList
              items={[
                { title: 'Real-Time Prices', description: 'Live quotes updated in milliseconds across all exchanges' },
                { title: 'Unified Symbol Format', description: 'Consistent ticker symbols and naming across exchanges' },
                { title: 'Cross-Market Analysis', description: 'Compare stocks across exchanges to find arbitrage opportunities' },
                { title: 'Time Zone Handling', description: 'Automatic conversion to your local time zone' },
                { title: 'Currency Conversion', description: 'All prices displayed in your preferred currency' }
              ]}
            />
          </InfoBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Global Opportunities</h2>
          <FeatureCardGrid
            columns={2}
            items={[
              {
                icon: 'fa-solid fa-clock',
                iconColor: 'text-blue-600',
                title: '24/7 Market Coverage',
                description: 'As one market closes, another opens. Track opportunities around the clock as trading moves from Asia to Europe to the Americas.'
              },
              {
                icon: 'fa-solid fa-exchange-alt',
                iconColor: 'text-green-600',
                title: 'Arbitrage Detection',
                description: 'Our system identifies price discrepancies between exchanges, helping you spot arbitrage opportunities automatically.'
              },
              {
                icon: 'fa-solid fa-chart-pie',
                iconColor: 'text-purple-600',
                title: 'Sector Analysis',
                description: 'Compare how the same sector performs across different markets. Find the best opportunities in tech, finance, or energy globally.'
              },
              {
                icon: 'fa-solid fa-globe',
                iconColor: 'text-orange-600',
                title: 'ADRs & International Stocks',
                description: 'Track American Depositary Receipts (ADRs) and understand how international companies trade in multiple markets.'
              }
            ]}
          />

          <KeyTakeawaysBox
            items={[
              <>Never miss an opportunity because it's on a different exchange</>,
              <>Compare stocks across markets to find the best entry points</>,
              <>Diversify your portfolio across global markets easily</>,
              <>Get alerts for stocks on any exchange, all in one place</>,
              <>Understand global market correlations and how they affect your trades</>
            ]}
            title="Key Benefits"
          />

    </FeatureLayout>
  );
};

export default GlobalMarketCoverage;
