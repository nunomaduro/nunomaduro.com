import Alpine from 'alpinejs'

import hljs from 'highlight.js'
import 'highlight.js/styles/github-dark.css'
import '../css/main.css'

// AlpineJS
Alpine.start()

// HighlightingJS
hljs.highlightAll()

function openExternalLinksInNewTab() {
    document.querySelectorAll('a[href]').forEach(link => {
        if (link.protocol.startsWith('http') && link.host !== window.location.host) {
            link.setAttribute('target', '_blank');
            link.setAttribute('rel', 'noopener noreferrer');
        }
    });
}

function makeCodeBlocksScrollableByKeyboard() {
    document.querySelectorAll('.prose pre').forEach(block => {
        block.setAttribute('tabindex', '0');
    });
}

function copyCommandOnClick() {
    document.querySelectorAll('button[data-copy]').forEach(button => {
        const label = button.textContent;
        const status = button.parentElement.querySelector('[data-copy-status]');

        button.addEventListener('click', () => {
            navigator.clipboard.writeText(button.dataset.copy).then(() => {
                button.textContent = 'copied!';
                status.textContent = 'copied';

                setTimeout(() => {
                    button.textContent = label;
                    status.textContent = '';
                }, 2000);
            });
        });
    });
}

document.addEventListener('DOMContentLoaded', function() {
    openExternalLinksInNewTab();
    makeCodeBlocksScrollableByKeyboard();
    copyCommandOnClick();
});
