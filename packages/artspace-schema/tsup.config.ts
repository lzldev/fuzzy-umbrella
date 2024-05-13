import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src/index.ts"],
  outDir: "./dist",
  splitting: true,
  dts: true,
  sourcemap: true,
  clean: true,
  platform: "browser",
  external: ["@artspace/db"],
  format: ["cjs", "esm"],
});
