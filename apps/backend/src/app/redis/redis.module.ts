import { Global, Module } from "@nestjs/common";
import { RedisClientProviderFactory } from "./redis.provider";

@Global()
@Module({
  providers: [RedisClientProviderFactory],
  exports: [RedisClientProviderFactory],
})
export class RedisModule {}
