# Create IAM user for SMTP credentials
resource "aws_iam_user" "ses_smtp_user" {
  name = "jimmillerdrums-ses-smtp"
  path = "/"
}

# Create SMTP credentials (access key)
resource "aws_iam_access_key" "ses_smtp_key" {
  user = aws_iam_user.ses_smtp_user.name
}

# Policy for sending emails via SES
resource "aws_iam_user_policy" "ses_smtp_policy" {
  name = "ses-smtp-policy"
  user = aws_iam_user.ses_smtp_user.name

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "ses:SendEmail",
          "ses:SendRawEmail"
        ]
        Resource = "*"
      }
    ]
  })
}

# Output SMTP credentials (sensitive)
output "ses_smtp_username" {
  value = aws_iam_access_key.ses_smtp_key.id
  description = "SMTP username for Gmail configuration"
}

output "ses_smtp_password" {
  value = aws_iam_access_key.ses_smtp_key.ses_smtp_password_v4
  sensitive = true
  description = "SMTP password for Gmail configuration (sensitive)"
}

output "ses_smtp_server" {
  value = "email-smtp.us-east-1.amazonaws.com"
  description = "SES SMTP server for Gmail configuration"
}
