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

const PatternRecognition: Component = () => {
  return (
    <FeatureLayout
      header={{
        title: 'Pattern Recognition',
        subtitle: 'Trade the patterns that work',
        icon: 'fa-solid fa-search',
        iconBgColor: 'bg-orange-600'
      }}
      showBackLink={true}
    >
          <KeyTakeawayBox color="orange" title="What You Get" icon="fa-solid fa-brain">
            Our AI automatically scans thousands of stocks to find profitable chart patterns. 
            No more manual chart scanning - let the system find the setups for you.
          </KeyTakeawayBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">How It Works</h2>
          <p class="text-gray-700 mb-4">
            Our pattern recognition engine uses advanced algorithms to identify proven chart patterns 
            across multiple timeframes. We analyze:
          </p>

          <FeatureCardGrid
            columns={2}
            items={[
              {
                icon: 'fa-solid fa-chart-candlestick',
                iconColor: 'text-orange-600',
                title: 'Candlestick Patterns',
                description: 'Detects classic patterns like Engulfing, Hammer, Doji, and more. Each pattern is validated against historical success rates.'
              },
              {
                icon: 'fa-solid fa-chart-line',
                iconColor: 'text-primary',
                title: 'Chart Patterns',
                description: 'Identifies triangles, flags, head-and-shoulders, double tops/bottoms, and other classic technical patterns.'
              },
              {
                icon: 'fa-solid fa-wave-square',
                iconColor: 'text-secondary',
                title: 'Support & Resistance',
                description: 'Automatically detects key support and resistance levels based on price history, volume, and consolidation zones.'
              },
              {
                icon: 'fa-solid fa-arrows-up-down',
                iconColor: 'text-accent',
                title: 'Breakout Patterns',
                description: 'Finds consolidation patterns that are about to break out, with volume confirmation and momentum indicators.'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Detected Patterns</h2>
          <SectionCardStack
            items={[
              {
                title: 'Bullish Engulfing',
                description: 'Strong reversal pattern where a bullish candle completely engulfs the previous bearish candle. Our system validates volume and location for maximum probability.',
                metadata: 'Best for: Reversal trades after downtrends',
                metadataIcon: 'fa-solid fa-check-circle text-secondary'
              },
              {
                title: 'Hammer Reversal',
                description: 'Single-candle pattern showing rejection of lower prices. We confirm with volume spikes and support level proximity.',
                metadata: 'Best for: Bottom-fishing and reversal entries',
                metadataIcon: 'fa-solid fa-check-circle text-secondary'
              },
              {
                title: 'Breakout & Retest',
                description: 'Identifies when price breaks above resistance, then retests that level as support. High-probability continuation pattern.',
                metadata: 'Best for: Trend-following and momentum trades',
                metadataIcon: 'fa-solid fa-check-circle text-secondary'
              },
              {
                title: 'Inside Bar (NR7)',
                description: 'Narrow range pattern indicating consolidation. When it breaks, it often leads to significant moves. We identify the narrowest range in weeks.',
                metadata: 'Best for: Volatility expansion trades',
                metadataIcon: 'fa-solid fa-check-circle text-secondary'
              },
              {
                title: 'Opening Range Breakout',
                description: 'Detects when price breaks the first 15 minutes\' range. High-probability intraday pattern with clear entry and stop levels.',
                metadata: 'Best for: Day trading and intraday momentum',
                metadataIcon: 'fa-solid fa-check-circle text-secondary'
              },
              {
                title: 'Trend Pullback to EMA',
                description: 'Finds pullbacks to moving averages in strong trends. We validate trend strength and entry timing for optimal risk/reward.',
                metadata: 'Best for: Trend-following and swing trading',
                metadataIcon: 'fa-solid fa-check-circle text-secondary'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Pattern Validation</h2>
          <p class="text-gray-700 mb-4">
            Not all patterns are created equal. Our system validates each pattern with:
          </p>
          <InfoBox>
            <FeatureList
              items={[
                { title: 'Volume Confirmation', description: 'Patterns with volume spikes are more reliable' },
                { title: 'Location Analysis', description: 'Patterns near support/resistance are higher probability' },
                { title: 'Trend Context', description: 'Patterns that align with the larger trend are preferred' },
                { title: 'Historical Success Rate', description: 'We track which patterns work best in current market conditions' },
                { title: 'Multiple Timeframe Confirmation', description: 'Patterns that appear on multiple timeframes are stronger' }
              ]}
            />
          </InfoBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Pattern Alerts</h2>
          <p class="text-gray-700 mb-4">
            When a pattern is detected, you get an alert with:
          </p>
          <FeatureCardGrid
            columns={2}
            items={[
              {
                icon: 'fa-solid fa-arrow-right',
                iconColor: 'text-blue-600',
                title: 'Entry Point',
                description: 'Exact price level to enter the trade'
              },
              {
                icon: 'fa-solid fa-stop-circle',
                iconColor: 'text-red-600',
                title: 'Stop Loss',
                description: 'Clear exit point if the pattern fails'
              },
              {
                icon: 'fa-solid fa-bullseye',
                iconColor: 'text-green-600',
                title: 'Profit Target',
                description: 'Where to take profits based on pattern history'
              },
              {
                icon: 'fa-solid fa-chart-line',
                iconColor: 'text-purple-600',
                title: 'Confidence Score',
                description: 'How strong the pattern signal is (0-100%)'
              }
            ]}
          />

          <KeyTakeawaysBox
            items={[
              <>Save hours of manual chart scanning - let AI find the patterns</>,
              <>Trade proven patterns with historical success rates, not guesswork</>,
              <>Get clear entry, stop, and target levels with every pattern</>,
              <>Focus on high-probability setups validated by multiple factors</>,
              <>Learn which patterns work best in different market conditions</>
            ]}
            title="Key Benefits"
          />

    </FeatureLayout>
  );
};

export default PatternRecognition;
