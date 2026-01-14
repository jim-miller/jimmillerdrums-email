# Archive Lambda function code (temporary Node.js version)
data "archive_file" "lambda_zip" {
  type        = "zip"
  source_file = "${path.module}/lambda-temp.js"
  output_path = "${path.module}/lambda.zip"
}

# Lambda Function
resource "aws_lambda_function" "email_processor" {
  filename         = data.archive_file.lambda_zip.output_path
  function_name    = "${var.project_name}-processor"
  role            = aws_iam_role.lambda_email_processor.arn
  handler         = "lambda-temp.handler"
  runtime         = "nodejs16.x"
  timeout         = 60
  memory_size     = 256
  architectures    = ["x86_64"]

  source_code_hash = data.archive_file.lambda_zip.output_base64sha256

  environment {
    variables = {
      EMAIL_BUCKET      = aws_s3_bucket.email_storage.bucket
      FORWARD_TO_EMAIL  = var.forward_to_email
    }
  }

  depends_on = [
    aws_iam_role_policy_attachment.lambda_basic_execution,
    aws_iam_role_policy.lambda_s3_access,
    aws_iam_role_policy.lambda_ses_access,
  ]
}


