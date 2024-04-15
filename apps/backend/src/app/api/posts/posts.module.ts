import { Module } from '@nestjs/common'
import { StorageModule } from '~/app/storage/storage.module'
import { PostsController } from './posts.controller'

@Module({
  imports: [StorageModule],
  controllers: [PostsController],
})
export class PostsModule {}
