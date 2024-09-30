import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { RustFunction } from "cargo-lambda-cdk";
import * as awsLambda from "aws-cdk-lib/aws-lambda";
import * as awsCloudfront from "aws-cdk-lib/aws-cloudfront";
import * as awsCloudfrontOrigins from "aws-cdk-lib/aws-cloudfront-origins";
import * as awsS3 from "aws-cdk-lib/aws-s3";
import * as awsS3Deployment from "aws-cdk-lib/aws-s3-deployment";
import * as awsIam from "aws-cdk-lib/aws-iam";
// import * as sqs from 'aws-cdk-lib/aws-sqs';

export class ProfileLeptosStack extends cdk.Stack {
  private rootDomainName = "vpurush.com";
  private profileSubDomainName = "profile.vpurush.com";

  private bucket: awsS3.IBucket;
  private distribution: awsCloudfront.Distribution;
  private cloudfrontOAI: awsCloudfront.OriginAccessIdentity;
  private functionUrl: awsLambda.FunctionUrl;

  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // The code that defines your stack goes here
    const rustFunction = new RustFunction(this, "ProfileLeptosRustFunction", {
      manifestPath: "../Cargo.toml",
      bundling: {
        environment: {
          LEPTOS_OUTPUT_NAME: "profile",
          CONTENTFUL_PREVIEW_TOKEN: process.env.CONTENTFUL_PREVIEW_TOKEN,
        },
        cargoLambdaFlags: [
          "--package=profile",
          "--bin=profile",
          "--no-default-features",
          "--features=ssr",
          "--release",
        ],
      },
    });

    this.functionUrl = new awsLambda.FunctionUrl(
      this,
      "ProfileLeptosRustFunctionUrl",
      {
        function: rustFunction,
        authType: cdk.aws_lambda.FunctionUrlAuthType.NONE,
      }
    );

    new cdk.CfnOutput(this, "ProfileLeptosRustFunctionUrlOutput", {
      value: this.functionUrl.url,
    });

    this.bucket = this.createS3Bucket("tempbucketforprofileleptos");
    const { distribution, cloudfrontOAI } = this.createCDN(
      "",
      "",
      this.bucket,
      this.functionUrl
    );
    this.distribution = distribution;
    this.cloudfrontOAI = cloudfrontOAI;

    this.bucket.addToResourcePolicy(
      new awsIam.PolicyStatement({
        actions: ["s3:GetObject"],
        resources: [this.bucket.arnForObjects("*")],
        principals: [
          new awsIam.CanonicalUserPrincipal(
            this.cloudfrontOAI.cloudFrontOriginAccessIdentityS3CanonicalUserId
          ),
        ],
      })
    );

    this.deployToS3Bucket(this.distribution, this.bucket);
  }

  createCDN(
    domainName: string,
    profileSubDomainName: string,
    bucket: awsS3.IBucket,
    functionUrl: awsLambda.FunctionUrl
  ) {
    const cloudfrontOAI = new awsCloudfront.OriginAccessIdentity(
      this,
      "ProfileLeptosCloudfrontOAI",
      {
        comment: `OAI for ProfileLeptosCloudfront`,
      }
    );

    const distribution = new awsCloudfront.Distribution(
      this,
      "ProfileLeptosDistribution",
      {
        // domainNames: [domainName, profileSubDomainName],
        minimumProtocolVersion:
          awsCloudfront.SecurityPolicyProtocol.TLS_V1_2_2021,
        defaultBehavior: {
          origin: new awsCloudfrontOrigins.FunctionUrlOrigin(functionUrl),
          compress: true,
          allowedMethods: awsCloudfront.AllowedMethods.ALLOW_GET_HEAD_OPTIONS,
          viewerProtocolPolicy:
            awsCloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
        },
        additionalBehaviors: {
          "/pkg/*": {
            origin: new awsCloudfrontOrigins.S3Origin(bucket, {
              originAccessIdentity: cloudfrontOAI,
            }),
            compress: true,
            allowedMethods: awsCloudfront.AllowedMethods.ALLOW_GET_HEAD_OPTIONS,
            viewerProtocolPolicy:
              awsCloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
          },
          "/favicon.ico": {
            origin: new awsCloudfrontOrigins.S3Origin(bucket, {
              originAccessIdentity: cloudfrontOAI,
            }),
            compress: true,
            allowedMethods: awsCloudfront.AllowedMethods.ALLOW_GET_HEAD_OPTIONS,
            viewerProtocolPolicy:
              awsCloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
          },
        },
        // certificate: props.certificate,
      }
    );


    new cdk.CfnOutput(this, "ProfileLeptosCloudfrontUrlOutput", {
      value: distribution.distributionDomainName,
    });

    return {
      distribution,
      cloudfrontOAI,
    };
  }

  createS3Bucket(bucketName: string) {
    return new awsS3.Bucket(this, "ProfileLeptosS3Bucket", {
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
      bucketName: bucketName,
      publicReadAccess: false,
      blockPublicAccess: awsS3.BlockPublicAccess.BLOCK_ALL,
    });
  }

  deployToS3Bucket(
    distribution: awsCloudfront.Distribution,
    bucket: awsS3.IBucket
  ) {
    return new awsS3Deployment.BucketDeployment(
      this,
      "ProfileLeptosS3BucketDeployment",
      {
        destinationBucket: bucket,
        sources: [awsS3Deployment.Source.asset("../target/site")],
        distribution: distribution,
        distributionPaths: ["/favicon.ico", "/pkg/*"],
      }
    );
  }
}
