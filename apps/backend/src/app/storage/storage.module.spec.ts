import { Test, type TestingModule } from '@nestjs/testing'
import {
  StorageProvider,
  StorageProviderFactory,
  StorageProviderToken,
} from './storage.provider'

describe('Storage', () => {
  let provider: StorageProvider

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      imports: [],
      providers: [StorageProviderFactory],
    }).compile()

    provider = module.get(StorageProviderToken)
  })

  it('should be defined', () => {
    expect(provider).toBeDefined()
  })
})
