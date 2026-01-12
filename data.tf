# Reference to existing Route53 hosted zone from website infrastructure
data "aws_route53_zone" "main" {
  name         = var.domain_name
  private_zone = false
}

# Get current AWS region
data "aws_region" "current" {}

# Get current AWS caller identity
data "aws_caller_identity" "current" {}
