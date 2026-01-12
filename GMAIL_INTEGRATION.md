# Gmail Integration Setup

This document explains how to configure Gmail to send emails from your custom domain (jimmillerdrums.com) using AWS SES.

## Overview

The email infrastructure is now set up to:
1. **Receive emails**: AWS SES receives emails sent to any address @jimmillerdrums.com
2. **Store emails**: Incoming emails are stored in S3 bucket with encryption
3. **Process emails**: Lambda function processes emails and forwards them to your Gmail
4. **Send emails**: You can configure Gmail to send emails from your custom domain

## Current Status

✅ **Domain Verification**: jimmillerdrums.com is verified with AWS SES  
✅ **DKIM Authentication**: Configured for email authenticity  
✅ **MX Records**: Configured to route emails to AWS SES  
✅ **Email Processing**: Lambda function processes and forwards emails  
✅ **Monitoring**: CloudWatch logs and alarms configured  

## Gmail SMTP Configuration

To send emails from Gmail using your custom domain:

### Step 1: Enable 2-Factor Authentication
1. Go to your Google Account settings
2. Enable 2-Factor Authentication if not already enabled

### Step 2: Generate App Password
1. Go to Google Account > Security > 2-Step Verification
2. Click "App passwords" at the bottom
3. Generate a new app password for "Mail"
4. Save this password securely

### Step 3: Configure Gmail
1. Open Gmail > Settings (gear icon) > "See all settings"
2. Go to "Accounts and Import" tab
3. Click "Add another email address" in "Send mail as" section
4. Enter:
   - **Name**: Your name (e.g., "Jim Miller")
   - **Email**: your-email@jimmillerdrums.com
   - **Treat as alias**: ✅ (checked)
5. Click "Next Step"
6. Configure SMTP settings:
   - **SMTP Server**: smtp.gmail.com
   - **Port**: 587
   - **Username**: your-gmail-address@gmail.com
   - **Password**: The app password from Step 2
   - **TLS**: ✅ (checked)
7. Click "Add Account"
8. Verify the email address using the verification code

### Step 4: Set as Default (Optional)
1. In Gmail settings > "Accounts and Import"
2. Click "make default" next to your custom domain email
3. Now Gmail will send from your custom domain by default

## Email Flow

### Incoming Emails
```
Email sent to contact@jimmillerdrums.com
    ↓
AWS SES receives email
    ↓
Email stored in S3 bucket (encrypted)
    ↓
Lambda function triggered
    ↓
Email forwarded to miller.jimd@gmail.com
```

### Outgoing Emails
```
Compose email in Gmail
    ↓
Select "From: your-email@jimmillerdrums.com"
    ↓
Gmail sends via SMTP using your credentials
    ↓
Recipient sees email from jimmillerdrums.com
```

## Testing the Setup

### Test Incoming Email
1. Send an email to any address @jimmillerdrums.com (e.g., test@jimmillerdrums.com)
2. Check your Gmail inbox for the forwarded email
3. Check CloudWatch logs: `/aws/lambda/jimmillerdrums-email-processor`
4. Check S3 bucket: `jimmillerdrums-email-emails-77808605` for stored email

### Test Outgoing Email
1. Compose a new email in Gmail
2. Click the "From" field and select your custom domain
3. Send the email
4. Verify the recipient sees it from your custom domain

## Monitoring and Troubleshooting

### CloudWatch Logs
- **Log Group**: `/aws/lambda/jimmillerdrums-email-processor`
- **Retention**: 14 days
- **View logs**: AWS Console > CloudWatch > Log groups

### CloudWatch Alarms
- **Lambda Errors**: Triggers if Lambda function errors occur
- **Lambda Duration**: Triggers if function takes longer than 30 seconds

### Common Issues

1. **Emails not forwarding**:
   - Check Lambda function logs in CloudWatch
   - Verify SES receipt rule is active
   - Check S3 bucket permissions

2. **Gmail SMTP not working**:
   - Verify app password is correct
   - Ensure 2FA is enabled on Google account
   - Check SMTP settings (server: smtp.gmail.com, port: 587, TLS enabled)

3. **Emails marked as spam**:
   - DKIM is configured to help with this
   - Recipients may need to mark as "not spam" initially
   - Consider adding SPF record for additional authentication

## Cost Optimization

- **AWS SES**: First 3,000 emails/month are free, then $0.10 per 1,000 emails
- **Lambda**: First 1M requests/month are free
- **S3**: Minimal cost for email storage
- **CloudWatch**: Basic monitoring included in free tier

## Security Features

- ✅ **Encryption**: S3 bucket uses AES256 encryption
- ✅ **DKIM**: Domain authentication configured
- ✅ **Private S3**: Bucket blocks all public access
- ✅ **IAM**: Least privilege access for Lambda function
- ✅ **Spam Filtering**: SES scans incoming emails
- ✅ **Lifecycle Policy**: Emails automatically deleted after 90 days

## Next Steps

1. **Test the email flow** by sending test emails
2. **Configure Gmail SMTP** following the steps above
3. **Set up email aliases** for different purposes (contact@, booking@, info@)
4. **Monitor the system** using CloudWatch logs and alarms
5. **Consider adding SNS notifications** for critical alerts

## Support

For issues with this setup:
1. Check CloudWatch logs for Lambda function errors
2. Verify SES domain and DKIM status in AWS Console
3. Test email delivery using AWS SES console
4. Review this documentation for configuration steps
