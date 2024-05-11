/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      minHeight: {
        navbar: "var(--navbar-size)",
        "screen-minus-navbar": "var(--screen-minus-navbar)",
      },
      maxHeight: {
        navbar: "var(--navbar-size)",
        "screen-minus-navbar": "var(--screen-minus-navbar)",
      },
      height: {
        navbar: "var(--navbar-size)",
        "screen-minus-navbar": "var(--screen-minus-navbar)",
      },
      fontFamily: {
        regular: "var(--regular-font)",
      },
    },
  },
  plugins: [],
};
