/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        // 背景层次（由深到浅）
        bg: "#0c0d11",
        panel: "#14161c",
        panel2: "#191c23",
        panel3: "#21252e",
        panel4: "#2a2f3a",
        border: "#272b34",
        border2: "#343a46",
        // 文本层次
        txt: "#e9ebf0",
        txt2: "#a7adb9",
        txt3: "#6c727f",
        // 强调色（indigo/violet）
        accent: "#6366f1",
        accent2: "#818cf8",
        accent3: "#a5b4fc",
      },
      fontFamily: {
        sans: [
          "Inter",
          "ui-sans-serif",
          "system-ui",
          "-apple-system",
          "Segoe UI",
          "Roboto",
          "PingFang SC",
          "Microsoft YaHei",
          "sans-serif",
        ],
        mono: [
          "JetBrains Mono",
          "ui-monospace",
          "SFMono-Regular",
          "Menlo",
          "Consolas",
          "monospace",
        ],
      },
      boxShadow: {
        soft: "0 1px 2px rgba(0,0,0,.4), 0 2px 8px rgba(0,0,0,.25)",
        pop: "0 8px 30px rgba(0,0,0,.45)",
        glow: "0 0 0 1px rgba(99,102,241,.4), 0 6px 22px rgba(99,102,241,.25)",
      },
      keyframes: {
        shimmer: {
          "100%": { transform: "translateX(100%)" },
        },
        "slide-in": {
          "0%": { transform: "translateX(16px)", opacity: "0" },
          "100%": { transform: "translateX(0)", opacity: "1" },
        },
        "fade-in": {
          "0%": { opacity: "0" },
          "100%": { opacity: "1" },
        },
        "toast-in": {
          "0%": { transform: "translate(-50%, 12px)", opacity: "0" },
          "100%": { transform: "translate(-50%, 0)", opacity: "1" },
        },
      },
      animation: {
        shimmer: "shimmer 1.4s infinite",
        "slide-in": "slide-in .22s cubic-bezier(.21,1.02,.73,1)",
        "fade-in": "fade-in .18s ease",
        "toast-in": "toast-in .25s cubic-bezier(.21,1.02,.73,1)",
      },
    },
  },
  plugins: [],
};
