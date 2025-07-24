import path from "path";
import HtmlWebpackPlugin from "html-webpack-plugin";

const __dirname = path.resolve(import.meta.dirname);

export default {
  entry: "./index.js",
  mode: "development",
  context: __dirname,
  target: "web",
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
