import { defineConfig } from 'vite'
import { viteSingleFile } from 'vite-plugin-singlefile'

export default defineConfig({
  plugins: [viteSingleFile()],
  build: {
    outDir: 'dist',
    rollupOptions: {
      input: 'index.html',
      output: {
        entryFileNames: 'template.js',
        chunkFileNames: 'template.js',
        assetFileNames: 'template.[ext]',
        inlineDynamicImports: true
      }
    },
    minify: true,
    sourcemap: false,
    assetsInlineLimit: 100000000 // Inline all assets
  },
  server: {
    port: 3000,
    open: true
  }
})
