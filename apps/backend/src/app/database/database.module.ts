import { Module } from "@nestjs/common";
import { DatabaseProviderFactory } from "./database.provider";

@Module({
  imports: [],
  providers: [DatabaseProviderFactory],
  exports: [DatabaseProviderFactory],
})
export class DatabaseModule {}
