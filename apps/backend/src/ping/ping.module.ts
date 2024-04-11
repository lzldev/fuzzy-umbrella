import { Module } from '@nestjs/common';
import { PingController, PingService } from './ping.controller';

@Module({
  controllers: [PingController],
  providers: [PingService],
})
export class PingModule {}
