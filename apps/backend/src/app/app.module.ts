import { Module } from '@nestjs/common'
import { RouterModule } from '@nestjs/core'
import { ApiModule } from './api/api.module'
import { AppController } from './app.controller'
import { AppService } from './app.service'
import { ConfigModule } from './config/config.module'
import { DatabaseModule } from './database/database.module'
import { StorageModule } from './storage/storage.module'
import { UploadModule } from './upload/upload.module'

@Module({
  imports: [
    DatabaseModule,
    ConfigModule,
    UploadModule,
    StorageModule,
    ApiModule,
  ],
  providers: [AppService],
  controllers: [AppController],
})
export class AppModule {}
