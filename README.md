# ActLog - Cloud Resource Management & Optimization CLI

☁️ A comprehensive Rust-based CLI tool for cloud resource management, cost optimization, and automated scaling across AWS, GCP, and Azure.

## Features

- **Multi-Cloud Support**: Manage resources across AWS, Google Cloud Platform, and Microsoft Azure
- **Cost Monitoring**: Generate detailed cost reports and set budget alerts
- **Auto-Scaling**: Automatically scale resources based on usage patterns and thresholds
- **Resource Cleanup**: Identify and remove unused or underutilized resources
- **Resource Listing**: View and monitor all your cloud resources in one place

## Installation

### Prerequisites

- Rust 1.70+
- AWS CLI (for AWS operations)
- Google Cloud SDK (for GCP operations)
- Azure CLI (for Azure operations)

### Build from Source

```bash
git clone https://github.com/Execute-Soft/actlog.git
cd actlog
cargo build --release
```

### Install Script

```bash
curl -fsSL https://raw.githubusercontent.com/Execute-Soft/actlog/main/install.sh | bash
```

## Quick Start

### 1. Configure Cloud Provider Credentials

```bash
# Configure AWS
actlog config --provider aws --api-key YOUR_ACCESS_KEY --secret-key YOUR_SECRET_KEY --region us-east-1

# Configure GCP
actlog config --provider gcp --project-id YOUR_PROJECT_ID

# Configure Azure
actlog config --provider azure --subscription-id YOUR_SUBSCRIPTION_ID
```

### 2. Authenticate with Cloud Providers

```bash
# Authenticate with AWS
actlog authenticate --provider aws

# Authenticate with GCP
actlog authenticate --provider gcp

# Authenticate with Azure
actlog authenticate --provider azure
```

### 3. Generate Cost Reports

```bash
# Generate AWS cost report for last 30 days
actlog report-costs --provider aws

# Generate cost report with custom date range
actlog report-costs --provider aws --start-date 2024-01-01 --end-date 2024-01-31

# Generate cost report with budget threshold
actlog report-costs --provider aws --budget-threshold 1000.0
```

### 4. Auto-Scale Resources

```bash
# Auto-scale AWS instances
actlog scale-instances --provider aws --min-instances 2 --max-instances 10 --cpu-threshold 70

# Dry run to see what would be scaled
actlog scale-instances --provider aws --dry-run
```

### 5. Clean Up Unused Resources

```bash
# Clean up unused EC2 instances
actlog cleanup --provider aws --resource-type ec2 --age-threshold 30

# Clean up unused S3 buckets
actlog cleanup --provider aws --resource-type s3 --dry-run

# Force cleanup without confirmation
actlog cleanup --provider aws --resource-type ec2 --force
```

### 6. List Resources

```bash
# List all AWS resources
actlog list --provider aws --resource-type all

# List specific resource types
actlog list --provider aws --resource-type ec2
actlog list --provider aws --resource-type s3

# Output in different formats
actlog list --provider aws --resource-type ec2 --format json
actlog list --provider aws --resource-type ec2 --format csv
```

## Commands Reference

### Authentication

```bash
actlog authenticate --provider <aws|gcp|azure> [--profile <name>] [--force]
```

### Configuration

```bash
actlog config --provider <aws|gcp|azure> [--profile <name>] [--api-key <key>] [--secret-key <secret>] [--region <region>] [--project-id <id>] [--subscription-id <id>]
```

### Cost Reporting

```bash
actlog report-costs --provider <aws|gcp|azure> [--start-date <YYYY-MM-DD>] [--end-date <YYYY-MM-DD>] [--format <table|json|csv>] [--budget-threshold <amount>] [--profile <name>]
```

### Auto-Scaling

```bash
actlog scale-instances --provider <aws|gcp|azure> [--min-instances <number>] [--max-instances <number>] [--cpu-threshold <percentage>] [--memory-threshold <percentage>] [--resource-group <name>] [--profile <name>] [--dry-run]
```

### Resource Cleanup

```bash
actlog cleanup --provider <aws|gcp|azure> --resource-type <ec2|s3|rds|lambda|loadbalancer|vpc|subnet|securitygroup|all> [--age-threshold <days>] [--utilization-threshold <percentage>] [--profile <name>] [--dry-run] [--force]
```

### Resource Listing

```bash
actlog list --provider <aws|gcp|azure> --resource-type <ec2|s3|rds|lambda|loadbalancer|vpc|subnet|securitygroup|all> [--profile <name>] [--format <table|json|csv>]
```

## Environment Variables

### AWS

- `AWS_ACCESS_KEY_ID`
- `AWS_SECRET_ACCESS_KEY`
- `AWS_DEFAULT_REGION`

### GCP

- `GOOGLE_CLOUD_PROJECT`
- `GOOGLE_APPLICATION_CREDENTIALS`

### Azure

- `AZURE_SUBSCRIPTION_ID`
- `AZURE_TENANT_ID`
- `AZURE_CLIENT_ID`
- `AZURE_CLIENT_SECRET`

## Configuration Files

Configuration and credentials are stored in:

- **macOS**: `~/Library/Application Support/actlog/`
- **Linux**: `~/.config/actlog/`
- **Windows**: `%APPDATA%\actlog\`

## Examples

### Monitor AWS Costs with Alerts

```bash
# Set up cost monitoring with $1000 budget threshold
actlog report-costs --provider aws --budget-threshold 1000.0 --format table
```

### Auto-Scale Based on CPU Usage

```bash
# Scale instances when CPU usage exceeds 80%
actlog scale-instances --provider aws --cpu-threshold 80.0 --min-instances 2 --max-instances 20
```

### Clean Up Old Resources

```bash
# Find and clean up resources older than 60 days
actlog cleanup --provider aws --resource-type ec2 --age-threshold 60 --dry-run
```

### Generate Resource Inventory

```bash
# Generate a complete inventory of all resources
actlog list --provider aws --resource-type all --format csv > inventory.csv
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- **Issues**: [GitHub Issues](https://github.com/Execute-Soft/actlog/issues)
- **Documentation**: [GitHub Wiki](https://github.com/Execute-Soft/actlog/wiki)
- **Discussions**: [GitHub Discussions](https://github.com/Execute-Soft/actlog/discussions)

## Roadmap

- [ ] Support for additional cloud providers (Oracle Cloud, DigitalOcean)
- [ ] Advanced cost optimization recommendations
- [ ] Scheduled cleanup and scaling operations
- [ ] Integration with monitoring and alerting systems
- [ ] Web dashboard for resource management
- [ ] Multi-account and multi-region support
- [ ] Cost forecasting and budgeting tools
