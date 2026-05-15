import type { Config } from "tailwindcss";

const config: Config = {
  content: ["./app/**/*.{ts,tsx}", "./components/**/*.{ts,tsx}", "./lib/**/*.{ts,tsx}"],
  theme: {
    extend: {
      // Named colors keep Tailwind classes readable across the operational UI.
      colors: {
        ink: "#111827",
        surface: "#f8fafc",
        panel: "#ffffff",
        line: "#d6dde8",
        accent: "#0f766e",
        warning: "#b45309",
        danger: "#b91c1c"
      },
      // Shared shadow token gives panels depth without repeating raw CSS.
      boxShadow: {
        enterprise: "0 1px 2px rgba(16, 24, 40, 0.06), 0 1px 3px rgba(16, 24, 40, 0.08)"
      }
    }
  },
  plugins: []
};

export default config;
