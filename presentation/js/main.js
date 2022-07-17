import { getHighlighter, setCDN } from 'shiki';

setCDN("/node_modules/shiki/")

getHighlighter({
    theme: 'nord'
})
.then(highlighter => {
    console.log(highlighter.codeToHtml(`console.log('shiki');`, { lang: 'js' }))
})
