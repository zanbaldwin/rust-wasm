const path = require('path');
const webpack = require('webpack');
const CleanWebpackPlugin = require('clean-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    mode: 'development',
    entry: './src/index.js',
    output: {
        path: path.resolve(__dirname, 'build/dist'),
        filename: '[name].[chunkhash:8].js?[contenthash]',
        chunkFilename: '[chunkhash].chunk.js',
    },
    module: {
        rules: [
            {
                test: /\.wasm$/,
                type: "webassembly/experimental"
            },
        //     {
        //         test: /\.tsx?$/,
        //         use: 'ts-loader',
        //         exclude: /node_modules/,
        //     }
        ],
    },
    plugins: [
        new CleanWebpackPlugin,
        new HtmlWebpackPlugin,
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "."),
            extraArgs: Object.entries({
                'out-dir': 'build/pkg',
                'out-name': 'wasm',
            }).reduce((argString, optionMap) => argString + ` --${optionMap[0]} ${optionMap[1]}`, ''),
        }),
        // Microsoft Edge doesn't ship with TextEncoder/TextDecoder, uncomment the following to provide Edge support:
        //new webpack.ProvidePlugin({
        //  TextDecoder: ['text-encoding', 'TextDecoder'],
        //  TextEncoder: ['text-encoding', 'TextEncoder']
        //}),
    ],
};
