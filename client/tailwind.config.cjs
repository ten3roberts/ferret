const colors = require('tailwindcss/colors');
module.exports = {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    colors: {
      gray: colors.zinc,
      blue: colors.slate,
      green: colors.emerald,
      purple: colors.violet,
      dark: colors.zinc[900],
      light: colors.zinc[300],
    },
    extend: {},
  },
  plugins: [],
}
