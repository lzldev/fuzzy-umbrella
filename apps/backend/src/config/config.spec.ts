import { Test, type TestingModule } from '@nestjs/testing';
import { EnvProvider } from './env.provider';

describe('Config', () => {
  let provider: EnvProvider;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [EnvProvider],
    }).compile();

    provider = module.get<EnvProvider>(EnvProvider);
  });

  it('should be defined', () => {
    expect(provider).toBeDefined();
  });
});
