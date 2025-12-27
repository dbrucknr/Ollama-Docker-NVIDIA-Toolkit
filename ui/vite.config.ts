import { defineConfig } from "vite";
// Plugins
import solid from "vite-plugin-solid";
import tailwindcss from "@tailwindcss/vite";
import path from "path";

export default defineConfig({
  plugins: [solid(), tailwindcss()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "src"),
    },
  },
  server: {
    proxy: {
      // Proxy requests that start with "/api" to our Rust backend (Dev Only)
      "/api": {
        target: "http://localhost:8000",
      },
    },
  },
  build: {
    outDir: "../core/static",
  },
});
