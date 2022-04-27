const colors = require('tailwindcss/colors');
module.exports = {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    colors: {
      gray: colors.zinc,
      teal: colors.teal,
      blue: colors.slate,
      green: colors.emerald,
      purple: colors.violet,
      dark: colors.zinc[900],
      light: colors.zinc[300],
    },
    extend: {},
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
}
