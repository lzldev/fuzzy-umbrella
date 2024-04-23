import { Global, Module } from "@nestjs/common";
import { DatabaseProviderFactory } from "./database.provider";

@Global()
@Module({
  imports: [],
  providers: [DatabaseProviderFactory],
  exports: [DatabaseProviderFactory],
})
export class DatabaseModule {}
