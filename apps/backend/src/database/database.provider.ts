import { Inject, applyDecorators } from '@nestjs/common';
import { createConnection } from 'src/lib/db/connection';

export const DatabaseProviderInject = 'DATABASE';

export const DatabaseProvider = {
  provide: DatabaseProviderInject,
  useFactory: async () => {
    return createConnection();
  },
};

export function Database() {
  return applyDecorators(Inject(DatabaseProviderInject));
}
