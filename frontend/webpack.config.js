const path = require('path');
const { readdirSync, mkdirSync } = require("fs");
const TerserPlugin = require('terser-webpack-plugin');
const targetFolder = "./static/js";

// Check if the target folder exists, create it if not
if (!readdirSync("./static").includes("js")) {
    mkdirSync(targetFolder);
}

module.exports = {
    entry: './src/app/js/script.ts',
    output: {
        filename: 'script.js',
        path: path.resolve(__dirname, 'static/js'),
    },
    module: {
        rules: [
            {
                test: /\.(js|ts)$/,
                exclude: /node_modules/,
                use: {
                    loader: 'babel-loader',
                },
            },
        ],
    },
    resolve: {
        extensions: ['.ts', '.js'],
    },
    mode: 'production',
    optimization: {
        minimize: true,
        minimizer: [
            new TerserPlugin({
                terserOptions: {
                    compress: {},
                    mangle: true,
                    format: {
                        comments: false,
                    },
                },
                extractComments: false,
            }),
        ],
    },
};
