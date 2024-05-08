import { Global, Module } from "@nestjs/common";
import { ClerkModule } from "./clerk/clerk.module";
import { JWTModule } from "./jwt/jwt.module";

@Global()
@Module({
  imports: [ClerkModule, JWTModule],
  providers: [],
  exports: [ClerkModule, JWTModule],
})
export class AuthModule {}
