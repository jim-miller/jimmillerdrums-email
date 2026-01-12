# JimMillerDrums Email Infrastructure

Email infrastructure for jimmillerdrums.com using AWS SES + Lambda for cost-effective email receiving and smart routing.

## Architecture

- **AWS SES**: Domain verification and email receiving
- **Lambda**: Smart email routing (booking@/contact@ → Gmail, info@ → separate handling)
- **S3**: Email storage and archival
- **Route53**: DNS records for email (MX, TXT verification)
- **CloudWatch**: Monitoring and logging

## Cost Structure

- AWS Free Tier: 3,000 emails/month free for 12 months
- After free tier: $0.10 per 1,000 emails
- Lambda: ~$0.20 per 1M requests
- S3: ~$0.023 per GB/month

## Deployment

```bash
# Initialize and plan
tofu init
tofu plan

# Apply infrastructure
tofu apply

# Destroy (if needed)
tofu destroy
```

## Email Addresses

- `booking@jimmillerdrums.com` → Forwards to Gmail
- `contact@jimmillerdrums.com` → Forwards to Gmail  
- `info@jimmillerdrums.com` → Separate inbox (future: WorkMail or IMAP)
- `*@jimmillerdrums.com` → Catch-all forwarding to Gmail
