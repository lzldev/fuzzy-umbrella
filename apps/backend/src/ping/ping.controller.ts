import { Controller, Get, Param, Post } from '@nestjs/common';
import { PingService } from './ping.service';

@Controller('ping')
export class PingController {
  private readonly pingService: PingService;

  @Get()
  ping() {
    return {
      data: this.pingService.entries(),
      time: new Date().toISOString(),
    };
  }
  @Get(':id')
  pingId(@Param('id') id: number) {
    return { data: this.pingService.get(id) };
  }
  @Post(':id')
  pongCreate(@Param('id') id: number) {
    this.pingService.push(id);
    return { data: this.pingService.push(id) };
  }
}
