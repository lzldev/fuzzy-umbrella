import "dotenv/config";
import { defineConfig } from "drizzle-kit";

export default defineConfig({
  strict: true,
  verbose: true,
  dialect: "postgresql",
  schema: "./src/db/schema.ts",
  out: "./migrations",
  dbCredentials: {
    url: process.env.PG_URL!,
  },
});
