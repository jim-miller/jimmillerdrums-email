#!/bin/bash

# DEPRECATED: This script is for the old Node.js Lambda
# Use deploy-rust.sh for the current Rust implementation

# Email Infrastructure Deployment Script
# This script helps manage the email infrastructure for jimmillerdrums.com

set -e

echo "⚠️  WARNING: This script is DEPRECATED"
echo "Use deploy-rust.sh for the current Rust Lambda implementation"
echo ""
read -p "Continue with legacy script? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    exit 1
fi

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_NAME="jimmillerdrums-email"
TFVARS_FILE="infra/opentofu.tfvars"
INFRA_DIR="infra"

# Functions
print_header() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}  Email Infrastructure Manager  ${NC}"
    echo -e "${BLUE}================================${NC}"
    echo
}

print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

check_prerequisites() {
    echo "Checking prerequisites..."
    
    # Check if OpenTofu is installed
    if ! command -v tofu &> /dev/null; then
        print_error "OpenTofu is not installed. Please install it first."
        exit 1
    fi
    print_status "OpenTofu is installed"
    
    # Check if AWS CLI is installed
    if ! command -v aws &> /dev/null; then
        print_error "AWS CLI is not installed. Please install it first."
        exit 1
    fi
    print_status "AWS CLI is installed"
    
    # Check if Rust is installed
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo is not installed. Please install it first."
        exit 1
    fi
    print_status "Rust/Cargo is installed"
    
    # Check if tfvars file exists
    if [ ! -f "$TFVARS_FILE" ]; then
        print_error "Configuration file $TFVARS_FILE not found."
        echo "Please copy infra/opentofu.tfvars.example to infra/opentofu.tfvars and configure it."
        exit 1
    fi
    print_status "Configuration file found"
    
    echo
}

build_lambda() {
    echo "Building Lambda function..."
    if ./build.sh; then
        print_status "Lambda function built successfully"
    else
        print_error "Failed to build Lambda function"
        exit 1
    fi
    echo
}

deploy_infrastructure() {
    echo "Deploying infrastructure..."
    
    # Initialize OpenTofu
    echo "Initializing OpenTofu..."
    if (cd "$INFRA_DIR" && tofu init); then
        print_status "OpenTofu initialized"
    else
        print_error "Failed to initialize OpenTofu"
        exit 1
    fi
    
    # Plan deployment
    echo "Planning deployment..."
    if (cd "$INFRA_DIR" && tofu plan -var-file="opentofu.tfvars"); then
        print_status "Deployment plan created"
    else
        print_error "Failed to create deployment plan"
        exit 1
    fi
    
    # Ask for confirmation
    echo
    read -p "Do you want to apply these changes? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "Applying changes..."
        if (cd "$INFRA_DIR" && tofu apply -var-file="opentofu.tfvars" -auto-approve); then
            print_status "Infrastructure deployed successfully"
        else
            print_error "Failed to deploy infrastructure"
            exit 1
        fi
    else
        print_warning "Deployment cancelled"
        exit 0
    fi
    echo
}

check_status() {
    echo "Checking infrastructure status..."
    
    # Check SES domain verification
    echo "Checking SES domain verification..."
    DOMAIN_STATUS=$(aws ses get-identity-verification-attributes --identities jimmillerdrums.com --query 'VerificationAttributes."jimmillerdrums.com".VerificationStatus' --output text 2>/dev/null || echo "Unknown")
    if [ "$DOMAIN_STATUS" = "Success" ]; then
        print_status "Domain verification: Success"
    else
        print_warning "Domain verification: $DOMAIN_STATUS"
    fi
    
    # Check DKIM verification
    echo "Checking DKIM verification..."
    DKIM_STATUS=$(aws ses get-identity-dkim-attributes --identities jimmillerdrums.com --query 'DkimAttributes."jimmillerdrums.com".DkimVerificationStatus' --output text 2>/dev/null || echo "Unknown")
    if [ "$DKIM_STATUS" = "Success" ]; then
        print_status "DKIM verification: Success"
    else
        print_warning "DKIM verification: $DKIM_STATUS"
    fi
    
    # Check Lambda function
    echo "Checking Lambda function..."
    LAMBDA_STATUS=$(aws lambda get-function --function-name jimmillerdrums-email-processor --query 'Configuration.State' --output text 2>/dev/null || echo "NotFound")
    if [ "$LAMBDA_STATUS" = "Active" ]; then
        print_status "Lambda function: Active"
    else
        print_warning "Lambda function: $LAMBDA_STATUS"
    fi
    
    # Check SES receipt rules
    echo "Checking SES receipt rules..."
    RULE_SET_STATUS=$(aws ses describe-active-receipt-rule-set --query 'Metadata.Name' --output text 2>/dev/null || echo "None")
    if [ "$RULE_SET_STATUS" = "jimmillerdrums-email-rules" ]; then
        print_status "Active receipt rule set: $RULE_SET_STATUS"
    else
        print_warning "Active receipt rule set: $RULE_SET_STATUS"
    fi
    
    echo
}

show_outputs() {
    echo "Infrastructure outputs:"
    (cd "$INFRA_DIR" && tofu output)
    echo
}

show_logs() {
    echo "Recent Lambda function logs:"
    aws logs describe-log-streams \
        --log-group-name "/aws/lambda/jimmillerdrums-email-processor" \
        --order-by LastEventTime \
        --descending \
        --max-items 1 \
        --query 'logStreams[0].logStreamName' \
        --output text | xargs -I {} aws logs get-log-events \
        --log-group-name "/aws/lambda/jimmillerdrums-email-processor" \
        --log-stream-name {} \
        --limit 10 \
        --query 'events[*].[timestamp,message]' \
        --output table
    echo
}

test_email() {
    echo "Testing email functionality..."
    print_warning "To test the email system:"
    echo "1. Send an email to test@jimmillerdrums.com"
    echo "2. Check your Gmail inbox for the forwarded email"
    echo "3. Check CloudWatch logs for processing details"
    echo "4. Verify the email was stored in S3 bucket"
    echo
    echo "To view logs, run: $0 logs"
    echo
}

destroy_infrastructure() {
    echo -e "${RED}WARNING: This will destroy all email infrastructure!${NC}"
    echo "This action cannot be undone."
    echo
    read -p "Are you sure you want to destroy the infrastructure? (type 'yes' to confirm): " -r
    if [ "$REPLY" = "yes" ]; then
        echo "Destroying infrastructure..."
        if (cd "$INFRA_DIR" && tofu destroy -var-file="opentofu.tfvars" -auto-approve); then
            print_status "Infrastructure destroyed"
        else
            print_error "Failed to destroy infrastructure"
            exit 1
        fi
    else
        print_warning "Destruction cancelled"
    fi
    echo
}

show_help() {
    echo "Usage: $0 [command]"
    echo
    echo "Commands:"
    echo "  deploy    - Deploy the email infrastructure"
    echo "  status    - Check the status of deployed infrastructure"
    echo "  outputs   - Show infrastructure outputs"
    echo "  logs      - Show recent Lambda function logs"
    echo "  test      - Show instructions for testing email functionality"
    echo "  destroy   - Destroy the email infrastructure"
    echo "  help      - Show this help message"
    echo
    echo "If no command is provided, 'deploy' is assumed."
    echo
}

# Main script
print_header

case "${1:-deploy}" in
    "deploy")
        check_prerequisites
        build_lambda
        deploy_infrastructure
        check_status
        show_outputs
        echo -e "${GREEN}Deployment complete!${NC}"
        echo "Next steps:"
        echo "1. Test email forwarding by sending an email to test@jimmillerdrums.com"
        echo "2. Configure Gmail SMTP using the instructions in GMAIL_INTEGRATION.md"
        echo "3. Monitor the system using CloudWatch logs and alarms"
        ;;
    "status")
        check_status
        ;;
    "outputs")
        show_outputs
        ;;
    "logs")
        show_logs
        ;;
    "test")
        test_email
        ;;
    "destroy")
        destroy_infrastructure
        ;;
    "help"|"-h"|"--help")
        show_help
        ;;
    *)
        print_error "Unknown command: $1"
        show_help
        exit 1
        ;;
esac
