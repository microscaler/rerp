import { Component } from 'solid-js';
import BlogPostLayout from './BlogPostLayout';
import {
  KeyTakeawayBox,
  FeatureCardGrid,
  SectionCardStack,
  FeatureList,
  KeyTakeawaysBox,
  InfoBox
} from './components';

const MultiExchangeTrading: Component = () => {
  return (
    <BlogPostLayout
      header={{
        category: 'Market Analysis',
        title: 'Trading Across Global Exchanges: A Complete Guide',
        date: 'November 3, 2024',
        readTime: '9 min read',
        icon: 'fa-solid fa-globe',
        iconBgColor: 'bg-accent'
      }}
      showBackLink={true}
    >
          <KeyTakeawayBox color="accent">
            Trading across global exchanges opens up opportunities 24/7. PriceWhisperer tracks 
            stocks across 20+ exchanges worldwide, helping you find the best opportunities no 
            matter where they are.
          </KeyTakeawayBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Why Trade Global Markets?</h2>
          <p class="text-gray-700 mb-4">
            Limiting yourself to one exchange means missing opportunities. Global markets offer:
          </p>
          <FeatureCardGrid
            columns={2}
            items={[
              {
                icon: 'fa-solid fa-clock',
                iconColor: 'text-primary',
                title: '24/7 Opportunities',
                description: 'As one market closes, another opens. Track opportunities around the clock as trading moves from Asia to Europe to the Americas.'
              },
              {
                icon: 'fa-solid fa-chart-pie',
                iconColor: 'text-secondary',
                title: 'Diversification',
                description: 'Spread risk across different markets, currencies, and economic regions. Reduce exposure to any single market\'s volatility.'
              },
              {
                icon: 'fa-solid fa-exchange-alt',
                iconColor: 'text-accent',
                title: 'Arbitrage Opportunities',
                description: 'Price discrepancies between exchanges can create arbitrage opportunities. PriceWhisperer identifies these automatically.'
              },
              {
                icon: 'fa-solid fa-industry',
                iconColor: 'text-primary',
                title: 'Sector Analysis',
                description: 'Compare how the same sector performs across different markets. Find the best opportunities in tech, finance, or energy globally.'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Major Global Exchanges</h2>
          <SectionCardStack
            items={[
              {
                title: 'North America',
                description: 'NYSE and NASDAQ are the world\'s largest stock exchanges. PriceWhisperer tracks all major US stocks with real-time data and alerts.'
              },
              {
                title: 'Europe',
                description: 'LSE, Euronext, and XETRA offer access to European markets. Track opportunities across the UK, France, Germany, and more.'
              },
              {
                title: 'Asia Pacific',
                description: 'TSE, HKEX, ASX, and SSE provide access to Asian markets. These exchanges often move first, setting the tone for global markets.'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Using PriceWhisperer for Global Trading</h2>
          <InfoBox>
            <FeatureList
              items={[
                { title: 'Unified Alerts', description: 'Get alerts for stocks on any exchange, all in one place' },
                { title: 'Currency Conversion', description: 'All prices displayed in your preferred currency automatically' },
                { title: 'Time Zone Handling', description: 'Alerts and data automatically converted to your local time' },
                { title: 'Cross-Market Analysis', description: 'Compare stocks across exchanges to find the best entry points' }
              ]}
            />
          </InfoBox>

          <KeyTakeawaysBox
            items={[
              'Global markets offer 24/7 trading opportunities',
              'PriceWhisperer tracks stocks across 20+ exchanges worldwide',
              'Unified alerts and analysis make global trading simple',
              'Currency and time zone conversions handled automatically'
            ]}
          />

    </BlogPostLayout>
  );
};

export default MultiExchangeTrading;
