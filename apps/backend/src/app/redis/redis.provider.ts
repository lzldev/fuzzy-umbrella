import { FactoryProvider, Inject, applyDecorators } from "@nestjs/common";
import { Redis } from "ioredis";
import { EnvProvider } from "../config/env.provider";

export const REDIS_PROVIDER_KEY = "";

export type RedisClientProvider = Redis;

export const RedisClientProviderFactory: FactoryProvider<RedisClientProvider> =
  {
    provide: REDIS_PROVIDER_KEY,
    inject: [EnvProvider],
    useFactory(env: EnvProvider) {
      const redis = new Redis({
        host: env.aws_elasticache_redis_cluster_endpoint_url,
      });

      return redis;
    },
  };

export function RedisClient() {
  return applyDecorators(Inject(REDIS_PROVIDER_KEY));
}
