import { render } from 'solid-js/web';
import App from './App';
import '@fortawesome/fontawesome-free/css/all.min.css';
import '@fortawesome/fontawesome-free/js/all.min.js';
import './index.css';

const root = document.getElementById('root');

if (root) {
  render(() => <App />, root);
}
