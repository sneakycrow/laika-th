/** @type {import('tailwindcss').Config} */
export default {
    darkMode: "selector",
    content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
    theme: {
        fontFamily: {
            sans: ["Outfit", "sans-serif"],
            mono: [
                "ui-monospace",
                "SFMono-Regular",
                "Menlo",
                "Monaco",
                "Consolas",
                "Liberation Mono",
                "Courier New",
                "monospace",
            ],
        },
        fontSize: {
            xs: "0.9rem",
            sm: "1rem",
            base: ["1.25rem", { lineHeight: "1.5rem" }],
            lg: ["1.5rem", { lineHeight: "1.8rem" }],
            xl: ["2rem", { lineHeight: "2.2rem" }],
            "2xl": ["3.2rem", { lineHeight: "3.4rem" }],
            huge: ["9.5rem", { lineHeight: "9.7rem" }],
        },
        extend: {
            colors: {
                black: "#1f1f1f",
                white: "#fefcd9",
            },
        },
    },
    plugins: [],
}
