import { Component } from 'solid-js';
import BlogPostLayout from './BlogPostLayout';
import {
  KeyTakeawayBox,
  FeatureCardGrid,
  ExampleBox,
  InfoBox,
  FeatureList,
  SectionCardStack,
  KeyTakeawaysBox
} from './components';

const OptionsGreeksExplained: Component = () => {
  return (
    <BlogPostLayout
      header={{
        category: 'Options Trading',
        title: 'Understanding Options Greeks: Delta, Gamma, Theta, Vega',
        date: 'November 8, 2024',
        readTime: '15 min read',
        icon: 'fa-solid fa-calculator',
        iconBgColor: 'bg-secondary'
      }}
      showBackLink={true}
    >
          <KeyTakeawayBox color="secondary">
            The Greeks measure how options prices change in response to various factors. Understanding 
            them is essential for successful options trading. PriceWhisperer calculates and displays 
            Greeks for every options strategy.
          </KeyTakeawayBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">What Are the Greeks?</h2>
          <p class="text-gray-700 mb-4">
            The "Greeks" are mathematical measures that describe how an option's price will change in 
            response to changes in underlying factors like stock price, time, and volatility. They're 
            called Greeks because they're typically represented by Greek letters: Delta (Δ), Gamma (Γ), 
            Theta (Θ), and Vega (ν).
          </p>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Delta (Δ) - Price Sensitivity</h2>
          <p class="text-gray-700 mb-4">
            Delta measures how much an option's price will change for every $1 move in the underlying 
            stock price. It's the most important Greek for most traders.
          </p>
          <FeatureCardGrid
            columns={2}
            items={[
              {
                icon: 'fa-solid fa-arrow-up',
                iconColor: 'text-green-600',
                title: 'Call Options',
                description: 'Delta ranges from 0 to 1 (or 0 to 100). At-the-money calls: ~0.50 delta. Deep in-the-money: ~1.0 delta (moves 1:1 with stock). Deep out-of-the-money: ~0.0 delta (little price movement).'
              },
              {
                icon: 'fa-solid fa-arrow-down',
                iconColor: 'text-red-600',
                title: 'Put Options',
                description: 'Delta ranges from -1 to 0 (or -100 to 0). At-the-money puts: ~-0.50 delta. Deep in-the-money: ~-1.0 delta. Deep out-of-the-money: ~0.0 delta.'
              }
            ]}
          />
          <ExampleBox title="Example" variant="blue">
            A call option with 0.60 delta will gain $0.60 for every $1 the stock moves up, and lose 
            $0.60 for every $1 the stock moves down. If you own 10 contracts (1,000 shares), a $1 stock 
            move = $600 profit or loss.
          </ExampleBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Gamma (Γ) - Delta Sensitivity</h2>
          <p class="text-gray-700 mb-4">
            Gamma measures how much Delta will change for every $1 move in the underlying stock. It's 
            the "rate of change" of Delta. High gamma means Delta changes quickly as the stock moves.
          </p>
          <InfoBox>
            <FeatureList
              items={[
                { title: 'Highest at-the-money', description: 'Gamma is highest for at-the-money options near expiration' },
                { title: 'Gamma scalping', description: 'Traders use high gamma to profit from stock movements by adjusting positions' },
                { title: 'Risk management', description: 'High gamma positions require more active management' }
              ]}
              iconColor="text-primary"
              variant="info"
            />
          </InfoBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Theta (Θ) - Time Decay</h2>
          <p class="text-gray-700 mb-4">
            Theta measures how much an option's price will decrease as time passes (time decay). 
            Options lose value every day, even if the stock price doesn't move.
          </p>
          <FeatureCardGrid
            columns={2}
            items={[
              {
                icon: 'fa-solid fa-user',
                iconColor: 'text-red-600',
                title: 'For Option Buyers',
                description: 'Theta works against you. Every day, your option loses value. This is why buying options requires the stock to move in your favor quickly.'
              },
              {
                icon: 'fa-solid fa-hand-holding-usd',
                iconColor: 'text-green-600',
                title: 'For Option Sellers',
                description: 'Theta works for you. You collect premium as time passes. This is why selling options (like Iron Condors) can be profitable even if the stock doesn\'t move.'
              }
            ]}
          />
          <ExampleBox title="Accelerating Decay" variant="yellow">
            Theta accelerates as expiration approaches. An option with 30 days to expiration might lose 
            $0.05 per day, but with 5 days left, it might lose $0.20 per day. This is why 0-DTE 
            (zero days to expiration) options decay so rapidly.
          </ExampleBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Vega (ν) - Volatility Sensitivity</h2>
          <p class="text-gray-700 mb-4">
            Vega measures how much an option's price will change for every 1% change in implied 
            volatility. Higher volatility = higher option prices (all else equal).
          </p>
          <InfoBox>
            <FeatureList
              items={[
                { title: 'Long options', description: 'Positive vega - benefit from volatility increases' },
                { title: 'Short options', description: 'Negative vega - benefit from volatility decreases' },
                { title: 'Highest at-the-money', description: 'Vega is highest for at-the-money options with more time to expiration' },
                { title: 'Volatility crush', description: 'After earnings or news, volatility often drops, hurting long option positions' }
              ]}
              iconColor="text-primary"
              variant="info"
            />
          </InfoBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">How PriceWhisperer Uses Greeks</h2>
          <p class="text-gray-700 mb-4">
            PriceWhisperer calculates and displays Greeks for every options strategy recommendation:
          </p>
          <InfoBox>
            <FeatureList
              items={[
                { title: 'Strategy Selection', description: 'We analyze Greeks to suggest strategies that match your risk profile and market outlook' },
                { title: 'Risk Assessment', description: 'Greeks help determine how sensitive your position is to price, time, and volatility changes' },
                { title: 'Position Management', description: 'Monitor how Greeks change as the stock moves to know when to adjust or exit' },
                { title: 'Delta-Neutral Strategies', description: 'We identify opportunities for delta-neutral trades that profit from time decay or volatility' }
              ]}
            />
          </InfoBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Practical Examples</h2>
          <SectionCardStack
            items={[
              {
                title: 'Example 1: Buying Calls',
                description: 'You buy a call with 0.60 delta, 0.05 gamma, -0.10 theta, 0.20 vega: Stock up $1: Option gains $0.60 (delta effect). Stock up another $1: Option gains $0.65 (delta now 0.65 due to gamma). One day passes: Option loses $0.10 (theta decay). Volatility up 5%: Option gains $1.00 (vega effect).'
              },
              {
                title: 'Example 2: Iron Condor',
                description: 'An Iron Condor typically has: Low delta (relatively insensitive to small stock moves), Negative theta (profits from time decay), Negative vega (profits from volatility decreases), High gamma risk (needs active management if stock moves toward strikes).'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Greeks and Risk Management</h2>
          <p class="text-gray-700 mb-4">
            Understanding Greeks helps you manage risk better:
          </p>
          <InfoBox>
            <FeatureList
              items={[
                { title: 'Delta exposure', description: 'High delta positions are more sensitive to stock moves - size accordingly' },
                { title: 'Gamma risk', description: 'High gamma means positions can become profitable or unprofitable quickly' },
                { title: 'Theta decay', description: 'Don\'t hold long options too close to expiration unless you expect a big move' },
                { title: 'Vega exposure', description: 'Be aware of volatility events (earnings, news) that can crush option values' }
              ]}
              iconColor="text-indigo-600"
              variant="shield"
            />
          </InfoBox>

          <KeyTakeawaysBox
            items={[
              'Delta measures price sensitivity - how much option moves with stock',
              'Gamma measures delta sensitivity - how quickly delta changes',
              'Theta measures time decay - options lose value over time',
              'Vega measures volatility sensitivity - how option price responds to volatility changes',
              'PriceWhisperer calculates and displays all Greeks for every options strategy',
              'Understanding Greeks helps you choose the right strategies and manage risk'
            ]}
          />

    </BlogPostLayout>
  );
};

export default OptionsGreeksExplained;
