import path from 'path';
import { defineConfig } from 'vite'
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({
    plugins: [tailwindcss()],
    mode: 'production',
    publicDir: 'public/non-existent-dir',

    build: {
        lib: {
          entry: path.resolve(__dirname, 'presentation/js/main.js'),
          name: 'main',
          fileName: (format) => `app.js`,
          cssFileName: 'style'
        },
        outDir: 'public/dist',
    }
})
