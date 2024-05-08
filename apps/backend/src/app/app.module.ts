import { Module } from "@nestjs/common";
import { ApiModule } from "./api/api.module";
import { AppController } from "./app.controller";
import { AppService } from "./app.service";
import { ConfigModule } from "./config/config.module";
import { DatabaseModule } from "./database/database.module";
import { StorageModule } from "./storage/storage.module";
import { UploadModule } from "./upload/upload.module";
import { RedisModule } from "./redis/redis.module";
import { APP_GUARD } from "@nestjs/core";
import { ClerkGuard } from "./api/auth/clerk/clerk.guard";

@Module({
  imports: [
    ConfigModule,
    DatabaseModule,
    RedisModule,
    UploadModule,
    StorageModule,
    ApiModule,
  ],
  providers: [AppService],
  controllers: [AppController],
})
export class AppModule {}
