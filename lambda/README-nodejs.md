# Email Forwarder Lambda Function

AWS Lambda function that forwards emails received via Amazon SES to a specified email address.

## Features

- Retrieves emails from S3 bucket
- Parses MIME multipart emails
- Extracts clean text content
- Forwards via SES with proper authentication
- Preserves original sender in Reply-To header

## Environment Variables

- `EMAIL_BUCKET` - S3 bucket containing incoming emails
- `FORWARD_TO_EMAIL` - Destination email address for forwarding

## Runtime

- Node.js 20.x
- AWS SDK v2 (included in Lambda runtime)

## Handler

`index.handler`

## Deployment

Deployed automatically via OpenTofu in `../infra/lambda.tf`
