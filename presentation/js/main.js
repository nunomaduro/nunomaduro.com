import Alpine from 'alpinejs'
import './newsletter.js'

import hljs from 'highlight.js'
import 'highlight.js/styles/base16/material.css'
import '../css/main.css'

// AlpineJS
Alpine.start()

// HighlightingJS
hljs.highlightAll()

// Make all links in content pages open in a new tab
function makeAllLinksTargetBlank() {
  const contentArea = document.querySelector('main');
  if (contentArea) {
    const links = contentArea.querySelectorAll('a');

    links.forEach(link => {
      if (!link.getAttribute('href').startsWith('#')) {
        link.setAttribute('target', '_blank');
        link.setAttribute('rel', 'noopener noreferrer');
      }
    });
  }
}

document.addEventListener('DOMContentLoaded', function() {
    makeAllLinksTargetBlank();
});
