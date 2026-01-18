variable "domain_name" {
  description = "Domain name for email services"
  type        = string
  default     = "jimmillerdrums.com"
}

variable "aws_region" {
  description = "AWS region"
  type        = string
  default     = "us-east-1"
}

variable "environment" {
  description = "Environment name"
  type        = string
  default     = "prod"
}

variable "email_general_prefix" {
  description = "Bucket prefix to store general email (human-readable)"
  type        = string
  default     = "incoming"
}

variable "email_reports_prefix" {
  description = "Bucket prefix to store email reports (e.g. dmarc)"
  type        = string
  default     = "reports"
}

variable "forward_to_email" {
  description = "Gmail address to forward emails to"
  type        = string
  # This will be set via terraform.tfvars or environment variable
}

variable "project_name" {
  description = "Project name for resource naming"
  type        = string
  default     = "jimmillerdrums-email"
}

variable "critical_alarm_email" {
  description = "Email address for critical alarms (P1)"
  type        = string
  default     = "alerts-critical@jimmillerdrums.com"
}

variable "warning_alarm_email" {
  description = "Email address for warning alarms (P2)"
  type        = string
  default     = "alerts-warning@jimmillerdrums.com"
}

variable "info_alarm_email" {
  description = "Email address for informational alarms (P3)"
  type        = string
  default     = "alerts-info@jimmillerdrums.com"
}
