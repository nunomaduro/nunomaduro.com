/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './presentation/templates/*.html',
    './presentation/templates/**/*.html',
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/typography'),
    require('@tailwindcss/line-clamp'),
    require('cssnano'),
  ],
}
