use clap::{Parser, Subcommand, ValueEnum};
use std::fmt;

#[derive(Parser)]
#[command(
    name = "actlog",
    about = "☁️ A CLI tool for cloud resource management, cost optimization, and automated scaling across AWS, GCP, and Azure.",
    version,
    long_about = "A comprehensive CLI tool that helps businesses optimize their cloud resource usage, manage costs, and automate cloud infrastructure scaling across multiple cloud providers."
)]
pub struct Cli {
    /// The command to run
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Authenticate with cloud providers
    Authenticate {
        /// Cloud provider to authenticate with
        #[arg(value_enum)]
        provider: CloudProvider,

        /// Profile name for storing credentials
        #[arg(short, long, default_value = "default")]
        profile: String,

        /// Force re-authentication even if credentials exist
        #[arg(short, long)]
        force: bool,
    },

    /// Generate cost reports for cloud resources
    ReportCosts {
        /// Cloud provider to generate report for
        #[arg(value_enum)]
        provider: CloudProvider,

        /// Start date for cost analysis (YYYY-MM-DD)
        #[arg(short, long)]
        start_date: Option<String>,

        /// End date for cost analysis (YYYY-MM-DD)
        #[arg(short, long)]
        end_date: Option<String>,

        /// Output format for the report
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Table)]
        format: OutputFormat,

        /// Set cost budget threshold for alerts
        #[arg(short, long)]
        budget_threshold: Option<f64>,

        /// Profile name to use for authentication
        #[arg(short, long, default_value = "default")]
        profile: String,
    },

    /// Auto-scale cloud resources based on usage patterns
    ScaleInstances {
        /// Cloud provider to scale resources on
        #[arg(value_enum)]
        provider: CloudProvider,

        /// Minimum number of instances
        #[arg(short, long, default_value_t = 1)]
        min_instances: i32,

        /// Maximum number of instances
        #[arg(short, long, default_value_t = 10)]
        max_instances: i32,

        /// CPU utilization threshold for scaling (percentage)
        #[arg(short, long, default_value_t = 70.0)]
        cpu_threshold: f64,

        /// Memory utilization threshold for scaling (percentage)
        #[arg(short, long, default_value_t = 80.0)]
        memory_threshold: f64,

        /// Auto-scaling group name or resource identifier
        #[arg(short, long)]
        resource_group: Option<String>,

        /// Profile name to use for authentication
        #[arg(short, long, default_value = "default")]
        profile: String,

        /// Dry run mode (show what would be done without executing)
        #[arg(short, long)]
        dry_run: bool,
    },

    /// Clean up unused or underutilized resources
    Cleanup {
        /// Cloud provider to clean up resources on
        #[arg(value_enum)]
        provider: CloudProvider,

        /// Type of resource to clean up
        #[arg(value_enum)]
        resource_type: ResourceType,

        /// Age threshold for resources to be considered for cleanup (days)
        #[arg(short, long, default_value_t = 30)]
        age_threshold: u32,

        /// Utilization threshold for resources to be considered underutilized (percentage)
        #[arg(short, long, default_value_t = 10.0)]
        utilization_threshold: f64,

        /// Profile name to use for authentication
        #[arg(short, long, default_value = "default")]
        profile: String,

        /// Dry run mode (show what would be cleaned up without executing)
        #[arg(short, long)]
        dry_run: bool,

        /// Force cleanup without confirmation prompts
        #[arg(short, long)]
        force: bool,
    },

    /// List available resources and their current status
    List {
        /// Cloud provider to list resources from
        #[arg(value_enum)]
        provider: CloudProvider,

        /// Type of resource to list
        #[arg(value_enum)]
        resource_type: ResourceType,

        /// Profile name to use for authentication
        #[arg(short, long, default_value = "default")]
        profile: String,

        /// Output format for the list
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Table)]
        format: OutputFormat,
    },

    /// Configure cloud provider settings and credentials
    Config {
        /// Cloud provider to configure
        #[arg(value_enum)]
        provider: CloudProvider,

        /// Profile name for the configuration
        #[arg(short, long, default_value = "default")]
        profile: String,

        /// Set API key or access key
        #[arg(short, long)]
        api_key: Option<String>,

        /// Set secret key
        #[arg(short, long)]
        secret_key: Option<String>,

        /// Set region
        #[arg(short, long)]
        region: Option<String>,

        /// Set project ID (for GCP)
        #[arg(short, long)]
        project_id: Option<String>,

        /// Set subscription ID (for Azure)
        #[arg(short, long)]
        subscription_id: Option<String>,
    },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum CloudProvider {
    Aws,
    Gcp,
    Azure,
}

impl fmt::Display for CloudProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CloudProvider::Aws => write!(f, "AWS"),
            CloudProvider::Gcp => write!(f, "GCP"),
            CloudProvider::Azure => write!(f, "Azure"),
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ResourceType {
    Ec2,
    S3,
    Rds,
    Lambda,
    LoadBalancer,
    Vpc,
    Subnet,
    SecurityGroup,
    All,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Table,
    Json,
    Csv,
}
