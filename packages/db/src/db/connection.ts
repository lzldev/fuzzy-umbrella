import { drizzle } from "drizzle-orm/postgres-js";
import postgres from "postgres";
import * as schema from "./schema.js";

type Schema = typeof schema;

export function createConnection(connectionUri: string) {
  const client = postgres(connectionUri);

  return drizzle<Schema>(client, {
    schema,
  });
}

export type Connection = ReturnType<typeof createConnection>;
