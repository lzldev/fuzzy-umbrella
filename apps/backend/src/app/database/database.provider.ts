import { schema, type Schema } from "@artspace/db";
import { Inject, applyDecorators, type FactoryProvider } from "@nestjs/common";
import { drizzle } from "drizzle-orm/postgres-js";
import * as postgres from "postgres";
import { EnvProvider } from "~/app/config/env.provider";

export function createConnection(connectionUri: string) {
  const client = postgres(connectionUri);

  return drizzle<Schema>(client, {
    schema,
  });
}

type Connection = ReturnType<typeof createConnection>;
export const DatabaseProviderToken = "DATABASE_PROVIDER";
export type DatabaseProvider = Connection;

export const DatabaseProviderFactory: FactoryProvider<DatabaseProvider> = {
  inject: [EnvProvider],
  provide: DatabaseProviderToken,
  useFactory: (env: EnvProvider) => {
    return createConnection(env.pg_pool_url);
  },
};

/**
 * Database Injection Decorator
 */
export function Database() {
  return applyDecorators(Inject(DatabaseProviderToken));
}
