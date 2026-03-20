import { defineConfig } from "vitepress";

export default defineConfig({
    title: "VitePress MVP Example",
    description: "A minimal VitePress site for testing",
    themeConfig: {
        nav: [
            { text: "Home", link: "/" },
            { text: "Guide", link: "/guide/" },
            { text: "About", link: "/about/" },
        ],
        sidebar: {
            "/guide/": [
                { text: "Getting Started", link: "/guide/getting-started" },
                { text: "Configuration", link: "/guide/configuration" },
            ],
        },
    },
});
