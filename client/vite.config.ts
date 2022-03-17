import * as path from 'path';
import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import icons from 'unplugin-icons/vite';
import iconsResolver from 'unplugin-icons/resolver';
import components from 'unplugin-vue-components/vite';

// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: {
      '~': path.resolve(__dirname, 'src'),
    },
  },
  plugins: [
    vue({
      reactivityTransform: true,
    }),
    icons({
      scale: 1.25,
    }),
    components({
      dirs: ['src/components'],
      resolvers: [iconsResolver()],
      dts: 'src/types/components.d.ts',
    }),
  ],
});
