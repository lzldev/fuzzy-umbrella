import "dotenv/config";
import type { Config } from "drizzle-kit";

export default {
  driver: "turso",
  schema: "./src/lib/db/schema.ts",
  out: "./migrations",
  dbCredentials: {
    url: process.env.TURSO_CONNECTION_URL!,
    authToken: process.env.TURSO_AUTH_TOKEN!,
  },
} satisfies Config;
