const path = require('path');
const TerserPlugin = require('terser-webpack-plugin');

module.exports = {
    entry: './maes/assets/main.js',
    output: {
        filename: 'main.bundle.js',
        path: path.resolve(__dirname, './maes/assets'),
    },
    mode: 'production',
    resolve: {
        extensions: ['.js', '.ts'],
        mainFields: ['module', 'browser', 'main'],
    },
    optimization: {
        minimize: true,
        minimizer: [
            new TerserPlugin({
                terserOptions: {
                    compress: {
                        drop_console: true,
                        drop_debugger: true,
                        passes: 2,
                    },
                    output: {
                        comments: false,
                    },
                },
                extractComments: false,
            }),
        ],
    },
    module: {
        rules: [
            {
                test: /\.css$/,
                use: ['style-loader', 'css-loader'],
            },
        ],
    },
};