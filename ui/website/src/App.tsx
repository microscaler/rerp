import { Component } from 'solid-js';
import Header from './components/Header_solidJS';
import Hero from './components/Hero_solidJS';
// Original versions
// import About from './components/About_solidJS';
// import WhyRERP from './components/WhyRERP_solidJS';
// import Architecture from './components/Architecture_solidJS';
// Image versions for testing
import About from './components/AboutWithImage_solidJS';
import WhyRERP from './components/WhyRERPWithImage_solidJS';
import Architecture from './components/ArchitectureWithImage_solidJS';
import Suites from './components/Suites_solidJS';
import Footer from './components/Footer_solidJS';

const App: Component = () => {
  return (
    <div class="bg-gray-900">
      <Header />
      <main>
        <Hero />
        <About />
        <WhyRERP />
        <Suites />
        <Architecture />
      </main>
      <Footer />
    </div>
  );
};

export default App;
