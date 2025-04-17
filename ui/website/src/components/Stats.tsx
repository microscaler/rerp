import { Component } from 'solid-js';

const Stats: Component = () => {
  const stats = [
    { id: 'stat-services', value: '71', label: 'Independent Services' },
    { id: 'stat-phases', value: '6', label: 'Implementation Phases' },
    { id: 'stat-architecture', value: 'Cloud-Native', label: 'Architecture' },
    { id: 'stat-license', value: 'Open Source', label: 'License' },
  ];

  return (
    <section id="stats" class="py-20 bg-primary">
      <div class="max-w-7xl mx-auto px-6 lg:px-8">
        <div class="grid grid-cols-1 md:grid-cols-4 gap-8 text-center">
          {stats.map((stat) => (
            <div id={stat.id} class="text-white">
              <div class="text-4xl font-bold mb-2">{stat.value}</div>
              <div class="text-blue-100">{stat.label}</div>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
};

export default Stats;
