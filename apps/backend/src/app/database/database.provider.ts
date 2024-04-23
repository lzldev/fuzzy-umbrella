import { type FactoryProvider, Inject, applyDecorators } from "@nestjs/common";
import { EnvProvider } from "~/app/config/env.provider";
import { Connection, createConnection } from "~/lib/db/connection";

export const DatabaseProviderToken = "DATABASE_PROVIDER";
export type DatabaseProvider = Connection;

export const DatabaseProviderFactory: FactoryProvider<DatabaseProvider> = {
  inject: [EnvProvider],
  provide: DatabaseProviderToken,
  useFactory: (config: EnvProvider) => {
    return createConnection(
      config.turso_connection_url,
      config.turso_auth_token,
    );
  },
};

export function Database() {
  return applyDecorators(Inject(DatabaseProviderToken));
}
