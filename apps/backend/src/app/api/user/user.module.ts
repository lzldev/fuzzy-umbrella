import { Module } from "@nestjs/common";
import { ProfileService } from "./profile/profile.service";
import { ProfileController } from "./profile/profile.controller";
import { ClerkModule } from "../auth/clerk/clerk.module";

@Module({
  imports: [],
  providers: [ProfileService],
  controllers: [ProfileController],
})
export class UserModule {}
