import * as aws from '@pulumi/aws'
import * as awsx from '@pulumi/awsx'
import * as pulumi from '@pulumi/pulumi'

const postsBucket = new aws.s3.Bucket('mediathing-posts',{
   corsRules:[{
        allowedOrigins:['http://localhost:3000','http://localhost:5173'],
        allowedMethods:['POST'],
   }] 
})

const contentBucket = new aws.s3.Bucket('mediathing-content',{
   corsRules:[{
        allowedOrigins:['http://localhost:3000','http://localhost:5173'],
        allowedMethods:['POST'],
   }] 
},{aliases:["urn:pulumi:dev::mediathing::aws:s3/bucket:Bucket::mediathing-content"]})

export const posts_bucket_name = postsBucket.id
export const content_bucket_name = contentBucket.id