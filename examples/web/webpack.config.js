const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
  entry: "./index.js",
  devtool: "source-map",
  resolve: {
    extensions: [".js"],
  },
  output: {
    filename: "bundle.js",
    path: path.resolve(__dirname, "build"),
    clean: true,
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: "index.html",
    }),
  ],
  devServer: {
    static: path.join(__dirname, "build"),
    compress: true,
    port: 3000,
    open: true,
  },
  module: {
    // rules: [{ test: /\.wasm$/, type: "webassembly/async" }],
  },
  experiments: {
    // asyncWebAssembly: true,
    // topLevelAwait: true,
  },
};
