# Archive Lambda function code
data "archive_file" "lambda_zip" {
  type        = "zip"
  source_file = "${path.module}/../rust-lambda/target/lambda/email-processor/bootstrap"
  output_path = "${path.module}/.terraform/lambda.zip"
}

# Lambda Function
resource "aws_lambda_function" "email_processor" {
  filename      = data.archive_file.lambda_zip.output_path
  function_name = "${var.project_name}-processor"
  role          = aws_iam_role.lambda_email_processor.arn
  handler       = "bootstrap"
  runtime       = "provided.al2023"
  timeout       = 60
  memory_size   = 256
  architectures = ["arm64"]

  source_code_hash = data.archive_file.lambda_zip.output_base64sha256

  environment {
    variables = {
      EMAIL_BUCKET      = aws_s3_bucket.email_storage.bucket
      INCOMING_PREFIX   = var.email_general_prefix
      FORWARD_TO_EMAIL  = var.forward_to_email
      MAX_EMAIL_SIZE_MB = var.max_email_size_mb
      RUST_LOG          = var.log_level
    }
  }

  depends_on = [
    aws_iam_role_policy_attachment.lambda_basic_execution,
    aws_iam_role_policy.lambda_s3_access,
    aws_iam_role_policy.lambda_ses_access,
  ]

  tags = {
    Name = "${var.project_name}-email-processor"
  }
}


