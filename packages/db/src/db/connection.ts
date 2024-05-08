import "dotenv/config";

import { createClient } from "@libsql/client";
import { drizzle } from "drizzle-orm/libsql";
import * as schema from "./schema.js";

type Schema = typeof schema;

export function createConnection(connectingUrl: string, authToken: string) {
  const client = createClient({
    url: connectingUrl,
    authToken: authToken,
  });

  return drizzle<Schema>(client, {
    schema,
  });
}

export type Connection = ReturnType<typeof createConnection>;
