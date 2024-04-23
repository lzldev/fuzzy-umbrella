import * as aws from "@pulumi/aws";
import * as archive from "@pulumi/archive";
import { ManagedPolicies, ManagedPolicy, Principals } from "@pulumi/aws/iam";
import { Runtime } from "@pulumi/aws/lambda";
import * as awsx from "@pulumi/awsx";
import * as pulumi from "@pulumi/pulumi";
import { Archive } from "@pulumi/pulumi/asset";

const AWS_Managed_CachingOptimized_CACHE_POLICY_ID =
  "658327ea-f89d-4fab-a63d-7e88639e58f6";

const cloudFrontContentOriginAcessControl =
  new aws.cloudfront.OriginAccessControl("example", {
    name: "s3-content-policy",
    description: "Policy for cloudfront",
    originAccessControlOriginType: "s3",
    signingBehavior: "always",
    signingProtocol: "sigv4",
  });

const postsBucket = new aws.s3.Bucket("mediathing-posts", {
  corsRules: [
    {
      allowedOrigins: ["http://localhost:3000", "http://localhost:5173"],
      allowedMethods: ["POST"],
    },
  ],
});

const contentBucketCloudFrontOriginID = "artspace-content-bucket";
const contentBucket = new aws.s3.Bucket(
  "mediathing-content",
  {
    corsRules: [
      {
        allowedOrigins: ["http://localhost:3000", "http://localhost:5173"],
        allowedMethods: ["POST"],
      },
    ],
  },
  {
    aliases: [
      "urn:pulumi:dev::mediathing::aws:s3/bucket:Bucket::mediathing-content",
    ],
  }
);

const assumeRole = aws.iam.getPolicyDocument({
  statements: [
    {
      effect: "Allow",
      principals: [
        {
          type: "Service",
          identifiers: ["lambda.amazonaws.com"],
        },
      ],
      actions: ["sts:AssumeRole"],
    },
  ],
});

const iamForLambda = new aws.iam.Role("iam_for_lambda", {
  name: "iam_for_lambda",
  assumeRolePolicy: assumeRole.then((assumeRole) => assumeRole.json),
});

new aws.iam.RolePolicyAttachment("lambda_role_execute_attach", {
  role: iamForLambda,
  policyArn: ManagedPolicy.AWSLambdaExecute,
});

const processPostHandlerName = "process-post-handler";

const processPostHandler = new aws.lambda.Function(processPostHandlerName, {
  name: processPostHandlerName,
  code: new pulumi.asset.FileArchive("fns3process.zip"),
  handler: "rust.handler",
  role: iamForLambda.arn,
  runtime: Runtime.CustomAL2023,
  environment: {
    variables: pulumi.all([contentBucket.bucket]).apply(([contentBucket]) => ({
      OUTPUT_BUCKET: contentBucket,
    })),
  },
});

//TODO: This should't be required if the stack is destroyed
const processPostLogGroup = new aws.cloudwatch.LogGroup("example", {
  name: `/aws/lambda/${processPostHandlerName}`,
  retentionInDays: 1,
});

postsBucket.onObjectCreated(processPostHandlerName, processPostHandler);

const cloudFrontDistribution = new aws.cloudfront.Distribution(
  "artspace-content",
  {
    enabled: true,
    isIpv6Enabled: true,
    viewerCertificate: {
      cloudfrontDefaultCertificate: true,
    },
    defaultCacheBehavior: {
      cachePolicyId: AWS_Managed_CachingOptimized_CACHE_POLICY_ID,
      viewerProtocolPolicy: "redirect-to-https",
      allowedMethods: ["GET", "HEAD", "OPTIONS"],
      cachedMethods: ["GET", "HEAD"],
      targetOriginId: contentBucketCloudFrontOriginID,
    },
    origins: [
      {
        originId: contentBucketCloudFrontOriginID,
        domainName: contentBucket.bucketRegionalDomainName,
        originAccessControlId: cloudFrontContentOriginAcessControl.id,
      },
    ],
    restrictions: {
      geoRestriction: {
        restrictionType: "none",
        locations: [],
      },
    },
  }
);

const allowReadFromOAC = aws.iam.getPolicyDocumentOutput({
  policyId: "AllowReadFromCloudFrontOAC",
  version: "2012-10-17",
  statements: [
    {
      sid: "AllowCloudFrontServicePrincipalReadWrite",
      effect: "Allow",
      principals: [
        {
          type: "Service",
          identifiers: ["cloudfront.amazonaws.com"],
        },
      ],
      actions: ["s3:GetObject", "s3:PutObject"],
      resources: [contentBucket.arn.apply((arn) => `${arn}/*`)],
      conditions: [
        {
          test: "StringEquals",
          variable: "AWS:SourceArn",
          values: [cloudFrontDistribution.arn],
        },
      ],
    },
  ],
});

const contentBucketOACPolicy = new aws.s3.BucketPolicy(
  "artspace-content-bucket-oac-policy",
  {
    bucket: contentBucket.bucket,
    policy: allowReadFromOAC.json,
  }
);

export const posts_bucket_name = postsBucket.id;
export const content_bucket_name = contentBucket.id;
export const cloudFront_domain = cloudFrontDistribution.domainName;
