import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["bindings/index.ts"],
  outDir: "./dist",
  dts: true,
  splitting: false,
  sourcemap: true,
  clean: true,
});
