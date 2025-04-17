import { Component, For, JSX } from 'solid-js';

export interface Stat {
  value: string;
  label: string;
}

export interface HeroWithStatsProps {
  title: string;
  description: string | JSX.Element;
  stats: Stat[];
  backgroundClass?: string;
  textColor?: string;
}

const HeroWithStats: Component<HeroWithStatsProps> = (props) => {
  const backgroundClass = props.backgroundClass || 'bg-gradient-to-br from-primary to-blue-600';
  const textColor = props.textColor || 'text-white';
  const statTextColor = props.textColor === 'text-white' ? 'text-blue-100' : 'text-gray-600';

  return (
    <section class={`${backgroundClass} ${textColor} py-16`}>
      <div class="max-w-7xl mx-auto px-6 lg:px-8">
        <div class="text-center">
          <h1 class="text-5xl font-extrabold mb-4">{props.title}</h1>
          <div class="text-xl mb-8 max-w-3xl mx-auto">
            {typeof props.description === 'string' ? (
              <p>{props.description}</p>
            ) : (
              props.description
            )}
          </div>
          <div class="flex items-center justify-center space-x-8">
            <For each={props.stats}>
              {(stat) => (
                <div class="text-center">
                  <div class={`text-3xl font-bold ${textColor} mb-1`}>{stat.value}</div>
                  <div class={`text-sm ${statTextColor}`}>{stat.label}</div>
                </div>
              )}
            </For>
          </div>
        </div>
      </div>
    </section>
  );
};

export default HeroWithStats;

