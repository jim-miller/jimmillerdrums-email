# Quick Deployment Guide - Rust Lambda

## Prerequisites
- Rust 1.88+ installed
- `cargo-lambda` installed: `cargo install cargo-lambda`
- OpenTofu/Terraform configured
- AWS credentials configured

## Deploy in 3 Steps

### 1. Build the Lambda
```bash
cd rust-lambda
cargo lambda build --release --arm64
```

### 2. Deploy Infrastructure
```bash
cd ../infra
tofu apply
```

### 3. Verify
```bash
# Check Lambda logs
aws logs tail /aws/lambda/jimmillerdrums-email-processor --follow

# Send test email to contact@jimmillerdrums.com
```

## Or Use the Deployment Script

```bash
./deploy-rust.sh
```

## Local Testing

```bash
cd rust-lambda

# Run unit tests
cargo test

# Run with test event (requires AWS credentials)
cargo lambda invoke --data-file test-event.json
```

## Rollback to JavaScript

If you need to rollback:

```bash
cd infra
git checkout lambda.tf
tofu apply
```

## Monitoring

After deployment, monitor:
- CloudWatch Logs: `/aws/lambda/jimmillerdrums-email-processor`
- Lambda Metrics: Duration, Errors, Throttles
- SES Metrics: Bounce rate, Complaint rate

## Troubleshooting

### Build fails
```bash
# Update Rust
rustup update stable

# Clean and rebuild
cargo clean
cargo lambda build --release --arm64
```

### Lambda fails to invoke
- Check CloudWatch logs for errors
- Verify environment variables are set (EMAIL_BUCKET, FORWARD_TO_EMAIL)
- Ensure IAM role has S3 and SES permissions

### Email not forwarding
- Check S3 bucket for incoming email
- Verify SES receipt rule is active
- Check Lambda execution logs
