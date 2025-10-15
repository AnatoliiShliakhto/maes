const path = require('path');

module.exports = {
    entry: './maes/assets/main.js',
    output: {
        filename: 'main.bundle.js',
        path: path.resolve(__dirname, './maes/assets'),
    },
    mode: 'production',
    resolve: {
        extensions: ['.js', '.ts'],
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