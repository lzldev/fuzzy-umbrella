import {
  type ArgumentMetadata,
  HttpException,
  Injectable,
  type PipeTransform,
} from "@nestjs/common";

@Injectable()
export class UploadValidationPipe implements PipeTransform {
  transform(value: Express.Multer.File, metadata: ArgumentMetadata) {
    console.log(value);
    console.log(`${(value.size / 1024 / 1024).toFixed(2)} mb`);
    if (value.size > 1024 * 1024 * 10) {
      throw new HttpException("File too large", 413);
    }
    return value;
  }
}
