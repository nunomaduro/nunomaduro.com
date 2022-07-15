/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './presentation/templates/**/*.html',
  ],
  theme: {
    extend: {},
  },
  plugins: [
    "@tailwindcss/line-clamp",
    "@tailwindcss/typography",
    "cssnano",
  ],
}
