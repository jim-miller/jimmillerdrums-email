# Technology Stack

## Infrastructure
- **IaC**: OpenTofu (Terraform fork)
- **Cloud Provider**: AWS (us-east-1 region)
- **Domain**: Route53-hosted (jimmillerdrums.com)

## AWS Services
- **SES**: Email reception with DKIM authentication
- **S3**: Encrypted email storage with lifecycle policies
- **Lambda**: Email processing and forwarding (Node.js 20.x runtime)
- **CloudWatch**: Logs, alarms, and monitoring
- **SNS**: Severity-based alarm notifications

## Lambda Runtime
- **Runtime**: Node.js 20.x
- **SDK**: AWS SDK v3 (pre-installed in runtime, not bundled)
- **Deployment**: Minimal package (index.js + package.json only, ~1.8KB)

## Key Constraints
- Node.js 18+ runtimes do NOT include AWS SDK v2 (aws-sdk package)
- AWS SDK v3 is pre-installed in Node.js 18+ runtimes
- Lambda packages should NOT bundle AWS SDK v3 to avoid 250MB unzipped limit
- Use modular imports: `@aws-sdk/client-ses`, `@aws-sdk/client-s3`
