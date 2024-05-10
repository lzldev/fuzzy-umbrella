import { type FactoryProvider, Inject, applyDecorators } from "@nestjs/common";
import { EnvProvider } from "~/app/config/env.provider";
import { type Connection, createConnection } from "@artspace/db";

export const DatabaseProviderToken = "DATABASE_PROVIDER";
export type DatabaseProvider = Connection;

export const DatabaseProviderFactory: FactoryProvider<DatabaseProvider> = {
  inject: [EnvProvider],
  provide: DatabaseProviderToken,
  useFactory: (config: EnvProvider) => {
    return createConnection(
      config.turso_connection_url,
      config.turso_auth_token
    );
  },
};

/**
 * Database Injection Decorator
 */
export function Database() {
  return applyDecorators(Inject(DatabaseProviderToken));
}
