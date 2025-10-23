import fs from 'fs';
import { appTools, defineConfig } from '@modern-js/app-tools';
import { tailwindcssPlugin } from '@modern-js/plugin-tailwindcss';

// https://modernjs.dev/en/configure/app/usage
export default defineConfig({
  runtime: {
    router: {
      basename: './',
    },
  },
  source: {
    mainEntryName: 'index',
  },
  output: {
    distPath: {
      html: '',
      root: '../../dist',
    },
    assetPrefix: './', // 相对路径打包
  },
  html: {
    disableHtmlFolder: true,
  },
  plugins: [appTools(), tailwindcssPlugin()],
  tools: {
    webpack: config => {
      // 禁用代码分割（关键配置）
      if (config.optimization) {
        config.optimization.splitChunks = {
          chunks: 'all', // 不分割任何 chunk（包括同步和异步）
          cacheGroups: {}, // 清空缓存组（避免默认规则分割公共模块）
        };
        // 禁用 runtime chunk（可选，若存在单独的 runtime chunk）
        config.optimization.runtimeChunk = false;
      }

      return config;
    },
  },
});
