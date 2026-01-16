# AWS Lambda Code Standards

## SDK Version Requirements
- **MUST USE**: AWS SDK v3 with modular imports
- **NEVER USE**: AWS SDK v2 (`aws-sdk` package) - not available in Node.js 18+

## Import Patterns

### Correct (SDK v3)
```javascript
const { S3Client, GetObjectCommand } = require('@aws-sdk/client-s3');
const { SESClient, SendEmailCommand } = require('@aws-sdk/client-ses');

const s3 = new S3Client({ region: 'us-east-1' });
const ses = new SESClient({ region: 'us-east-1' });
```

### Incorrect (SDK v2 - DO NOT USE)
```javascript
const AWS = require('aws-sdk');  // ❌ Not available in Node.js 18+
const s3 = new AWS.S3();
const ses = new AWS.SES();
```

## API Call Patterns

### S3 GetObject (SDK v3)
```javascript
const response = await s3.send(new GetObjectCommand({
  Bucket: bucketName,
  Key: objectKey
}));
const emailContent = await response.Body.transformToString();
```

### SES SendEmail (SDK v3)
```javascript
await ses.send(new SendEmailCommand({
  Source: 'sender@example.com',
  Destination: { ToAddresses: ['recipient@example.com'] },
  Message: {
    Subject: { Data: 'Subject' },
    Body: { Text: { Data: 'Body content' } }
  }
}));
```

## Deployment Standards

### Package Contents
- **Include**: `index.js`, `package.json`
- **Exclude**: `node_modules/` (runtime provides SDK v3)
- **Target Size**: ~1-2KB

### package.json Dependencies
```json
{
  "dependencies": {
    "@aws-sdk/client-s3": "^3.x.x",
    "@aws-sdk/client-ses": "^3.x.x"
  }
}
```

Note: These dependencies are for local development/testing only. Do NOT bundle them in deployment package.

## Error Handling
- Always use try-catch blocks for AWS SDK calls
- Log errors to CloudWatch with context
- Return appropriate HTTP status codes
- Include error details in CloudWatch metrics
