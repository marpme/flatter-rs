/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./index.html",
        './src/**/*.rs',
    ],
    theme: {},
    plugins: [require("daisyui")]
}