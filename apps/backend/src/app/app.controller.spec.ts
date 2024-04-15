import { Test, TestingModule } from '@nestjs/testing'
import { PingModule } from './api/ping/ping.module'
import { AppController } from './app.controller'
import { AppService } from './app.service'
import { ConfigModule } from './config/config.module'
import { DatabaseModule } from './database/database.module'
import { UploadModule } from './upload/upload.module'

describe('AppController', () => {
  let appController: AppController

  beforeEach(async () => {
    const app: TestingModule = await Test.createTestingModule({
      imports: [PingModule, DatabaseModule, ConfigModule, UploadModule],
      controllers: [AppController],
      providers: [AppService],
    }).compile()

    appController = app.get<AppController>(AppController)
  })

  describe('root', () => {
    it('should return "Hello World!"', () => {
      expect(appController.getHello()).toBe('Hello World!')
    })
  })
})
