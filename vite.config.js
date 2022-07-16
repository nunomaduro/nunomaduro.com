import path from 'path';
import { defineConfig } from 'vite'

export default defineConfig({
    mode: 'production',
    publicDir: 'public/non-existent-dir',

    build: {
        lib: {
          entry: path.resolve(__dirname, 'presentation/js/main.js'),
          name: 'main',
          fileName: (format) => `app.js`
        },
        outDir: 'public/js',
    }
})
