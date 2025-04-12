import Alpine from 'alpinejs'
import './newsletter.js'

import hljs from 'highlight.js'
import 'highlight.js/styles/base16/material.css'
import '../css/main.css'

// AlpineJS
Alpine.start()

// HighlightingJS
hljs.highlightAll()

// Terminal effect script
document.addEventListener('DOMContentLoaded', function() {
    // Typing effect for the terminal prompt
    const terminalPrompt = document.querySelector('.terminal-prompt span');
    if (terminalPrompt) {
      const originalText = terminalPrompt.textContent;
      terminalPrompt.textContent = '';
      
      let i = 0;
      const typeEffect = setInterval(() => {
        if (i < originalText.length) {
          terminalPrompt.textContent += originalText.charAt(i);
          i++;
        } else {
          clearInterval(typeEffect);
          
          // After typing effect, show the navigation links with a delay
          setTimeout(() => {
            const navLinks = document.querySelector('.terminal-nav-links');
            if (navLinks) {
              navLinks.style.opacity = '1';
            }
            
            // Then show the content with a delay
            setTimeout(() => {
              const terminalPage = document.querySelector('.terminal-page');
              if (terminalPage) {
                terminalPage.style.opacity = '1';
              }
            }, 300);
          }, 300);
        }
      }, 50);
    }
    
    // Simulate terminal commands
    const cursor = document.querySelector('.cursor');
    if (cursor) {
      // Position cursor at the end of the content
      const terminalContent = document.querySelector('.terminal-content');
      if (terminalContent) {
        terminalContent.appendChild(cursor);
      }
    }
    
    // Terminal button interactions
    const closeButton = document.querySelector('.terminal-button.close');
    if (closeButton) {
      closeButton.addEventListener('click', function() {
        const terminalWindow = document.querySelector('.terminal-window');
        if (terminalWindow) {
          terminalWindow.style.opacity = '0';
          setTimeout(() => {
            terminalWindow.style.display = 'none';
            
            // Show a restart button
            const restartBtn = document.createElement('button');
            restartBtn.textContent = 'Restart Terminal';
            restartBtn.className = 'terminal-restart-btn';
            restartBtn.addEventListener('click', function() {
              terminalWindow.style.display = 'block';
              setTimeout(() => {
                terminalWindow.style.opacity = '1';
                this.remove();
              }, 10);
            });
            document.body.appendChild(restartBtn);
          }, 300);
        }
      });
    }
    
    // Minimize button
    const minimizeButton = document.querySelector('.terminal-button.minimize');
    if (minimizeButton) {
      minimizeButton.addEventListener('click', function() {
        const terminalContent = document.querySelector('.terminal-content');
        if (terminalContent) {
          if (terminalContent.style.display === 'none') {
            terminalContent.style.display = 'block';
          } else {
            terminalContent.style.display = 'none';
          }
        }
      });
    }
    
    // Maximize button
    const maximizeButton = document.querySelector('.terminal-button.maximize');
    if (maximizeButton) {
      maximizeButton.addEventListener('click', function() {
        const terminalWindow = document.querySelector('.terminal-window');
        if (terminalWindow) {
          terminalWindow.classList.toggle('terminal-fullscreen');
        }
      });
    }
  });
  