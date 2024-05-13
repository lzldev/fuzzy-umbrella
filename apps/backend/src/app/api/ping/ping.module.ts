import { Module } from "@nestjs/common";
import { PingController } from "./ping.controller";

@Module({
  providers: [],
  controllers: [PingController],
})
export class PingModule {}
