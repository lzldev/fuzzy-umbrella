import { Module } from "@nestjs/common";
import { StorageProviderFactory } from "./storage.provider";
import { StorageService } from "./storage.service";

@Module({
  providers: [StorageProviderFactory, StorageService],
  exports: [StorageService],
})
export class StorageModule {}
