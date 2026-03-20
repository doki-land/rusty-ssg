module.exports = function(eleventyConfig) {
  return {
    dir: {
      input: "src",
      output: "dist",
      includes: "_includes",
      data: "_data"
    },
    plugins: [
      {
        name: "@11ty/eleventy-plugin-syntaxhighlight",
        options: {}
      }
    ]
  };
};