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
