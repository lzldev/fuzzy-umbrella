import { Module } from "@nestjs/common";
import { ClerkClientProviderFactory } from "./clerk.client";
import { ClerkGuard } from "./clerk.guard";
import { JWTModule } from "../jwt/jwt.module";

@Module({
  imports: [JWTModule],
  providers: [ClerkClientProviderFactory, ClerkGuard],
  exports: [ClerkClientProviderFactory, ClerkGuard],
})
export class ClerkModule {}
