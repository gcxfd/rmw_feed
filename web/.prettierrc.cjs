// https://prettier.github.io/plugin-pug/guide/pug-specific-options.html
module.exports = {
  plugins: [require.resolve("@prettier/plugin-pug")],
  pugAttributeSeparator: "none",
  pugSortAttributes: "asc",
  pugEmptyAttributes: "none",
};
