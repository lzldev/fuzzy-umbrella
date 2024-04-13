import * as aws from '@pulumi/aws'
import * as awsx from '@pulumi/awsx'
import * as pulumi from '@pulumi/pulumi'

const postsBucket = new aws.s3.Bucket('mediathing-posts')

export const bucketName = postsBucket.id
