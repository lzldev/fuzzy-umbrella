import { Module } from "@nestjs/common";
import { JWKSProviderFactory } from "./jwks.provider";

@Module({
  providers: [JWKSProviderFactory],
  exports: [JWKSProviderFactory],
})
export class JWTModule {}
