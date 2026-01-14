# Verify jim@jimmillerdrums.com as a sender identity
resource "aws_ses_email_identity" "jim_email" {
  email = "jim@jimmillerdrums.com"
}

# Output the email identity ARN
output "jim_email_identity_arn" {
  value = aws_ses_email_identity.jim_email.arn
  description = "ARN for jim@jimmillerdrums.com email identity"
}
