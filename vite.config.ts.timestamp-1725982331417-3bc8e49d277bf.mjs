// vite.config.ts
import { defineConfig } from "file:///home/jonas/Epic-Mickey-Launcher/node_modules/vite/dist/node/index.js";
import { svelte, vitePreprocess } from "file:///home/jonas/Epic-Mickey-Launcher/node_modules/.pnpm/@sveltejs+vite-plugin-svelte@2.5.3_svelte@3.59.2_vite@4.5.3/node_modules/@sveltejs/vite-plugin-svelte/src/index.js";
import pkg from "file:///home/jonas/Epic-Mickey-Launcher/node_modules/.pnpm/svelte-preprocess@5.1.4_postcss@8.4.45_svelte@3.59.2_typescript@5.5.3/node_modules/svelte-preprocess/dist/index.js";
var { typescript } = pkg;
var cfg = {
  plugins: [svelte({
    preprocess: [
      typescript({})
    ]
  })],
  preprocessor: [vitePreprocess()],
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true
  },
  // to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    // Tauri supports es2021
    target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG
  }
};
console.log(cfg);
var vite_config_default = defineConfig(cfg);
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCIvaG9tZS9qb25hcy9FcGljLU1pY2tleS1MYXVuY2hlclwiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9maWxlbmFtZSA9IFwiL2hvbWUvam9uYXMvRXBpYy1NaWNrZXktTGF1bmNoZXIvdml0ZS5jb25maWcudHNcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfaW1wb3J0X21ldGFfdXJsID0gXCJmaWxlOi8vL2hvbWUvam9uYXMvRXBpYy1NaWNrZXktTGF1bmNoZXIvdml0ZS5jb25maWcudHNcIjtpbXBvcnQgeyBkZWZpbmVDb25maWcgfSBmcm9tIFwidml0ZVwiO1xuaW1wb3J0IHsgc3ZlbHRlLCB2aXRlUHJlcHJvY2VzcyB9IGZyb20gXCJAc3ZlbHRlanMvdml0ZS1wbHVnaW4tc3ZlbHRlXCI7XG5pbXBvcnQgcGtnIGZyb20gXCJzdmVsdGUtcHJlcHJvY2Vzc1wiO1xuY29uc3QgeyB0eXBlc2NyaXB0IH0gPSBwa2dcblxubGV0IGNmZyA9IHtcbiAgcGx1Z2luczogW3N2ZWx0ZSh7XG4gICAgcHJlcHJvY2VzczogW1xuICAgICAgdHlwZXNjcmlwdCh7fSksXG4gICAgXSxcbiAgfSldLFxuICBwcmVwcm9jZXNzb3I6IFt2aXRlUHJlcHJvY2VzcygpXSxcbiAgLy8gVml0ZSBvcHRpb25zIHRhaWxvcmVkIGZvciBUYXVyaSBkZXZlbG9wbWVudCBhbmQgb25seSBhcHBsaWVkIGluIGB0YXVyaSBkZXZgIG9yIGB0YXVyaSBidWlsZGBcbiAgLy8gcHJldmVudCB2aXRlIGZyb20gb2JzY3VyaW5nIHJ1c3QgZXJyb3JzXG4gIGNsZWFyU2NyZWVuOiBmYWxzZSxcbiAgLy8gdGF1cmkgZXhwZWN0cyBhIGZpeGVkIHBvcnQsIGZhaWwgaWYgdGhhdCBwb3J0IGlzIG5vdCBhdmFpbGFibGVcbiAgc2VydmVyOiB7XG4gICAgcG9ydDogMTQyMCxcbiAgICBzdHJpY3RQb3J0OiB0cnVlLFxuICB9LFxuICAvLyB0byBtYWtlIHVzZSBvZiBgVEFVUklfREVCVUdgIGFuZCBvdGhlciBlbnYgdmFyaWFibGVzXG4gIC8vIGh0dHBzOi8vdGF1cmkuc3R1ZGlvL3YxL2FwaS9jb25maWcjYnVpbGRjb25maWcuYmVmb3JlZGV2Y29tbWFuZFxuICBlbnZQcmVmaXg6IFtcIlZJVEVfXCIsIFwiVEFVUklfXCJdLFxuICBidWlsZDoge1xuICAgIC8vIFRhdXJpIHN1cHBvcnRzIGVzMjAyMVxuICAgIHRhcmdldDogcHJvY2Vzcy5lbnYuVEFVUklfUExBVEZPUk0gPT0gXCJ3aW5kb3dzXCIgPyBcImNocm9tZTEwNVwiIDogXCJzYWZhcmkxM1wiLFxuICAgIC8vIGRvbid0IG1pbmlmeSBmb3IgZGVidWcgYnVpbGRzXG4gICAgbWluaWZ5OiAhcHJvY2Vzcy5lbnYuVEFVUklfREVCVUcgPyBcImVzYnVpbGRcIiA6IGZhbHNlLFxuICAgIC8vIHByb2R1Y2Ugc291cmNlbWFwcyBmb3IgZGVidWcgYnVpbGRzXG4gICAgc291cmNlbWFwOiAhIXByb2Nlc3MuZW52LlRBVVJJX0RFQlVHLFxuICB9LFxufVxuY29uc29sZS5sb2coY2ZnKVxuLy8gaHR0cHM6Ly92aXRlanMuZGV2L2NvbmZpZy9cbmV4cG9ydCBkZWZhdWx0IGRlZmluZUNvbmZpZyhjZmcpO1xuXG4iXSwKICAibWFwcGluZ3MiOiAiO0FBQWtSLFNBQVMsb0JBQW9CO0FBQy9TLFNBQVMsUUFBUSxzQkFBc0I7QUFDdkMsT0FBTyxTQUFTO0FBQ2hCLElBQU0sRUFBRSxXQUFXLElBQUk7QUFFdkIsSUFBSSxNQUFNO0FBQUEsRUFDUixTQUFTLENBQUMsT0FBTztBQUFBLElBQ2YsWUFBWTtBQUFBLE1BQ1YsV0FBVyxDQUFDLENBQUM7QUFBQSxJQUNmO0FBQUEsRUFDRixDQUFDLENBQUM7QUFBQSxFQUNGLGNBQWMsQ0FBQyxlQUFlLENBQUM7QUFBQTtBQUFBO0FBQUEsRUFHL0IsYUFBYTtBQUFBO0FBQUEsRUFFYixRQUFRO0FBQUEsSUFDTixNQUFNO0FBQUEsSUFDTixZQUFZO0FBQUEsRUFDZDtBQUFBO0FBQUE7QUFBQSxFQUdBLFdBQVcsQ0FBQyxTQUFTLFFBQVE7QUFBQSxFQUM3QixPQUFPO0FBQUE7QUFBQSxJQUVMLFFBQVEsUUFBUSxJQUFJLGtCQUFrQixZQUFZLGNBQWM7QUFBQTtBQUFBLElBRWhFLFFBQVEsQ0FBQyxRQUFRLElBQUksY0FBYyxZQUFZO0FBQUE7QUFBQSxJQUUvQyxXQUFXLENBQUMsQ0FBQyxRQUFRLElBQUk7QUFBQSxFQUMzQjtBQUNGO0FBQ0EsUUFBUSxJQUFJLEdBQUc7QUFFZixJQUFPLHNCQUFRLGFBQWEsR0FBRzsiLAogICJuYW1lcyI6IFtdCn0K
