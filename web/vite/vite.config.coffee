import { join,dirname } from 'path'
import sveltePreprocess from '@rmw/svelte-preprocess'
import { merge } from 'lodash-es'
import vitePluginStylusAlias from './plugin/vite-plugin-stylus-alias.mjs'
import coffee from '@rmw/rollup-plugin-coffee'
import pug from 'pug'
import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import thisdir from '@rmw/thisdir'
import { writeFileSync,renameSync } from 'fs'
import import_pug from './plugin/pug.js'

ROOT = dirname thisdir(import.meta)
DIST = join ROOT,'dist'
SRC = join ROOT,'src'
FILE = join ROOT,'file'
PRODUCTION = process.env.NODE_ENV == 'production'

INDEX_HTML = 'index.html'
SRC_INDEX_HTML = join SRC,INDEX_HTML
writeFileSync(
  SRC_INDEX_HTML
  pug.compileFile(join SRC, 'index.pug')({
  })
)
host = '0.0.0.0' or env.VITE_HOST
port = 5001 or env.VITE_PORT

config = {
  plugins: [
    coffee(
      bare:true
      sourceMap:true
    )
    svelte(
      preprocess: [
        sveltePreprocess(
          coffeescript: {
            label:true
            sourceMap: true
          }
          stylus: true
          pug: true
        )
      ]
    )
    vitePluginStylusAlias()
    import_pug()
  ]
  clearScreen: false
  server:{
    host
    port

    ###
    proxy:
      '^/[^@.]+$':
        target: "http://#{host}:#{port}"
        rewrite: (path)=>'/'
        changeOrigin: false
    ###
  }
  resolve:
    alias:
      ":": join(ROOT, "file")
      '~': SRC
  esbuild:
    charset:'utf8'
    legalComments: 'none'
    treeShaking: true
  root: SRC
  build:
    outDir: DIST
    rollupOptions:
      input:
        index:SRC_INDEX_HTML
    target:['edge90','chrome90','firefox90','safari15']
    assetsDir: '/'
    emptyOutDir: true
}

config = merge config, await do =>
  if PRODUCTION
    FILENAME = '[name].[hash].[ext]'
    JSNAME = '[name].[hash].js'

    return {
      plugins:[
        (await import('./plugin/mini_html.js')).default
      ]
      base: '/'
      build:
        rollupOptions:
          output:
            chunkFileNames: JSNAME
            assetFileNames: FILENAME
            entryFileNames: "m.js"
    }
  else
    return {
      plugins:[
        {
          name:'html-img-src'
          transformIndexHtml:(html)=>
            html.replaceAll(
              'src=":/'
              'src="/@fs'+FILE+'/'
            )
        }
      ]
    }


export default =>
  defineConfig config
