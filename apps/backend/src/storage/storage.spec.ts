import { Test, TestingModule } from '@nestjs/testing';
import { Storage } from './storage';

describe('Storage', () => {
  let provider: Storage;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [Storage],
    }).compile();

    provider = module.get<Storage>(Storage);
  });

  it('should be defined', () => {
    expect(provider).toBeDefined();
  });
});
