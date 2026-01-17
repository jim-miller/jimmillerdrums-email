# Product Overview

## Purpose
Professional email infrastructure for jimmillerdrums.com that receives emails via AWS SES, stores them in S3, and forwards them to Gmail using a Lambda function.

## Key Features
- **Email Reception**: AWS SES receives emails sent to @jimmillerdrums.com addresses
- **Secure Storage**: Emails stored in S3 with encryption and 90-day lifecycle
- **Smart Forwarding**: Lambda function processes and forwards emails to Gmail
- **Monitoring**: CloudWatch alarms with severity-based routing (P1-Critical, P2-Warning, P3-Info)
- **Cost Effective**: ~$0.50-2.00/month for typical usage

## Target Users
Single domain owner (jimmillerdrums.com) managing professional email through Gmail integration.

## Business Objectives
- Maintain professional email presence with custom domain
- Minimize operational overhead through automation
- Keep costs low while ensuring reliability
- Monitor system health proactively
