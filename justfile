set dotenv-load := true

# Build the Rust Lambda for ARM64 (Graviton)
build:
    cd rust-lambda && cargo lambda build --release --arm64

# Preview infrastructure changes
plan:
    cd infra && tofu plan -var-file="opentofu.tfvars"

# Build code and apply infrastructure
deploy: build
    cd infra && tofu apply -var-file="opentofu.tfvars" -auto-approve

# Tail the Lambda logs in real-time
logs:
    aws logs tail "/aws/lambda/email_processor" --follow --format short

# Clean up AWS SES setup notifications from S3
clean-setup:
    aws s3 rm s3://$EMAIL_BUCKET/ --recursive --exclude "*" --include "AMAZON_SES_SETUP_NOTIFICATION*"

# Run Rust tests
test:
    cd rust-lambda && cargo test

