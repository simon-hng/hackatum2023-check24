import { type Config } from "tailwindcss";
import { fontFamily } from "tailwindcss/defaultTheme";

export default {
  content: ["./src/**/*.tsx"],
  theme: {
    extend: {
      fontFamily: {
        sans: ["var(--font-sans)", ...fontFamily.sans],
      },
    },
  },
  daisyui: {
    themes: [
      {
        check24: {
          primary: "#063773",
          secondary: "#084085",
          accent: "#ffbb1c",
          neutral: "#0271c2",
          "base-100": "#ffffff",
          info: "#575757",
          success: "#00ffff",
          warning: "#ffffff",
          error: "#ffffff",
        },
      },
    ],
  },
  plugins: [require("daisyui")],
} satisfies Config;
