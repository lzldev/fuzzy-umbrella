import { Module } from '@nestjs/common';
import { DatabaseProvider } from './database.provider';

@Module({
  imports: [],
  providers: [DatabaseProvider],
  exports: [DatabaseProvider],
})
export class DatabaseModule {}
