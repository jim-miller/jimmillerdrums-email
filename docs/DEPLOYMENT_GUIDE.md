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

## Environment Variables

The Lambda requires these environment variables (set in `infra/lambda.tf`):

- `EMAIL_BUCKET`: S3 bucket for email storage
- `INCOMING_PREFIX`: S3 prefix for incoming emails (e.g., "incoming" or "reports/dmarc")
- `FORWARD_TO_EMAIL`: Email address to forward messages to

## Local Testing

```bash
cd rust-lambda

# Run all tests (unit + integration + AWS mocking)
cargo test

# Run specific test categories
cargo test config_tests      # Configuration tests
cargo test aws_integration   # AWS SDK mocking tests
cargo test integration_tests # Email parsing tests

# Run with test event (requires AWS credentials)
cargo lambda invoke --data-file test-event.json
```

## Testing Architecture

The project uses a clean testing approach:

- **Unit Tests**: Domain logic and configuration
- **AWS Integration Tests**: AWS SDK mocking with `aws-smithy-mocks`
- **Config Tests**: Dependency injection without environment variables
- **No Global State**: Tests run in parallel without interference

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
- Verify environment variables are set (EMAIL_BUCKET, INCOMING_PREFIX, FORWARD_TO_EMAIL)
- Ensure IAM role has S3 and SES permissions

### Email not forwarding
- Check S3 bucket for incoming email
- Verify SES receipt rule is active
- Check Lambda execution logs

### Configuration Issues
- Environment variables loaded once at startup
- Configuration errors fail fast during Lambda initialization
- Check CloudWatch logs for "Configuration error" messages
