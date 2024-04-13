import { Controller, Get, Inject, Param, Post } from '@nestjs/common';
import { PingService } from './ping.service';
import { CookieRecord, Cookies } from 'src/lib/Cookies';

@Controller('ping')
export class PingController {
  @Inject()
  private readonly pingService: PingService;

  @Get()
  ping(@Cookies() cookies: CookieRecord) {
    console.log(cookies['__session']);
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
