import { Controller,  Inject, Post } from '@nestjs/common'
import { StorageService } from '~/app/storage/storage.service'

@Controller('posts')
export class PostsController {
  @Inject()
  private readonly storage: StorageService
  @Post('create')
  sendPost() {
    return this.storage.presignedPost('mediathing-posts-57edd0f', 'object.png')
  }
}