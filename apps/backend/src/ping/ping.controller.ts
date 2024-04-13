import { Controller, Get, Inject, Param, Post, Req } from '@nestjs/common';
import { PingService } from './ping.service';
import { Request } from 'express';

@Controller('ping')
export class PingController {
  @Inject()
  private readonly pingService: PingService;

  @Get()
  ping(@Req() cookies: Request) {
    console.log(cookies.cookies);

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
