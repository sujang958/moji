/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.svelte", "./index.html"],
  theme: {
    extend: {
      fontFamily: {
        pretendard: [
          "Toss Face",
          "Pretendard Variable",
          "Helvetica",
          "Noto Sans KR",
          "sans-serif",
        ],
      },
    },
  },
  plugins: [],
}
