import { Component } from 'solid-js';

interface CommunityProps {
  standalone?: boolean;
}

const Community: Component<CommunityProps> = (props) => {
  const contributionAreas = [
    {
      icon: 'fa-code',
      title: 'Code Contributions',
      description: 'Contribute to core services, add new modules, fix bugs, or improve existing functionality',
      link: 'https://github.com/microscaler/rerp',
    },
    {
      icon: 'fa-book',
      title: 'Documentation',
      description: 'Help improve documentation, write guides, create tutorials, or translate content',
      link: 'https://github.com/microscaler/rerp',
    },
    {
      icon: 'fa-bug',
      title: 'Bug Reports',
      description: 'Report issues, suggest improvements, or help test new features',
      link: 'https://github.com/microscaler/rerp/issues',
    },
    {
      icon: 'fa-comments',
      title: 'Community Support',
      description: 'Help other users, answer questions, share experiences, and build the community',
      link: 'https://github.com/microscaler/rerp/discussions',
    },
  ];

  const values = [
    {
      title: 'Open Source',
      description: '100% open-source with full transparency. Code is available for review, audit, and improvement.',
    },
    {
      title: 'Collaboration',
      description: 'Built by the community, for the community. Everyone's contributions make RERP better.',
    },
    {
      title: 'Transparency',
      description: 'Public roadmap, open discussions, and transparent decision-making processes.',
    },
    {
      title: 'Innovation',
      description: 'Freedom to experiment, innovate, and extend RERP to meet diverse business needs.',
    },
  ];

  return (
    <section id="community" class="py-20 bg-white">
      <div class="max-w-7xl mx-auto px-6 lg:px-8">
        {!props.standalone && (
          <div class="text-center mb-16">
            <h2 class="text-4xl font-bold text-gray-900 mb-4">Join the RERP Community</h2>
            <p class="text-xl text-gray-600 max-w-3xl mx-auto">
              RERP is built by and for the community. Open-source values, collaborative development, and shared innovation drive continuous improvement.
            </p>
          </div>
        )}

        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-16">
          {contributionAreas.map((area) => (
            <a
              href={area.link}
              target="_blank"
              rel="noopener noreferrer"
              class="bg-gray-50 rounded-xl p-6 hover:shadow-lg transition-shadow border border-gray-200"
            >
              <div class="w-12 h-12 bg-primary rounded-lg flex items-center justify-center mb-4">
                <i class={`fa-solid ${area.icon} text-white text-xl`}></i>
              </div>
              <h3 class="text-lg font-semibold text-gray-900 mb-2">{area.title}</h3>
              <p class="text-gray-600 text-sm">{area.description}</p>
            </a>
          ))}
        </div>

        <div class="bg-gradient-to-br from-blue-50 to-indigo-100 rounded-xl p-8 mb-16">
          <h3 class="text-2xl font-bold text-gray-900 mb-6 text-center">Our Values</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            {values.map((value) => (
              <div class="bg-white rounded-lg p-6">
                <h4 class="text-lg font-semibold text-gray-900 mb-2">{value.title}</h4>
                <p class="text-gray-600 text-sm">{value.description}</p>
              </div>
            ))}
          </div>
        </div>

        <div class="bg-primary rounded-2xl p-8 text-white text-center">
          <h3 class="text-2xl font-bold mb-4">Get Involved Today</h3>
          <p class="text-blue-100 mb-6 max-w-2xl mx-auto">
            Whether you're a developer, business user, or just curious about open-source ERP, there's a place for you in the RERP community.
          </p>
          <div class="flex flex-wrap justify-center gap-4">
            <a
              href="https://github.com/microscaler/rerp"
              target="_blank"
              rel="noopener noreferrer"
              class="bg-white text-primary px-6 py-3 rounded-lg hover:bg-gray-100 font-semibold transition-colors inline-flex items-center"
            >
              <i class="fa-brands fa-github mr-2"></i>
              View Repository
            </a>
            <a
              href="https://github.com/microscaler/rerp/discussions"
              target="_blank"
              rel="noopener noreferrer"
              class="bg-blue-700 text-white px-6 py-3 rounded-lg hover:bg-blue-800 font-semibold transition-colors inline-flex items-center border-2 border-white"
            >
              Join Discussions
              <i class="fa-solid fa-arrow-right ml-2"></i>
            </a>
          </div>
        </div>
      </div>
    </section>
  );
};

export default Community;
