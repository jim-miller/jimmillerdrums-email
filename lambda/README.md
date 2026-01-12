# Email Processor Lambda Function

A Rust-based AWS Lambda function for processing and routing emails received via Amazon SES.

## Architecture

This Lambda function implements smart email routing:
- **booking@/contact@/hello@/support@** → Forward to Gmail
- **info@** → Separate handling (currently also forwarded with [INFO] prefix)
- All emails are archived in S3 before processing

## Build & Deploy

### Prerequisites

```bash
# Install cargo-lambda
cargo install cargo-lambda

# Add the Lambda target
rustup target add x86_64-unknown-linux-gnu
```

### Build Command

```bash
cargo lambda build --release --target x86_64-unknown-linux-gnu
```

### Environment Variables

The Lambda function requires these environment variables:

- `EMAIL_BUCKET`: S3 bucket name where SES stores incoming emails
- `FORWARD_TO_EMAIL`: Gmail address to forward emails to

## Required IAM Permissions

The Lambda execution role needs these permissions:

```json
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Action": [
                "logs:CreateLogGroup",
                "logs:CreateLogStream",
                "logs:PutLogEvents"
            ],
            "Resource": "arn:aws:logs:*:*:*"
        },
        {
            "Effect": "Allow",
            "Action": [
                "s3:GetObject"
            ],
            "Resource": "arn:aws:s3:::YOUR_EMAIL_BUCKET/*"
        },
        {
            "Effect": "Allow",
            "Action": [
                "ses:SendEmail",
                "ses:SendRawEmail"
            ],
            "Resource": "*"
        }
    ]
}
```

## Lambda Configuration

- **Runtime**: `provided.al2023`
- **Handler**: `bootstrap`
- **Memory**: 256 MB (recommended)
- **Timeout**: 60 seconds
- **Architecture**: x86_64

## Features

- **Zero-cost abstractions**: Uses Rust's type system for compile-time safety
- **Structured logging**: Compatible with CloudWatch using `tracing`
- **Proper error handling**: Uses `anyhow` for context-rich error propagation
- **Memory safety**: No `unsafe` blocks, leverages Rust's ownership system
- **Performance optimized**: AWS clients initialized globally for execution environment reuse

## Email Processing Flow

1. SES receives email and stores in S3
2. SES triggers Lambda function
3. Lambda retrieves email from S3
4. Email headers are modified for forwarding:
   - Original sender becomes Reply-To
   - From header uses verified SES domain
   - Forwarding headers added for tracking
5. Email forwarded via SES to configured Gmail address

## Development

Run Clippy for linting:
```bash
cargo clippy -- -D warnings
```

Run tests:
```bash
cargo test
```

Format code:
```bash
cargo fmt
```
