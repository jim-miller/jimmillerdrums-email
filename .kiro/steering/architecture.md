# System Architecture

## Email Flow

### Incoming Email Processing
```
Email to @jimmillerdrums.com
    ↓
AWS SES (spam/virus filtering, DKIM verification)
    ↓
S3 Bucket (encrypted storage, 90-day lifecycle)
    ↓
Lambda Trigger (Node.js 20.x)
    ↓
Process & Forward to Gmail
```

## Lambda Function Design

### Original Working Implementation (Pre-4d77e4a)
- **Language**: Rust
- **SDK**: AWS Rust SDK
- **Status**: Worked reliably

### Node.js Migration (Commit 4d77e4a2c8cacc465fb9a94)
- **Language**: Node.js
- **SDK**: AWS SDK v2 (`require('aws-sdk')`)
- **Issue**: Node.js 20.x runtime doesn't include aws-sdk package
- **Result**: ImportModuleError - "Cannot find module 'aws-sdk'"

### Current Implementation (SDK v3 Migration)
- **SDK**: AWS SDK v3 modular imports
- **Imports**:
  ```javascript
  const { S3Client, GetObjectCommand } = require('@aws-sdk/client-s3');
  const { SESClient, SendEmailCommand } = require('@aws-sdk/client-ses');
  ```
- **API Changes**:
  - `s3.getObject().promise()` → `s3.send(new GetObjectCommand())`
  - `s3Response.Body.transformToString()` for reading response
  - `ses.sendEmail().promise()` → `ses.send(new SendEmailCommand())`

## Monitoring Architecture

### CloudWatch Alarms (10 Total)
**P1-Critical** (4 alarms):
- Lambda function errors
- SES bounce rate >5%
- SES complaint rate >0.1%
- Lambda throttling

**P2-Warning** (3 alarms):
- Lambda duration >10s
- SES send failures
- High Lambda concurrency

**P3-Info** (3 alarms):
- Unusual email volume >100/day
- No activity in 7 days
- Lambda invocation tracking

### SNS Notification Routing
- **Critical**: alerts-critical@jimmillerdrums.com
- **Warning**: alerts-warning@jimmillerdrums.com
- **Info**: alerts-info@jimmillerdrums.com

### Alarm Features
- Multiple evaluation periods for noise reduction
- Datapoints to alarm configuration
- OK actions for critical/warning alarms
- Proper missing data handling
- Descriptive subject lines with severity prefix
