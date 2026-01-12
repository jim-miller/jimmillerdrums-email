output "domain_name" {
  description = "Domain name configured for email"
  value       = var.domain_name
}

output "ses_domain_identity_arn" {
  description = "ARN of the SES domain identity"
  value       = aws_ses_domain_identity.main.arn
}

output "ses_verification_token" {
  description = "SES domain verification token"
  value       = aws_ses_domain_identity.main.verification_token
}

output "ses_dkim_tokens" {
  description = "SES DKIM tokens for domain authentication"
  value       = aws_ses_domain_dkim.main.dkim_tokens
}

output "email_bucket_name" {
  description = "Name of the S3 bucket storing emails"
  value       = aws_s3_bucket.email_storage.bucket
}

output "lambda_role_arn" {
  description = "ARN of the Lambda execution role"
  value       = aws_iam_role.lambda_email_processor.arn
}

output "lambda_function_name" {
  description = "Name of the email processing Lambda function"
  value       = aws_lambda_function.email_processor.function_name
}

output "lambda_function_arn" {
  description = "ARN of the email processing Lambda function"
  value       = aws_lambda_function.email_processor.arn
}

# SES Receipt Rules Outputs
output "ses_rule_set_name" {
  description = "Name of the SES receipt rule set"
  value       = aws_ses_receipt_rule_set.main.rule_set_name
}

output "ses_receipt_rule_name" {
  description = "Name of the SES receipt rule"
  value       = aws_ses_receipt_rule.email_processor.name
}

# Monitoring Outputs
output "cloudwatch_log_group_name" {
  description = "Name of the CloudWatch log group for Lambda"
  value       = aws_cloudwatch_log_group.lambda_logs.name
}

output "lambda_errors_alarm_name" {
  description = "Name of the Lambda errors CloudWatch alarm"
  value       = aws_cloudwatch_metric_alarm.lambda_errors.alarm_name
}

output "lambda_duration_alarm_name" {
  description = "Name of the Lambda duration CloudWatch alarm"
  value       = aws_cloudwatch_metric_alarm.lambda_duration.alarm_name
}
