import fs from 'fs';
import path from 'path';
import { defineConfig } from 'vite'
import tailwindcss from '@tailwindcss/vite'

const fonts = [
    '@fontsource-variable/geist/files/geist-latin-wght-normal.woff2',
    '@fontsource-variable/geist-mono/files/geist-mono-latin-wght-normal.woff2',
    '@fontsource-variable/newsreader/files/newsreader-latin-opsz-normal.woff2',
    '@fontsource-variable/newsreader/files/newsreader-latin-opsz-italic.woff2',
];

function copyFonts() {
    return {
        name: 'copy-fonts',
        generateBundle() {
            for (const font of fonts) {
                this.emitFile({
                    type: 'asset',
                    fileName: `fonts/${path.basename(font)}`,
                    source: fs.readFileSync(path.resolve(import.meta.dirname, 'node_modules', font)),
                });
            }
        },
    };
}

export default defineConfig({
    plugins: [tailwindcss(), copyFonts()],
    mode: 'production',
    publicDir: 'public/non-existent-dir',

    build: {
        lib: {
          entry: path.resolve(import.meta.dirname, 'presentation/js/main.js'),
          name: 'main',
          fileName: (format) => `app.js`,
          cssFileName: 'style'
        },
        outDir: 'public/dist',
    }
})
