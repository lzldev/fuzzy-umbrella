import { createPresignedPost } from '@aws-sdk/s3-presigned-post'
import { Injectable } from '@nestjs/common'
import { Storage, StorageProvider } from './storage.provider'

@Injectable()
export class StorageService {
  @Storage()
  private storage: StorageProvider

  //https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-UsingHTTPPOST.html
  //User needs https://docs.aws.amazon.com/AmazonS3/latest/API/RESTObjectPOST.html
  public async presignedPost(bucket: string, key: string) {
    return await createPresignedPost(this.storage, {
      Bucket: bucket, //TODO: GET BUCKET NAME FROM ENV
      Key: key,
      Expires: 3600,
    })

  }
}
