import * as cdk from 'aws-cdk-lib';
import { CfnOutput } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { ProfileBucket } from './profile-bucket';
import { ProfileDistribution } from './profile-distribution';
// import * as sqs from 'aws-cdk-lib/aws-sqs';

export class ProfileStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const rootDomainName = "vpurush-staging.com";
    const profileSubDomainName = "profile.vpurush-staging.com";

    const bucket = new ProfileBucket(this, {
      bucketName: rootDomainName,
    });

    const distribution = new ProfileDistribution(this, {
      // domainName: rootDomainName,
      // profileSubDomainName: profileSubDomainName,
      bucket: bucket.bucket,
      // certificate: certificate.certificate,
    });

    bucket.deploy(this, {
      distribution: distribution.distribution,
    });

    new CfnOutput(this, "profile-distribution-url-output", {value: distribution.distribution.distributionDomainName});
  }
}
