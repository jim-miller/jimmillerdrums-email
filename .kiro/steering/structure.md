# Project Structure

## Directory Layout
```
jimmillerdrums-email/
├── .kiro/steering/          # Kiro steering files
├── infra/                   # OpenTofu infrastructure code
│   ├── *.tf                 # Main infrastructure definitions
│   ├── monitoring.tf        # CloudWatch alarms (10 alarms)
│   ├── sns.tf              # SNS topics for alarm routing
│   ├── variables.tf        # Input variables
│   └── outputs*.tf         # Infrastructure outputs
├── lambda/                  # Lambda function code
│   ├── index.js            # Email processing logic
│   └── package.json        # Dependencies (SDK v3)
├── deploy.sh               # Deployment automation
├── MONITORING.md           # Monitoring documentation
└── README.md               # Project documentation
```

## Lambda Deployment Pattern
- **Source**: `lambda/index.js` and `lambda/package.json`
- **Package**: Zip containing ONLY source files (no node_modules)
- **Size**: ~1.8KB deployment package
- **Runtime Dependencies**: Provided by Node.js 20.x runtime (AWS SDK v3)

## Infrastructure Organization
- Separate files for logical grouping (monitoring, SNS, outputs)
- Severity-based alarm organization (P1-Critical, P2-Warning, P3-Info)
- Email variables for different alert severities
