import { Module } from "@nestjs/common";
import { ClerkClientProviderFactory } from "./clerk.client";
import { ClerkGuard } from "./clerk.guard";
import { JWTModule } from "../jwt/jwt.module";
import { ClerkService } from "./clerk.service";

@Module({
  imports: [JWTModule],
  providers: [ClerkClientProviderFactory, ClerkService, ClerkGuard],
  exports: [ClerkClientProviderFactory, ClerkService, ClerkGuard],
})
export class ClerkModule {}
