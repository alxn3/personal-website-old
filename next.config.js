const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const path = require('path');

module.exports = {
  webpack(config) {
    config.output.webassemblyModuleFilename = 'static/wasm/[modulehash].wasm';
    config.plugins.push(
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, 'wasm'),
      })
    );

    return config;
  },
};
