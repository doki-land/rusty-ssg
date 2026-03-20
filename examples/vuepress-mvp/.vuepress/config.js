module.exports = {
    title: "VuePress MVP Example",
    description: "A minimal VuePress site for testing",
    themeConfig: {
        nav: [
            { text: "Home", link: "/" },
            { text: "Guide", link: "/guide/" },
            { text: "About", link: "/about/" },
        ],
        sidebar: {
            "/guide/": [{ title: "Guide", collapsable: false, children: ["", "getting-started"] }],
        },
    },
};
