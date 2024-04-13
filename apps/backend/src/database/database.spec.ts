import { Test, type TestingModule } from '@nestjs/testing';
import { DatabaseProvider, DatabaseProviderToken } from './database.provider';
import type { Connection } from '~/lib/db/connection';
import { ConfigModule } from '~/config/config.module';

describe('Database', () => {
  let provider: Connection;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      imports: [ConfigModule],
      providers: [DatabaseProvider],
    }).compile();

    provider = module.get(DatabaseProviderToken);
  });

  it('should be defined', () => {
    expect(provider).toBeDefined();
  });
});
