# Monitoring & Alerting

This infrastructure includes comprehensive CloudWatch alarms following AWS best practices for SES and Lambda monitoring.

## Alarm Severity Levels

Alarms are categorized by severity with dedicated email routing:

### P1 - CRITICAL (Immediate Action Required)
**Email**: `alerts-critical@jimmillerdrums.com`

- **Lambda Function Errors**: Email forwarding is failing, emails NOT being delivered
- **SES Bounce Rate High**: >5% bounce rate, account suspension risk
- **SES Complaint Rate High**: >0.1% complaint rate, account suspension risk  
- **Lambda Throttling**: Function throttled, emails may be lost
- **Email System Health**: Composite alarm for overall system failure

**Action**: Investigate immediately, emails are not being processed.

### P2 - WARNING (Action Within 24 Hours)
**Email**: `alerts-warning@jimmillerdrums.com`

- **Lambda Slow Performance**: Execution time >10 seconds
- **SES Send Failures**: Multiple send failures detected
- **High Lambda Concurrency**: Approaching concurrency limits

**Action**: Review within 24 hours to prevent escalation to critical.

### P3 - INFO (Review When Convenient)
**Email**: `alerts-info@jimmillerdrums.com`

- **Unusual Email Volume**: >100 emails/day (potential spam)
- **No Email Activity**: No emails received in 7 days

**Action**: Review when convenient, may be normal behavior.

## Alarm Features

### Noise Reduction
- **Evaluation Periods**: Multiple datapoints required before alarming
- **Datapoints to Alarm**: Prevents single spike false positives
- **OK Actions**: Notifies when issues resolve (Critical/Warning only)
- **Treat Missing Data**: Configured to avoid false alarms during no-traffic periods

### Actionable Descriptions
Each alarm includes:
- Clear description of the problem
- Impact on email delivery
- Specific action to take

### Subject Line Format
SNS topics use display names that appear in email subjects:
- `[P1-CRITICAL] jimmillerdrums-email Email System`
- `[P2-WARNING] jimmillerdrums-email Email System`
- `[P3-INFO] jimmillerdrums-email Email System`

## Setup

1. **Configure Email Addresses** (optional, defaults provided):
   ```bash
   # In terraform.tfvars or environment variables
   critical_alarm_email = "alerts-critical@jimmillerdrums.com"
   warning_alarm_email  = "alerts-warning@jimmillerdrums.com"
   info_alarm_email     = "alerts-info@jimmillerdrums.com"
   ```

2. **Deploy**:
   ```bash
   cd infra
   tofu apply
   ```

3. **Confirm SNS Subscriptions**:
   Check each email inbox and confirm the SNS subscription (one-time setup).

## Monitoring Dashboard

View alarms in AWS Console:
```
CloudWatch → Alarms → Filter by "jimmillerdrums-email"
```

## Alarm Thresholds

| Metric | Threshold | Rationale |
|--------|-----------|-----------|
| Lambda Errors | >0 in 10min | Any error means emails not forwarded |
| SES Bounce Rate | >5% | AWS suspension threshold |
| SES Complaint Rate | >0.1% | AWS suspension threshold |
| Lambda Duration | >10s avg | Normal processing <2s |
| Lambda Throttles | >0 | Should never throttle at current volume |
| Email Volume | >100/day | Typical volume <10/day |
| No Activity | <1 in 7 days | Unusual but may be normal |

## Testing Alarms

```bash
# Test Lambda error alarm
aws lambda invoke --function-name jimmillerdrums-email-processor \
  --payload '{"invalid":"payload"}' /dev/null

# Check alarm state
aws cloudwatch describe-alarms \
  --alarm-name-prefix "[P1-CRITICAL] jimmillerdrums-email"
```

## Cost

- CloudWatch Alarms: $0.10/alarm/month = ~$1.00/month
- SNS: First 1,000 email notifications free, then $2/100,000
- **Total**: ~$1-2/month

## Best Practices Implemented

✅ Severity-based routing  
✅ Descriptive alarm names with severity prefix  
✅ Actionable alarm descriptions  
✅ Noise reduction (evaluation periods, datapoints)  
✅ OK actions for critical/warning alarms  
✅ Composite alarm for system health  
✅ Treat missing data appropriately  
✅ AWS SES reputation monitoring  
✅ Lambda performance and error tracking  
✅ Cost-effective alarm configuration
