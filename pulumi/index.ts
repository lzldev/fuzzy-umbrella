import * as aws from '@pulumi/aws'
import * as awsx from '@pulumi/awsx'
import * as pulumi from '@pulumi/pulumi'

const postsBucket = new aws.s3.Bucket('mediathing-posts',{
   corsRules:[{
        allowedOrigins:['http://localhost:3000','http://localhost:5173'],
        allowedMethods:['POST'],
   }] 
},{aliases:['mediathing-posts']})

export const bucketName = postsBucket.id
