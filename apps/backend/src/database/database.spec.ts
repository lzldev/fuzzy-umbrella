import { Test, type TestingModule } from '@nestjs/testing'
import { ConfigModule } from '~/config/config.module'
import type { Connection } from '~/lib/db/connection'
import { DatabaseProvider, DatabaseProviderToken } from './database.provider'

describe('Database', () => {
  let provider: Connection

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      imports: [ConfigModule],
      providers: [DatabaseProvider],
    }).compile()

    provider = module.get(DatabaseProviderToken)
  })

  it('should be defined', () => {
    expect(provider).toBeDefined()
  })
})
