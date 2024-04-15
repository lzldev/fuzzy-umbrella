import { Test, TestingModule } from '@nestjs/testing'
import { StorageModule } from '~/app/storage/storage.module'
import { PostsController } from './posts.controller'

describe('PostsController', () => {
  let controller: PostsController

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      imports: [StorageModule],
      controllers: [PostsController],
    }).compile()

    controller = module.get<PostsController>(PostsController)
  })

  it('should be defined', () => {
    expect(controller).toBeDefined()
  })
})
