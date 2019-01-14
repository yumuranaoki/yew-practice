const ExtractTextPlugin = require('extract-text-webpack-plugin');
const enabledSourceMap = false;

module.exports = {
  mode: 'development',
  entry: './prebundle/index.js',
  output: {
    //  出力ファイルのディレクトリ名
    path: `${__dirname}/dist`,
    // 出力ファイル名
    filename: 'main.js'
  },

  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: "webassembly/experimental"
			},
      // Sassファイルの読み込みとコンパイル
      {
        test: /\.scss/, // 対象となるファイルの拡張子
        use: ExtractTextPlugin.extract({
          use:
            [
            // CSSをバンドルするための機能
            {
              loader: 'css-loader',
              options: {
                // オプションでCSS内のurl()メソッドの取り込みを禁止する
                url: false,
                // ソースマップの利用有無
                sourceMap: enabledSourceMap,

                // 0 => no loaders (default);
                // 1 => postcss-loader;
                // 2 => postcss-loader, sass-loader
                importLoaders: 2
              },
            },
            {
              loader: 'sass-loader',
              options: {
                // ソースマップの利用有無
                sourceMap: enabledSourceMap,
              }
            }
          ]
        }),
      },
    ],
  },

  plugins: [
    new ExtractTextPlugin('style.css'),
  ],
};