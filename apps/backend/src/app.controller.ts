import {
  Body,
  Controller,
  Get,
  Inject,
  Post,
  UploadedFile,
  UseInterceptors,
} from '@nestjs/common';
import { AppService } from './app.service';
import { FileInterceptor } from '@nestjs/platform-express';
import { IsString } from 'class-validator';
import { UploadValidationPipe } from './upload/upload.validation.pipe';
import { EnvProvider } from './config/env.provider';
import { ReadStream, createWriteStream } from 'fs';
import { resolve } from 'path';

class FunnyForm {
  @IsString()
  text: string;
}

@Controller()
export class AppController {
  @Inject()
  env: EnvProvider;

  @Inject()
  private appService: AppService;

  @Get()
  getHello(): string {
    return this.appService.getHello();
  }

  @Post('/form')
  @UseInterceptors(FileInterceptor('file'))
  async form(
    @UploadedFile(new UploadValidationPipe())
    file: Express.Multer.File,
    @Body() form: FunnyForm,
  ) {
    const location = `${file.originalname}`;
    console.log(this.env.upload_location, location);

    ReadStream.from(file.buffer).pipe(
      createWriteStream(resolve(this.env.upload_location, location)),
    );

    return `the file is ${file.size / 1024 / 1024} MB`;
  }
}
