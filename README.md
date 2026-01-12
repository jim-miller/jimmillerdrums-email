# Email Infrastructure for jimmillerdrums.com

Professional email infrastructure using AWS SES, Lambda, and S3 with smart routing and Gmail integration.

## 🏗 Architecture

```mermaid
graph TD
    A[Email to @jimmillerdrums.com] --> B[AWS SES]
    B --> C[S3 Bucket Storage]
    B --> D[Lambda Function]
    D --> E[Smart Routing Logic]
    E --> F[Forward to Gmail]
    G[Gmail SMTP] --> H[Send from Custom Domain]
    
    I[CloudWatch Logs] --> D
    J[CloudWatch Alarms] --> D
```

## ✨ Features

- **Smart Email Routing**: Automatically forwards emails based on recipient address
- **Secure Storage**: Encrypted S3 storage with lifecycle policies
- **Spam Protection**: AWS SES built-in spam and virus scanning
- **DKIM Authentication**: Configured for email authenticity and deliverability
- **Monitoring**: CloudWatch logs and alarms for system health
- **Cost Effective**: ~$0.10 per 1,000 emails after free tier (3,000 emails/month free)
- **Gmail Integration**: Send emails from custom domain via Gmail SMTP

## 🚀 Quick Start

### Prerequisites

- AWS CLI configured with appropriate permissions
- OpenTofu installed
- Rust/Cargo installed (for Lambda function)
- Domain hosted on Route53 (jimmillerdrums.com)

### Deployment

1. **Clone and configure**:
   ```bash
   cd /path/to/jimmillerdrums-email
   cp opentofu.tfvars.example opentofu.tfvars
   # Edit opentofu.tfvars with your Gmail address
   ```

2. **Deploy infrastructure**:
   ```bash
   ./deploy.sh
   ```

3. **Configure Gmail** (see [Gmail Integration Guide](GMAIL_INTEGRATION.md)):
   - Enable 2FA on Google account
   - Generate app password
   - Configure Gmail SMTP settings

## 📧 Email Flow

### Incoming Emails
```
contact@jimmillerdrums.com
    ↓
AWS SES (spam filtering)
    ↓
S3 Storage (encrypted)
    ↓
Lambda Processing
    ↓
Forward to Gmail
```

### Outgoing Emails
```
Gmail Compose
    ↓
Select custom domain
    ↓
Gmail SMTP
    ↓
Delivered from @jimmillerdrums.com
```

## 🛠 Management Commands

```bash
# Deploy infrastructure
./deploy.sh deploy

# Check system status
./deploy.sh status

# View infrastructure outputs
./deploy.sh outputs

# View recent logs
./deploy.sh logs

# Test email functionality
./deploy.sh test

# Destroy infrastructure
./deploy.sh destroy
```

## 📊 Monitoring

- **CloudWatch Logs**: `/aws/lambda/jimmillerdrums-email-processor`
- **Error Alarms**: Triggers on Lambda function errors
- **Duration Alarms**: Triggers on slow processing (>30s)
- **Retention**: 14 days log retention

## 🔒 Security

- ✅ **S3 Encryption**: AES256 server-side encryption
- ✅ **Private Bucket**: No public access allowed
- ✅ **DKIM**: Domain authentication configured
- ✅ **IAM**: Least privilege access policies
- ✅ **Spam Filtering**: AWS SES built-in protection
- ✅ **Lifecycle**: Automatic email deletion after 90 days

## 💰 Cost Breakdown

- **AWS SES**: $0.10 per 1,000 emails (after 3,000 free/month)
- **Lambda**: Free for first 1M requests/month
- **S3**: ~$0.01/month for email storage
- **CloudWatch**: Basic monitoring included in free tier
- **Total**: ~$0.50-2.00/month for typical usage

## 📁 Project Structure

```
jimmillerdrums-email/
├── README.md                 # This file
├── GMAIL_INTEGRATION.md      # Gmail setup guide
├── deploy.sh                 # Deployment script
├── build.sh                  # Lambda build script
├── *.tf                      # OpenTofu configuration files
├── opentofu.tfvars.example   # Configuration template
└── lambda/                   # Rust Lambda function
    ├── src/main.rs           # Smart routing logic
    ├── Cargo.toml            # Rust dependencies
    └── README.md             # Lambda documentation
```

## 🧪 Testing

1. **Send test email**: Send to `test@jimmillerdrums.com`
2. **Check Gmail**: Verify forwarded email received
3. **Check logs**: `./deploy.sh logs`
4. **Check S3**: Verify email stored in bucket

## 🔧 Troubleshooting

### Common Issues

1. **Emails not forwarding**:
   - Check Lambda logs: `./deploy.sh logs`
   - Verify SES receipt rule is active
   - Check S3 bucket permissions

2. **Gmail SMTP issues**:
   - Verify app password is correct
   - Ensure 2FA is enabled
   - Check SMTP settings (smtp.gmail.com:587)

3. **Domain verification**:
   - Check DNS records in Route53
   - Verify DKIM tokens are correct
   - Allow up to 72 hours for propagation

### Getting Help

1. Check CloudWatch logs for errors
2. Verify SES domain status in AWS Console
3. Test email delivery using SES console
4. Review [Gmail Integration Guide](GMAIL_INTEGRATION.md)

## 📚 Documentation

- [Gmail Integration Setup](GMAIL_INTEGRATION.md) - Complete Gmail configuration guide
- [Lambda Function README](lambda/README.md) - Technical details about email processing
- [OpenTofu Configuration](*.tf) - Infrastructure as Code definitions

## 🎯 Next Steps

1. Test the complete email flow
2. Configure Gmail SMTP for sending
3. Set up email aliases for different purposes
4. Monitor system performance
5. Consider adding SNS notifications for alerts

---

**Note**: This infrastructure is designed for professional email handling with security, monitoring, and cost optimization in mind. The smart routing logic can be customized in the Lambda function to handle different email addresses differently.
