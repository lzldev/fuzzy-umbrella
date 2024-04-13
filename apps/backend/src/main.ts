import { NestFactory } from '@nestjs/core'
import { AppModule } from './app.module'
import * as cookieParser from 'cookie-parser'

async function bootstrap() {
  const app = await NestFactory.create(AppModule, {
    //TODO:move this into a module
    cors: {
      origin: process.env.CORS_FRONTEND_ORIGIN,
      credentials: true,
    },
  })

  //TODO: move this into a module
  app.use(cookieParser())

  await app.listen(3000)
}

bootstrap()
