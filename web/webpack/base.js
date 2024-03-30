import MiniCssExtractPlugin from 'mini-css-extract-plugin';
import webpack from 'webpack';
import path, { dirname } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const modules = (
  profile,
  {
    tsconfigPath,
  } = {
    tsconfigPath: path.resolve(__dirname, '../tsconfig.webpack.json'),
  },
) => ({
  rules: [
    {
      test: /\.js$/,
      enforce: 'pre',
      use: [
        {
          loader: 'source-map-loader',
          options: {
            /**
             * @param {string} _url
             * @param {string} resourcePath
             */
            filterSourceMappingUrl: (_url, resourcePath) => {
              if (/tsee\//i.test(resourcePath)) {
                return false;
              }

              return true;
            },
          },
        },
      ],
    },
    {
      test: /\.(sa|sc|c)ss$/,
      oneOf: [
        {
          resourceQuery: /lit/,
          exclude: path.resolve(__dirname, '../node_modules/'),
          use: [
            { loader: 'lit-scss-loader', options: { minify: false } },
            { loader: path.resolve(__dirname, './escape-lit-scss.js') },
            {
              loader: 'extract-loader',
              options: {
                publicPath: '',
                sourceMap: true,
              },
            },
            {
              loader: 'css-loader',
              options: { sourceMap: true, esModule: false },
            },
            {
              loader: 'sass-loader',
              options: {
                sassOptions: {
                  includePaths: [ '.', './src', '../node_modules' ],
                },
                sourceMap: true,
              },
            },
          ],
        },
        {
          use: [
            { loader: MiniCssExtractPlugin.loader },
            { loader: 'css-loader', options: { sourceMap: true } },
            { loader: 'resolve-url-loader', options: { sourceMap: true } },
            {
              loader: 'sass-loader',
              options: {
                sassOptions: {
                  includePaths: [ '.', './src', '../node_modules' ],
                },
                sourceMap: true,
              },
            },
          ],
        },
      ],
    },
    {
      test: /\.(png|jpg|gif|woff|woff2)/,
      exclude: path.resolve(__dirname, '../node_modules/'),
      dependency: { not: [ 'url' ] },
      use: [
        {
          loader: 'url-loader',
          options: {
            limit: 10000,
            alias: {},
            esModule: false,
          },
        },
      ],
    },
    {
      test: /\.(ttf|eot|svg|otf)/,
      exclude: path.resolve(__dirname, '../node_modules/'),
      dependency: { not: [ 'url' ] },
      use: [
        {
          loader: 'file-loader',
          options: {
            esModule: false,
          },
        },
      ],
    },
    {
      test: /\.tsx?$/,
      exclude: path.resolve(__dirname, '../node_modules/'),
      oneOf: [
        {
          test: /\.tsx?$/,
          use: [
            {
              loader: 'babel-loader',
              options: {
                presets: [
                  [ '@babel/preset-env', {
                    corejs: '3.36.0',
                    useBuiltIns: 'entry',
                  } ],
                ],
              },
            },
            {
              loader: 'ts-loader',
              options: {
                configFile: tsconfigPath,
              },
            },
          ],
        },
      ],
    },
    {
      test: /\.m?jsx?$/,
      exclude: path.resolve(__dirname, '../node_modules/'),
      oneOf: [
        {
          use: [
            {
              loader: 'babel-loader',
              options: {
                presets: [
                  [ '@babel/preset-env', {
                    corejs: '3.36.0',
                    useBuiltIns: 'entry',
                  } ],
                ],
                plugins: [
                  '@babel/plugin-transform-class-properties',
                ],
              },
            },
          ],
        },
      ],
    },
    {
      test: /\.hbs$/,
      exclude: path.resolve(__dirname, '../node_modules/'),
      use: [
        {
          loader: 'handlebars-loader',
          options: {
            minimize: profile.mode === 'development',
            extensions: [ 'handlebars', 'hbs', '' ],
            helperDirs: [
              path.resolve(__dirname, '../src/html/helpers/'),
            ],
          },
        },
      ],
    },
    {
      test: /\.mp3$/,
      exclude: path.resolve(__dirname, '../node_modules/'),
      loader: 'file-loader',
    },
    {
      test: /\.pug$/,
      exclude: path.resolve(__dirname, '../node_modules/'),
      include: path.join(__dirname, '../src'),
      loader: 'pug-loader',
    },
    {
      resourceQuery: /inline/,
      exclude: path.resolve(__dirname, '../node_modules/'),
      type: 'asset/inline',
    },
  ],
});

const plugins = (
  profile,
  plugins,
) => ([
  new webpack.LoaderOptionsPlugin({
    minimize: profile.mode === 'production',
  }),
  ...plugins,
]);

const resolve = ({
  modules = [],
} = {}) => ({
  symlinks: false,
  modules: [
    ...modules,
    path.resolve(__dirname, '../src'),
    'node_modules',
  ],
  extensions: [ '.js', '.jsx', '.ts', '.tsx', '.mjs' ],
});

export {
  modules,
  plugins,
  resolve,
};
