import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src/index.ts"],
  outDir: "./dist",
  splitting: false,
  sourcemap: true,
  format: ["cjs", "esm"],
  clean: true,
});
