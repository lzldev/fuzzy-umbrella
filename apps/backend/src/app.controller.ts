import { Controller, Get } from '@nestjs/common';
import { AppService } from './app.service';
import { posts } from './lib/db/schema';
import { DB } from './lib/db/db';
import { Database } from './database/database.provider';

@Controller()
export class AppController {
  @Database()
  private database: DB;
  private appService: AppService;

  @Get()
  getHello(): string {
    return this.appService.getHello();
  }

  @Get('test')
  async test() {
    return this.database.select().from(posts).all();
  }
}
