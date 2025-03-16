import node_path from 'node:path'
import type { PluginOption } from 'vite'
import { defineConfig, loadEnv } from 'vite'
import vue from '@vitejs/plugin-vue'
import {nodePolyfills} from 'vite-plugin-node-polyfills';

//import cssnanoPlugin from 'cssnano';



function setupPlugins(env: ImportMetaEnv): PluginOption[] {
  return [
    vue(),
    nodePolyfills({
      include: ['path','url'],
    })
  ]
}

export default defineConfig((env) => {
  const viteEnv = loadEnv(env.mode, process.cwd()) as unknown as ImportMetaEnv

  return {
    resolve: {
      alias: {
        '@': node_path.resolve(process.cwd(), 'src'),
        //path: "path-browserify",

      },
    },

    optimizeDeps: {
      exclude: ['fs', ],
      include: []
  },
    
    /*
    esbuild: {
      charset: 'ascii'
    },
    */
    plugins: setupPlugins(viteEnv),

    server: {
      host: '0.0.0.0',
      port: 1002,
      open: false,
      proxy: {
        '/proxy': {
          target: viteEnv.VITE_CONFIG_API_BASE_URL,
          changeOrigin: true, // 允许跨域
          rewrite: path => path.replace('/proxy/', '/'),
        },

      },
    },

    css: {
      /*
      postcss: {
        plugins: [
          cssnanoPlugin()
        ]
      }
      */
      /*
          loaderOptions: {
            scss: { 
              sassOptions: { outputStyle: 'expanded' } 
            }
          }
       */
    },

    build: {
      reportCompressedSize: false,
      sourcemap: false,
      commonjsOptions: {
        ignoreTryCatch: false,
      },
      cssMinify: false
    },
  }
})
