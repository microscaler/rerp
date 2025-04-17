import { Component } from 'solid-js';
import BlogPostLayout from './BlogPostLayout';
import {
  KeyTakeawayBox,
  InfoBox,
  FeatureList,
  SectionCardStack,
  DoDontGrid,
  KeyTakeawaysBox
} from './components';

const PatternRecognitionGuide: Component = () => {
  return (
    <BlogPostLayout
      header={{
        category: 'Technical Analysis',
        title: 'Chart Pattern Recognition: A Complete Guide',
        date: 'November 12, 2024',
        readTime: '10 min read',
        icon: 'fa-solid fa-search',
        iconBgColor: 'bg-orange-600'
      }}
      showBackLink={true}
    >
          <KeyTakeawayBox color="orange">
            Chart patterns are visual representations of market psychology. PriceWhisperer uses 
            AI to automatically identify these patterns, giving you an edge in finding high-probability 
            trading setups.
          </KeyTakeawayBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">What is Chart Pattern Recognition?</h2>
          <p class="text-gray-700 mb-4">
            Chart pattern recognition is the process of identifying recurring shapes and formations in 
            price charts that historically lead to predictable price movements. These patterns form 
            because human psychology and market behavior tend to repeat, creating recognizable structures 
            that traders can use to anticipate future price action.
          </p>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">How PriceWhisperer Uses AI for Pattern Recognition</h2>
          <p class="text-gray-700 mb-4">
            PriceWhisperer's pattern recognition engine uses advanced algorithms to scan thousands of 
            stocks in real-time, identifying profitable patterns that human traders might miss:
          </p>
          <InfoBox>
            <FeatureList
              items={[
                { title: 'Multi-Timeframe Analysis', description: 'Patterns are detected across multiple timeframes for confirmation' },
                { title: 'Volume Confirmation', description: 'Patterns with volume spikes are prioritized' },
                { title: 'Historical Validation', description: 'Each pattern is validated against historical success rates' },
                { title: 'Context Awareness', description: 'Patterns are evaluated within the larger trend and market structure' }
              ]}
            />
          </InfoBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Breakout Patterns</h2>
          <p class="text-gray-700 mb-4">
            Breakout patterns occur when price breaks through a key level of support or resistance, 
            often leading to significant moves:
          </p>
          <SectionCardStack
            items={[
              {
                title: 'Triangle Breakouts',
                description: 'Ascending, descending, and symmetrical triangles form as price consolidates. The breakout direction often continues the prior trend. PriceWhisperer identifies triangles forming and alerts you when the breakout occurs.',
                metadata: 'Best for: Trend continuation trades',
                metadataIcon: 'fa-solid fa-chart-line text-primary'
              },
              {
                title: 'Rectangle/Range Breakouts',
                description: 'When price trades in a horizontal range, the breakout above or below often leads to significant moves. Our system tracks these ranges and alerts on breakouts with volume confirmation.',
                metadata: 'Best for: Momentum trades after consolidation',
                metadataIcon: 'fa-solid fa-chart-line text-primary'
              },
              {
                title: 'Cup and Handle',
                description: 'A bullish pattern where price forms a "cup" shape followed by a small "handle" consolidation. The breakout from the handle often leads to strong upward moves.',
                metadata: 'Best for: Long-term bullish positions',
                metadataIcon: 'fa-solid fa-chart-line text-primary'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Reversal Patterns</h2>
          <p class="text-gray-700 mb-4">
            Reversal patterns signal that a trend may be ending and a new trend beginning:
          </p>
          <SectionCardStack
            items={[
              {
                title: 'Head and Shoulders',
                description: 'A bearish reversal pattern with three peaks - a higher middle peak (head) and two lower peaks (shoulders). The breakdown below the "neckline" confirms the reversal.',
                metadata: 'Best for: Trend reversal trades',
                metadataIcon: 'fa-solid fa-arrow-down text-red-600'
              },
              {
                title: 'Double Top/Bottom',
                description: 'When price tests a level twice and fails to break through, it often reverses. Double tops are bearish, double bottoms are bullish.',
                metadata: 'Best for: Reversal trades at key levels',
                metadataIcon: 'fa-solid fa-arrows-up-down text-primary'
              },
              {
                title: 'Engulfing Patterns',
                description: 'Bullish or bearish candlestick patterns where one candle completely engulfs the previous. PriceWhisperer identifies these with volume confirmation for higher probability.',
                metadata: 'Best for: Short-term reversal trades',
                metadataIcon: 'fa-solid fa-chart-candlestick text-secondary'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Continuation Patterns</h2>
          <p class="text-gray-700 mb-4">
            Continuation patterns suggest the current trend will resume after a brief pause:
          </p>
          <SectionCardStack
            items={[
              {
                title: 'Flags and Pennants',
                description: 'Small consolidation patterns that form during strong trends. The breakout typically continues in the direction of the prior trend with similar momentum.',
                metadata: 'Best for: Trend-following trades',
                metadataIcon: 'fa-solid fa-flag text-primary'
              },
              {
                title: 'Wedges',
                description: 'Rising or falling wedges that form as price consolidates. Rising wedges are typically bearish, falling wedges are typically bullish.',
                metadata: 'Best for: Trend continuation or reversal depending on context',
                metadataIcon: 'fa-solid fa-chart-line text-primary'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">How PriceWhisperer Alerts Work</h2>
          <p class="text-gray-700 mb-4">
            When PriceWhisperer detects a pattern, you receive an alert with:
          </p>
          <InfoBox>
            <FeatureList
              items={[
                { title: 'Pattern Type', description: 'What pattern was detected (breakout, reversal, continuation)' },
                { title: 'Entry Point', description: 'Exact price level to enter the trade' },
                { title: 'Stop Loss', description: 'Clear exit point if the pattern fails' },
                { title: 'Profit Target', description: 'Where to take profits based on pattern history' },
                { title: 'Confidence Score', description: 'How strong the pattern signal is (0-100%)' },
                { title: 'Historical Success Rate', description: 'How often this pattern has worked in similar conditions' }
              ]}
            />
          </InfoBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Best Practices</h2>
          <DoDontGrid
            doItems={[
              'Wait for pattern confirmation before entering',
              'Use volume to validate pattern breakouts',
              'Consider the larger trend context',
              'Set stop-losses based on pattern structure',
              'Take profits at pattern-based targets'
            ]}
            dontItems={[
              'Trade patterns without confirmation',
              'Ignore volume on breakouts',
              'Trade against the larger trend',
              'Hold patterns that fail',
              'Overtrade - wait for quality setups'
            ]}
          />

          <KeyTakeawaysBox
            items={[
              'Chart patterns reflect market psychology and tend to repeat',
              'PriceWhisperer uses AI to identify patterns across thousands of stocks automatically',
              'Patterns are validated with volume, context, and historical success rates',
              'Always wait for pattern confirmation before entering trades',
              'Use stop-losses and profit targets based on pattern structure'
            ]}
          />

    </BlogPostLayout>
  );
};

export default PatternRecognitionGuide;
