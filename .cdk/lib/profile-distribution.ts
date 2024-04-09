import { Construct } from "constructs";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as s3Deploy from "aws-cdk-lib/aws-s3-deployment";
import * as cloudfront from "aws-cdk-lib/aws-cloudfront";
import * as cloudfrontOrigins from "aws-cdk-lib/aws-cloudfront-origins";
import * as acm from "aws-cdk-lib/aws-certificatemanager";
import { RemovalPolicy } from "aws-cdk-lib";
import { CloudFrontWebDistribution, OriginAccessIdentity } from "aws-cdk-lib/aws-cloudfront";

type ProfileDistrubutionProps = {
  // domainName: string;
  // profileSubDomainName: string;
  bucket: s3.Bucket;
};

export class ProfileDistribution extends Construct {
  distribution: cloudfront.IDistribution;
  constructor(scope: Construct, props: ProfileDistrubutionProps) {
    super(scope, "profile-distribution");

    const oai = new OriginAccessIdentity(this, `profile-distribution-origin-access-id`, {});
    props.bucket.grantRead(oai);

    this.distribution = new CloudFrontWebDistribution(this, `profile-web-distribution`, {
      originConfigs: [
        {
          s3OriginSource: {
            s3BucketSource: props.bucket,
            originAccessIdentity: oai
          },
          behaviors: [{ isDefaultBehavior: true }],
        },
      ],
    });
  }
}