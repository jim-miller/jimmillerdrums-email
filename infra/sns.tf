# SNS Topics for Alarm Notifications by Severity
resource "aws_sns_topic" "critical_alarms" {
  name         = "${var.project_name}-critical-alarms"
  display_name = "[P1-CRITICAL] ${var.project_name} Email System"

  tags = {
    Environment = var.environment
    Project     = var.project_name
    Severity    = "Critical"
    ManagedBy   = "opentofu"
  }
}

resource "aws_sns_topic" "warning_alarms" {
  name         = "${var.project_name}-warning-alarms"
  display_name = "[P2-WARNING] ${var.project_name} Email System"

  tags = {
    Environment = var.environment
    Project     = var.project_name
    Severity    = "Warning"
    ManagedBy   = "opentofu"
  }
}

resource "aws_sns_topic" "info_alarms" {
  name         = "${var.project_name}-info-alarms"
  display_name = "[P3-INFO] ${var.project_name} Email System"

  tags = {
    Environment = var.environment
    Project     = var.project_name
    Severity    = "Info"
    ManagedBy   = "opentofu"
  }
}

# SNS Topic Subscriptions
resource "aws_sns_topic_subscription" "critical_email" {
  topic_arn = aws_sns_topic.critical_alarms.arn
  protocol  = "email"
  endpoint  = var.critical_alarm_email
}

resource "aws_sns_topic_subscription" "warning_email" {
  topic_arn = aws_sns_topic.warning_alarms.arn
  protocol  = "email"
  endpoint  = var.warning_alarm_email
}

resource "aws_sns_topic_subscription" "info_email" {
  topic_arn = aws_sns_topic.info_alarms.arn
  protocol  = "email"
  endpoint  = var.info_alarm_email
}
