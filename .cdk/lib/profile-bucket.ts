import { Construct } from "constructs";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as s3Deploy from "aws-cdk-lib/aws-s3-deployment";
import * as cloudfront from "aws-cdk-lib/aws-cloudfront";
import { RemovalPolicy } from "aws-cdk-lib";

type ProfileBucketProps = {
  bucketName: string;
};

type ProfileBucketDeploymentProps = {
  distribution: cloudfront.IDistribution;
};

export class ProfileBucket extends Construct {
  bucket: s3.Bucket;
  constructor(scope: Construct, props: ProfileBucketProps) {
    super(scope, "profile-bucket");
    this.bucket = new s3.Bucket(this, "profile-bucket", {
      removalPolicy: RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
      bucketName: props.bucketName,
    });
  }

  deploy(scope: Construct, props: ProfileBucketDeploymentProps) {
    const bucketDeployment = new s3Deploy.BucketDeployment(
      scope,
      "profile-bucket-deployment",
      {
        destinationBucket: this.bucket,
        sources: [s3Deploy.Source.asset("../.output/public")],
        distribution: props.distribution
      }
    );
  }
}