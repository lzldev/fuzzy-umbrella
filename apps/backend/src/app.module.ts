import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { PingModule } from './ping/ping.module';
import { DatabaseModule } from './database/database.module';
import { ConfigModule } from './config/config.module';
import { UploadModule } from './upload/upload.module';

@Module({
  imports: [PingModule, DatabaseModule, ConfigModule, UploadModule],
  providers: [AppService],
  controllers: [AppController],
})
export class AppModule {}
