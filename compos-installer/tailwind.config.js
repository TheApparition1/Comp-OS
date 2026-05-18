/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        'compos': {
          primary: '#667eea',
          secondary: '#764ba2',
          accent: '#48bb78',
          danger: '#f56565'
        }
      }
    }
  },
  plugins: []
}