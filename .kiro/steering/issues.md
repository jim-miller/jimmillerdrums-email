# Known Issues and Troubleshooting

## Current Critical Issue

### Lambda Still Failing After SDK v3 Migration
**Status**: UNRESOLVED as of 2026-01-14

**Symptoms**:
- All emails in last 12 hours (14:02-17:14) failed with ImportModuleError
- Error: "Cannot find module 'aws-sdk'"
- Lambda logs show old SDK v2 code still running

**Investigation**:
- Lambda code updated to SDK v3 in `lambda/index.js`
- Deployment package created and uploaded multiple times
- `aws lambda update-function-code` executed successfully
- Lambda configuration shows correct CodeSha256 and LastModified timestamp
- BUT: Actual running code appears to be old version

**Possible Causes**:
1. Lambda function code not actually updated despite successful API calls
2. Cached version of function still running
3. Deployment package issue (wrong files included)
4. Lambda function state issue

**Next Steps to Investigate**:
- Download deployed Lambda package and verify contents
- Check Lambda function state and update status
- Consider deleting and recreating Lambda function
- Verify SES receipt rule is triggering correct Lambda version

## Historical Issues (Resolved)

### Issue 1: AWS SDK v2 Not Available in Node.js 20.x
**Root Cause**: AWS removed aws-sdk (v2) from Node.js 18+ runtimes to reduce bundle size

**Solution**: Migrate to AWS SDK v3 with modular imports

### Issue 2: Lambda Package Size Exceeding 250MB Limit
**Root Cause**: Bundling AWS SDK v3 with all dependencies created 200MB+ packages

**Solution**: Deploy minimal package without node_modules, rely on runtime-provided SDK v3

### Issue 3: Composite Alarm Name Escaping
**Root Cause**: Alarm names with special characters (brackets, hyphens) require careful escaping

**Solution**: Commented out composite alarm, using individual alarms instead

## Deployment Best Practices

### Lambda Deployment
```bash
cd lambda
rm -rf node_modules
zip -r ../lambda-final.zip index.js package.json
cd ..
aws lambda update-function-code \
  --function-name jimmillerdrums-email-processor \
  --zip-file fileb://lambda-final.zip \
  --region us-east-1
```

### Verification Steps
1. Check deployment: `aws lambda get-function-configuration`
2. Wait for function to be Active
3. Send test email
4. Check logs: `aws logs tail /aws/lambda/jimmillerdrums-email-processor`
5. Verify email forwarded to Gmail

## Monitoring Verification

### Test Critical Alarm
```bash
aws cloudwatch set-alarm-state \
  --alarm-name "[P1-CRITICAL] jimmillerdrums-email - Lambda Function Errors" \
  --state-value ALARM \
  --state-reason "Testing alarm notification" \
  --region us-east-1
```

**Result**: Successfully triggered alarm, email received at alerts-critical@jimmillerdrums.com
