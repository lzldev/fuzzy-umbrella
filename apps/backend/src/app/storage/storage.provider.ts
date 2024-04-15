import { S3Client } from '@aws-sdk/client-s3'
import { fromEnv } from '@aws-sdk/credential-providers'
import { FactoryProvider, Inject, applyDecorators } from '@nestjs/common'

export const StorageProviderToken = 'STORAGE_PROVIDER'
export type StorageProvider = S3Client

export const StorageProviderFactory: FactoryProvider<StorageProvider> = {
  provide: StorageProviderToken,
  useFactory: async () =>
    new S3Client({
      credentials: fromEnv(),
    }),
}

export function Storage() {
  return applyDecorators(Inject(StorageProviderToken))
}
