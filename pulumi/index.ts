import * as aws from "@pulumi/aws";
import * as archive from "@pulumi/archive";
import { ManagedPolicies, ManagedPolicy, Principals } from "@pulumi/aws/iam";
import { Runtime } from "@pulumi/aws/lambda";
import * as awsx from "@pulumi/awsx";
import * as pulumi from "@pulumi/pulumi";
import { Archive } from "@pulumi/pulumi/asset";
import { readFileSync } from "fs";

const svixIps = (() => {
  const ips = JSON.parse(readFileSync("./svixip.json").toString());

  return Object.entries(ips).flatMap(([k, v]) => {
    return Object.values(v as object);
  });
})();

const config = new pulumi.Config();

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

const iamForLambdaWebhooks = new aws.iam.Role("iam_for_lambda_webhooks", {
  name: "iam_for_lambda_webhooks",
  assumeRolePolicy: assumeRole.then((assumeRole) => assumeRole.json),
});

const svixExecuteFunctionsDocument = aws.iam.getPolicyDocumentOutput({
  policyId: "SVIX",
  statements: [
    {
      effect: "Allow",
      actions: ["lambda:InvokeFunctionUrl"],
      resources: ["*"],
      conditions: [
        { test: "IpAddress", variable: "aws:sourceIp", values: svixIps },
      ],
    },
  ],
});

const svixInvokeFunctionPolicy = new aws.iam.Policy(
  "svix_invoke_function_policy",
  {
    description: "Allow svix ip's to invoke the lambda url",
    policy: svixExecuteFunctionsDocument.json,
  }
);

new aws.iam.RolePolicyAttachment("lambda_webhook_svix_invoke_attach", {
  role: iamForLambdaWebhooks,
  policyArn: svixInvokeFunctionPolicy.arn,
});

const iamForLambda = new aws.iam.Role("iam_for_lambda", {
  name: "iam_for_lambda",
  assumeRolePolicy: assumeRole.then((assumeRole) => assumeRole.json),
});

new aws.iam.RolePolicyAttachment("lambda_role_execute_attach", {
  role: iamForLambda,
  policyArn: ManagedPolicy.AWSLambdaExecute,
});

new aws.iam.RolePolicyAttachment("lambda_elasticache_attach", {
  role: iamForLambda,
  policyArn: ManagedPolicy.AmazonElastiCacheFullAccess,
});

new aws.iam.RolePolicyAttachment("lambda_vpc_attach", {
  role: iamForLambda,
  policyArn: ManagedPolicy.AWSLambdaVPCAccessExecutionRole,
});

const processPostHandlerName = "process-post-handler";

const redis = new aws.elasticache.Cluster("artspace-cache", {
  clusterId: "artspace-cache",
  engine: "redis",
  nodeType: "cache.t3.micro",
  azMode: "single-az",
  numCacheNodes: 1,
  parameterGroupName: "default.redis7",
  engineVersion: "7.1",
  ipDiscovery: "ipv4",
  networkType: "ipv4",
  port: 6379,
});

const vpc = new aws.ec2.DefaultVpc("default-vpc", {});

const subnet = new aws.ec2.DefaultSubnet("default-subnet", {
  availabilityZone: "sa-east-1a",
  assignIpv6AddressOnCreation: true,
});

const routeTable = new aws.ec2.DefaultRouteTable("default-route-table", {
  defaultRouteTableId: vpc.defaultRouteTableId,
});

const processPostHandler = new aws.lambda.Function(processPostHandlerName, {
  name: processPostHandlerName,
  code: new pulumi.asset.FileArchive(
    "../target/lambda/lambda_s3_process/bootstrap.zip"
  ),
  handler: "rust.handler",
  role: iamForLambda.arn,
  runtime: Runtime.CustomAL2023,
  timeout: 15,
  vpcConfig: {
    subnetIds: subnet.id.apply((id) => [id]),
    ipv6AllowedForDualStack: true,
    securityGroupIds: vpc.defaultSecurityGroupId.apply((id) => [id]),
  },
  loggingConfig: {
    logFormat: "JSON",
    applicationLogLevel: "DEBUG",
  },
  environment: {
    variables: pulumi
      .all([
        contentBucket.bucket,
        config.getSecret("TURSO_URL")!,
        config.getSecret("TURSO_TOKEN")!,
        redis.cacheNodes.apply((arr) => {
          const node = arr.at(0)!;

          return `${node.address}:${node.port}`;
        }),
      ])
      .apply(([contentBucket, turso_url, turso_token, redis_address]) => ({
        RUST_LOG: "DEBUG",
        OUTPUT_BUCKET: contentBucket,
        TURSO_URL: turso_url,
        TURSO_TOKEN: turso_token,
        REDIS_URL: `redis://${redis_address}`,
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
