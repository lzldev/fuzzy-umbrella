import { Controller, Get } from '@nestjs/common';
import { AppService } from './app.service';

@Controller()
export class AppController {
  private appService: AppService;

  @Get()
  getHello(): string {
    return this.appService.getHello();
  }
}
