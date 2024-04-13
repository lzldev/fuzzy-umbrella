import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';
import { readFileSync } from 'fs';

async function bootstrap() {
  const app = await NestFactory.create(AppModule, {
    httpsOptions: {
      key: readFileSync('./src/certificates/localhost-key.pem'),
      cert: readFileSync('./src/certificates/localhost.pem'),
    },
  });
  await app.listen(3000);
}
bootstrap();
