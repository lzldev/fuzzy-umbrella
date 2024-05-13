import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src/index.ts"],
  outDir: "./dist",
  splitting: false,
  sourcemap: true,
  dts: true,
  external: ["dotenv", "dotenv-cli"],
  format: ["cjs", "esm"],
  clean: true,
});
