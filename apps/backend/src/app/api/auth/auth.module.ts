import { Module } from "@nestjs/common";
import { ClerkModule } from "./clerk/clerk.module";
import { JWTModule } from "./jwt/jwt.module";

@Module({
  imports: [ClerkModule, JWTModule],
  providers: [],
  exports: [ClerkModule, JWTModule],
})
export class AuthModule {}
