const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  resolve: {
    extensions: ['.js', '.wasm'],
  },
  plugins: [new HtmlWebpackPlugin()],
}
