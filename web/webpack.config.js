// @ts-check

import MiniCssExtractPlugin from 'mini-css-extract-plugin';
import HtmlWebpackPlugin from 'html-webpack-plugin';
import path, { dirname } from 'path';
import { fileURLToPath } from 'url';
import webpack from 'webpack';
import { BundleAnalyzerPlugin } from 'webpack-bundle-analyzer';
import fs from 'fs';
import { modules, plugins, resolve } from 'fuzionkit-build/webpack/base.js';
import { CycloneDxWebpackPlugin } from '@cyclonedx/webpack-plugin';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const packageJson = JSON.parse(fs.readFileSync('./package.json').toString());

/**
 * @typedef {import('fuzionkit-build/webpack/base.js').BuildProfile} BuildProfile
 */

/**
 * @param {{ [key: string]: any }=} env
 * @param {{ [key: string]: any }=} options
 * @return {BuildProfile}
 */
function buildProfile(env = {}, options) {
  const { profile = 'debug' } = env;
  /**
   * @type {BuildProfile}
   */
  return {
    name: profile,
    mode: profile === 'debug' ? 'development' : 'production',
    outputPath: './build' +
      (options?.output ? `/${options?.output}` : '') +
      `/${env.profile}`,
    basePath: path.join(__dirname, '.'),
    srcPath: path.join(__dirname, 'src'),
    nodeModulesPath: path.join(__dirname, 'node_modules'),
    tsconfigPath: path.join(__dirname, 'tsconfig.webpack.json'),
  };
}

/**
 * @param {{[key: string]: any}} env
 */
const webConfig = function (env) {
  const profile = buildProfile(env);

  const indexCss = 'app.css';

  /**
   * @type {import('webpack').Configuration}
   */
  const out = {
    name: 'web',
    mode: profile.mode,
    node: {
      global: true,
    },
    target: 'web',
    entry: [
      'core-js/stable',
      'regenerator-runtime/runtime',
      path.resolve(__dirname, 'src/js/apps/verita/index.ts'),
    ],
    output: {
      path: path.resolve(__dirname, profile.outputPath),
      filename: 'app.js',
      chunkFilename: 'app.[contenthash].js',
      assetModuleFilename: '[hash][ext][query]',
    },
    cache: {
      type: 'filesystem',
    },
    module: modules(profile),
    resolve: resolve(profile),
    plugins: plugins(profile, [
      new CycloneDxWebpackPlugin({
        // @ts-expect-error no export
        specVersion: '1.4',
        outputLocation: './bom',
      }),
      new MiniCssExtractPlugin({
        filename: indexCss,
        chunkFilename: profile.mode === 'development' ? '[id].css' : '[id].[hash].css',
      }),
      new HtmlWebpackPlugin({
        filename: 'index.html',
        template: 'src/html/index.hbs',
        inject: 'head',
        minify: profile.mode === 'development'
          ? false
          : {
            removeAttributeQuotes: true,
            collapseWhitespace: true,
            html5: true,
            minifyCSS: true,
            removeComments: true,
            removeEmptyAttributes: true,
          },
        hash: true,
        cacheBreaker: Math.round(Math.random() * 1024 * 8),
      }),
      new HtmlWebpackPlugin({
        filename: 'loader.html',
        template: 'src/html/loader.hbs',
        inject: false,
        minify: profile.mode === 'development'
          ? false
          : {
            removeAttributeQuotes: true,
            collapseWhitespace: true,
            html5: true,
            minifyCSS: true,
            removeComments: true,
            removeEmptyAttributes: true,
          },
        hash: true,
      }),
      new webpack.DefinePlugin({
        'process.env.PUBLIC_PATH': JSON.stringify(profile.outputPath),
        'process.env.VERSION': JSON.stringify(packageJson.version),
      }),
    ]),
    optimization: {},
    stats: {
      colors: true,
      children: true,
    },
    devtool: profile.mode === 'development' ? 'inline-source-map' : false,
    experiments: {
      backCompat: false,
    },
    devServer: {
      historyApiFallback: true,
      host: '0.0.0.0',
      port: 7182,
      allowedHosts: 'all',
      client: {
        overlay: {
          warnings: false,
          errors: false,
        },
      },
      webSocketServer: {
        type: 'ws',
        options: {
          path: '/wds',
        },
      },
      static: [
        {
          directory: path.resolve(__dirname, 'public'),
          publicPath: '/',
          watch: true,
        },
      ],
      headers: {
        'Access-Control-Allow-Origin': '*',
        'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, PATCH, OPTIONS',
        'Access-Control-Allow-Headers': 'X-Requested-With, content-type, Authorization',
      },
      proxy: [
        {
          context: ['/api'],
          target: 'http://127.0.0.1:10666',
        },
      ],
    },
  };

  if (env['bundle-analyzer'] === 'true') {
    out.plugins?.push(
      new BundleAnalyzerPlugin({
        analyzerPort: 7184,
        openAnalyzer: false,
      }),
    );
  }

  return out;
};

export default [
  webConfig,
];
