import { Module } from "@nestjs/common";
import { ProfileService } from "./profile/profile.service";
import { ProfileController } from "./profile/profile.controller";

@Module({
  imports: [],
  providers: [ProfileService],
  controllers: [ProfileController],
})
export class UserModule {}
