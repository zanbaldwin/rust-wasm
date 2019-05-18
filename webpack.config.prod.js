const merge = require('lodash.merge');
const config = require('./webpack.config');

module.exports = merge(config, {
    mode: 'production',
    optimization: {
        nodeEnv: 'production',
        runtimeChunk: 'single',
    }
});
