import { FactoryProvider, Inject, applyDecorators } from "@nestjs/common";
import { Redis } from "ioredis";
import { EnvProvider } from "../config/env.provider";

export const REDIS_PROVIDER_KEY = "redis-client-provider";

export type RedisClientProvider = Redis;

export const RedisClientProviderFactory: FactoryProvider<RedisClientProvider> =
  {
    provide: REDIS_PROVIDER_KEY,
    inject: [EnvProvider],
    async useFactory(env: EnvProvider) {
      const redis = new Redis({
        host: env.aws_elasticache_redis_cluster_endpoint_url,
      });

      redis.options.maxRetriesPerRequest = 0;
      await redis.ping();
      redis.options.maxRetriesPerRequest = 20;

      return redis;
    },
  };

export function RedisClient() {
  return applyDecorators(Inject(REDIS_PROVIDER_KEY));
}
