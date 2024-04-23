import { Module } from "@nestjs/common";
import { RedisClientProviderFactory } from "./redis.provider";

@Module({
  providers: [RedisClientProviderFactory],
  exports: [RedisClientProviderFactory],
})
export class RedisModule {}
