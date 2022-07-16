import { getHighlighter } from 'shiki';

getHighlighter({
    theme: 'nord'
})
.then(highlighter => {
    console.log(highlighter.codeToHtml(`console.log('shiki');`, { lang: 'js' }))
})
