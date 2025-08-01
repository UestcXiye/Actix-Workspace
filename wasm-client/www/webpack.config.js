const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: 'webassembly/experimental'
      }
    ]
  },
  plugins: [
    new CopyWebpackPlugin(['index.html'])
  ],
};