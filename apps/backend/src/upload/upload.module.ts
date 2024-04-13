import { Module } from '@nestjs/common';
import { UploadValidationPipe } from './upload.validation.pipe';

@Module({
  imports: [],
  providers: [UploadValidationPipe],
  exports: [],
})
export class UploadModule {}
