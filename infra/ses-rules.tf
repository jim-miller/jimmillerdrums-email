# SES Receipt Rule Set and Rules Configuration

# Create the receipt rule set
resource "aws_ses_receipt_rule_set" "main" {
  rule_set_name = "${var.project_name}-rules"
}

# Set the active rule set
resource "aws_ses_active_receipt_rule_set" "main" {
  rule_set_name = aws_ses_receipt_rule_set.main.rule_set_name
}

# Specific Rule for DMARC Reports (No Forwarding)
resource "aws_ses_receipt_rule" "dmarc_reports" {
  name          = "${var.project_name}-dmarc-reports"
  rule_set_name = aws_ses_receipt_rule_set.main.rule_set_name
  recipients    = ["dmarc@${var.domain_name}"]
  enabled       = true
  scan_enabled  = true

  after = null # Ensure this is evaluated in the correct order if needed

  # Store in a specific folder in S3 for organization
  s3_action {
    bucket_name       = aws_s3_bucket.email_storage.bucket
    object_key_prefix = "${var.email_reports_prefix}/dmarc/"
    position          = 1
  }
}

# Receipt rule for all emails to the domain
resource "aws_ses_receipt_rule" "email_processor" {
  name          = "${var.project_name}-email-processor"
  rule_set_name = aws_ses_receipt_rule_set.main.rule_set_name
  recipients    = [var.domain_name]
  enabled       = true
  scan_enabled  = true

  after = aws_ses_receipt_rule.dmarc_reports.name

  # Store email in S3 first
  s3_action {
    bucket_name       = aws_s3_bucket.email_storage.bucket
    object_key_prefix = "${var.email_general_prefix}/"
    position          = 1
  }

  # Then trigger Lambda for processing
  lambda_action {
    function_arn = aws_lambda_function.email_processor.arn
    position     = 2
  }

  depends_on = [
    aws_lambda_permission.ses_invoke,
    aws_s3_bucket_policy.email_storage
  ]
}

# Permission for SES to invoke Lambda
resource "aws_lambda_permission" "ses_invoke" {
  statement_id  = "AllowExecutionFromSES"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.email_processor.function_name
  principal     = "ses.amazonaws.com"
  source_arn    = "arn:aws:ses:${var.aws_region}:${data.aws_caller_identity.current.account_id}:receipt-rule-set/${aws_ses_receipt_rule_set.main.rule_set_name}:receipt-rule/${var.project_name}-email-processor"
}
