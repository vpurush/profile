import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { RustFunction } from "cargo-lambda-cdk";
import { FunctionUrl } from "aws-cdk-lib/aws-lambda";
import { CfnOutput } from "aws-cdk-lib";
// import * as sqs from 'aws-cdk-lib/aws-sqs';

export class ProfileLeptosStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // The code that defines your stack goes here
    const rustFunction = new RustFunction(this, "ProfileLeptosRustFunction", {
      manifestPath: "../Cargo.toml",
      bundling: {
        environment: {
          LEPTOS_OUTPUT_NAME: 'profile'
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

    const rustFunctionUrl = new FunctionUrl(
      this,
      "ProfileLeptosRustFunctionUrl",
      {
        function: rustFunction,
        authType: cdk.aws_lambda.FunctionUrlAuthType.NONE
      }
    );

    new CfnOutput(this, "ProfileLeptosRustFunctionUrlOutput", {
      value: rustFunctionUrl.url,
    });
  }
}
