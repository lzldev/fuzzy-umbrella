import { Inject, applyDecorators } from '@nestjs/common';
import { EnvProvider } from 'src/config/env.provider';
import { createConnection } from 'src/lib/db/connection';

export const DatabaseProviderInject = 'DATABASE';

export const DatabaseProvider = {
  inject: [EnvProvider],
  provide: DatabaseProviderInject,
  useFactory: async (config: EnvProvider) => {
    return createConnection(
      config.turso_connection_url,
      config.turso_auth_token,
    );
  },
};

export function Database() {
  return applyDecorators(Inject(DatabaseProviderInject));
}
