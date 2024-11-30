module.exports = {
  daisyui: {
    themes: [
      {
        maidono: {
          "primary": "#fbbf24",
          "secondary": "#bef264",
          "accent": "#c084fc",
          "neutral": "#374151",
          "base-100": "#1f2937",
          "info": "#60a5fa",
          "success": "#34d399",
          "warning": "#fb923c",
          "error": "#fb7185",
          "--rounded-btn": "9999px;",
        },
      },
    ],
  },
  plugins: [require('daisyui')],
  content: ['./src/**/*.{vue,js,ts}'],
}