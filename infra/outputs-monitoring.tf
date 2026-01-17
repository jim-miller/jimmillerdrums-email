output "sns_topic_critical_arn" {
  description = "ARN of the critical alarms SNS topic"
  value       = aws_sns_topic.critical_alarms.arn
}

output "sns_topic_warning_arn" {
  description = "ARN of the warning alarms SNS topic"
  value       = aws_sns_topic.warning_alarms.arn
}

output "sns_topic_info_arn" {
  description = "ARN of the info alarms SNS topic"
  value       = aws_sns_topic.info_alarms.arn
}

output "alarm_summary" {
  description = "Summary of configured alarms by severity"
  value = {
    critical = [
      "Lambda Function Errors",
      "SES Bounce Rate High",
      "SES Complaint Rate High",
      "Lambda Throttling",
      "Email System Health (Composite)"
    ]
    warning = [
      "Lambda Slow Performance",
      "SES Send Failures",
      "High Lambda Concurrency"
    ]
    info = [
      "Unusual Email Volume",
      "No Email Activity"
    ]
  }
}
