import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "path";

const sourceDir: string = "src";
const outDir: string = path.resolve("..", "backend", "public")

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [react()],
    /*
     * Build output dir set the public directory for the
     * backend project empty out dir is enabled to delete
     * the old files when building
     */
    build: {
        outDir,
        emptyOutDir: true,
    },
    /*
    * Resolving import aliases to clean up imports and
    * making them more distinct to avoid lots of relative
    * ../../ to find files
    */
    resolve: {
        alias: {
            "$components": path.resolve(sourceDir, "components"),
            "$assets": path.resolve(sourceDir, "assets"),
            "$api": path.resolve(sourceDir, "api"),
            "$routes": path.resolve(sourceDir, "routes"),
            "$app": sourceDir
        }
    }
})
