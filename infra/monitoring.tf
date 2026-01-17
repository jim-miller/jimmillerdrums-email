# CloudWatch Log Group for Lambda Function
resource "aws_cloudwatch_log_group" "lambda_logs" {
  name              = "/aws/lambda/${aws_lambda_function.email_processor.function_name}"
  retention_in_days = 14
  
  tags = {
    Environment = var.environment
    Project     = var.project_name
    ManagedBy   = "opentofu"
  }
}

# ============================================================================
# CRITICAL ALARMS (P1) - Immediate Action Required
# ============================================================================

resource "aws_cloudwatch_metric_alarm" "lambda_errors_critical" {
  alarm_name          = "[P1-CRITICAL] ${var.project_name} - Lambda Function Errors"
  alarm_description   = "Email forwarding Lambda is failing. Emails are NOT being forwarded. Check CloudWatch logs immediately."
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 2
  threshold           = 0
  treat_missing_data  = "notBreaching"
  
  metric_name = "Errors"
  namespace   = "AWS/Lambda"
  period      = 300
  statistic   = "Sum"
  
  dimensions = {
    FunctionName = aws_lambda_function.email_processor.function_name
  }
  
  alarm_actions = [aws_sns_topic.critical_alarms.arn]
  ok_actions    = [aws_sns_topic.critical_alarms.arn]
  
  tags = {
    Environment = var.environment
    Project     = var.project_name
    Severity    = "Critical"
    ManagedBy   = "opentofu"
  }
}

resource "aws_cloudwatch_metric_alarm" "ses_reputation_bounce_rate" {
  alarm_name          = "[P1-CRITICAL] ${var.project_name} - SES Bounce Rate High"
  alarm_description   = "SES bounce rate exceeds 5%. Account may be suspended. Review bounced emails and remove invalid addresses."
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 1
  threshold           = 0.05
  treat_missing_data  = "notBreaching"
  
  metric_name = "Reputation.BounceRate"
  namespace   = "AWS/SES"
  period      = 900
  statistic   = "Average"
  
  alarm_actions = [aws_sns_topic.critical_alarms.arn]
  ok_actions    = [aws_sns_topic.critical_alarms.arn]
  
  tags = {
    Environment = var.environment
    Project     = var.project_name
    Severity    = "Critical"
    ManagedBy   = "opentofu"
  }
}

resource "aws_cloudwatch_metric_alarm" "ses_reputation_complaint_rate" {
  alarm_name          = "[P1-CRITICAL] ${var.project_name} - SES Complaint Rate High"
  alarm_description   = "SES complaint rate exceeds 0.1%. Account may be suspended. Review spam complaints immediately."
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 1
  threshold           = 0.001
  treat_missing_data  = "notBreaching"
  
  metric_name = "Reputation.ComplaintRate"
  namespace   = "AWS/SES"
  period      = 900
  statistic   = "Average"
  
  alarm_actions = [aws_sns_topic.critical_alarms.arn]
  ok_actions    = [aws_sns_topic.critical_alarms.arn]
  
  tags = {
    Environment = var.environment
    Project     = var.project_name
    Severity    = "Critical"
    ManagedBy   = "opentofu"
  }
}

resource "aws_cloudwatch_metric_alarm" "lambda_throttles" {
  alarm_name          = "[P1-CRITICAL] ${var.project_name} - Lambda Throttling"
  alarm_description   = "Lambda function is being throttled. Emails may be delayed or lost. Check concurrency limits."
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 2
  threshold           = 0
  treat_missing_data  = "notBreaching"
  
  metric_name = "Throttles"
  namespace   = "AWS/Lambda"
  period      = 300
  statistic   = "Sum"
  
  dimensions = {
    FunctionName = aws_lambda_function.email_processor.function_name
  }
  
  alarm_actions = [aws_sns_topic.critical_alarms.arn]
  ok_actions    = [aws_sns_topic.critical_alarms.arn]
  
  tags = {
    Environment = var.environment
    Project     = var.project_name
    Severity    = "Critical"
    ManagedBy   = "opentofu"
  }
}

# ============================================================================
# WARNING ALARMS (P2) - Action Required Within 24 Hours
# ============================================================================

resource "aws_cloudwatch_metric_alarm" "lambda_duration_warning" {
  alarm_name          = "[P2-WARNING] ${var.project_name} - Lambda Slow Performance"
  alarm_description   = "Lambda execution time exceeds 10 seconds. Performance degradation detected. Review function efficiency."
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 3
  datapoints_to_alarm = 2
  threshold           = 10000
  treat_missing_data  = "notBreaching"
  
  metric_name = "Duration"
  namespace   = "AWS/Lambda"
  period      = 300
  statistic   = "Average"
  
  dimensions = {
    FunctionName = aws_lambda_function.email_processor.function_name
  }
  
  alarm_actions = [aws_sns_topic.warning_alarms.arn]
  ok_actions    = [aws_sns_topic.warning_alarms.arn]
  
  tags = {
    Environment = var.environment
    Project     = var.project_name
    Severity    = "Warning"
    ManagedBy   = "opentofu"
  }
}

resource "aws_cloudwatch_metric_alarm" "ses_send_failures" {
  alarm_name          = "[P2-WARNING] ${var.project_name} - SES Send Failures"
  alarm_description   = "SES is experiencing send failures. Some forwarded emails may not be delivered. Check SES status."
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 2
  threshold           = 5
  treat_missing_data  = "notBreaching"
  
  metric_name = "Send"
  namespace   = "AWS/SES"
  period      = 900
  statistic   = "Sum"
  
  alarm_actions = [aws_sns_topic.warning_alarms.arn]
  ok_actions    = [aws_sns_topic.warning_alarms.arn]
  
  tags = {
    Environment = var.environment
    Project     = var.project_name
    Severity    = "Warning"
    ManagedBy   = "opentofu"
  }
}

resource "aws_cloudwatch_metric_alarm" "lambda_concurrent_executions" {
  alarm_name          = "[P2-WARNING] ${var.project_name} - High Lambda Concurrency"
  alarm_description   = "Lambda concurrent executions approaching limits. May impact performance. Monitor for throttling."
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 2
  threshold           = 5
  treat_missing_data  = "notBreaching"
  
  metric_name = "ConcurrentExecutions"
  namespace   = "AWS/Lambda"
  period      = 300
  statistic   = "Maximum"
  
  dimensions = {
    FunctionName = aws_lambda_function.email_processor.function_name
  }
  
  alarm_actions = [aws_sns_topic.warning_alarms.arn]
  ok_actions    = [aws_sns_topic.warning_alarms.arn]
  
  tags = {
    Environment = var.environment
    Project     = var.project_name
    Severity    = "Warning"
    ManagedBy   = "opentofu"
  }
}

# ============================================================================
# INFO ALARMS (P3) - Informational, Review When Convenient
# ============================================================================

resource "aws_cloudwatch_metric_alarm" "ses_daily_volume_high" {
  alarm_name          = "[P3-INFO] ${var.project_name} - Unusual Email Volume"
  alarm_description   = "Email volume is higher than normal. This may be legitimate traffic or potential spam. Review if unexpected."
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 1
  threshold           = 100
  treat_missing_data  = "notBreaching"
  
  metric_name = "Send"
  namespace   = "AWS/SES"
  period      = 86400
  statistic   = "Sum"
  
  alarm_actions = [aws_sns_topic.info_alarms.arn]
  
  tags = {
    Environment = var.environment
    Project     = var.project_name
    Severity    = "Info"
    ManagedBy   = "opentofu"
  }
}

resource "aws_cloudwatch_metric_alarm" "lambda_invocations_low" {
  alarm_name          = "[P3-INFO] ${var.project_name} - No Email Activity"
  alarm_description   = "No emails received in 7 days. This may be normal or could indicate a configuration issue."
  comparison_operator = "LessThanThreshold"
  evaluation_periods  = 1
  threshold           = 1
  treat_missing_data  = "breaching"
  
  metric_name = "Invocations"
  namespace   = "AWS/Lambda"
  period      = 604800
  statistic   = "Sum"
  
  dimensions = {
    FunctionName = aws_lambda_function.email_processor.function_name
  }
  
  alarm_actions = [aws_sns_topic.info_alarms.arn]
  
  tags = {
    Environment = var.environment
    Project     = var.project_name
    Severity    = "Info"
    ManagedBy   = "opentofu"
  }
}

# ============================================================================
# COMPOSITE ALARM - Overall System Health
# ============================================================================
# Note: Composite alarms with special characters in names require careful escaping
# Monitoring individual critical alarms provides equivalent coverage

# Uncomment and adjust if composite alarm is needed:
# resource "aws_cloudwatch_composite_alarm" "email_system_health" {
#   alarm_name    = "COMPOSITE-email-system-unhealthy"
#   alarm_rule    = "ALARM(alarm-name-1) OR ALARM(alarm-name-2)"
#   alarm_actions = [aws_sns_topic.critical_alarms.arn]
# }


