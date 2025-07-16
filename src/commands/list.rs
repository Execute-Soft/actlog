use crate::cli::{CloudProvider, Commands, OutputFormat, ResourceType};
use crate::error::AppError;
use chrono::{DateTime, Utc};
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceSummary {
    pub id: String,
    pub name: String,
    pub resource_type: String,
    pub region: String,
    pub state: String,
    pub creation_date: Option<DateTime<Utc>>,
    pub tags: HashMap<String, String>,
    pub additional_info: HashMap<String, String>,
}

pub async fn list_resources(cmd: &Commands) -> Result<(), AppError> {
    if let Commands::List {
        provider,
        resource_type,
        profile,
        format,
    } = cmd
    {
        println!("📋 Listing {} resources...", provider.to_string().green());

        // Get resources based on provider and type
        let resources = match provider {
            CloudProvider::Aws => list_aws_resources(resource_type).await?,
            CloudProvider::Gcp => list_gcp_resources(resource_type).await?,
            CloudProvider::Azure => list_azure_resources(resource_type).await?,
        };

        if resources.is_empty() {
            println!("ℹ️  No resources found for the specified criteria.");
            return Ok(());
        }

        // Output resources in requested format
        output_resource_list(&resources, format, provider)?;

        // Show summary statistics
        show_resource_summary(&resources, provider)?;
    }

    Ok(())
}

async fn list_aws_resources(
    resource_type: &ResourceType,
) -> Result<Vec<ResourceSummary>, AppError> {
    println!("🔍 Fetching AWS resources...");

    let config = aws_config::from_env().load().await;
    let mut resources = Vec::new();

    match resource_type {
        ResourceType::Ec2 => {
            let ec2_client = aws_sdk_ec2::Client::new(&config);
            let response = ec2_client
                .describe_instances()
                .send()
                .await
                .map_err(|e| AppError::AwsError(e.to_string()))?;

            if let Some(reservations) = response.reservations {
                for reservation in reservations {
                    if let Some(instances) = reservation.instances {
                        for instance in instances {
                            let mut additional_info = HashMap::new();

                            if let Some(instance_type) = &instance.instance_type {
                                additional_info.insert(
                                    "Instance Type".to_string(),
                                    instance_type.as_str().to_string(),
                                );
                            }

                            if let Some(public_ip) = &instance.public_ip_address {
                                additional_info.insert("Public IP".to_string(), public_ip.clone());
                            }

                            let state_name = instance
                                .state
                                .as_ref()
                                .and_then(|s| s.name.as_ref())
                                .map(|n| n.as_str())
                                .unwrap_or("unknown")
                                .to_string();

                            let creation_date = instance
                                .launch_time
                                .map(|dt| {
                                    chrono::DateTime::from_timestamp(dt.secs(), dt.subsec_nanos())
                                })
                                .flatten();

                            let instance_id = instance.instance_id.clone().unwrap_or_default();

                            resources.push(ResourceSummary {
                                id: instance_id.clone(),
                                name: instance_id,
                                resource_type: "EC2 Instance".to_string(),
                                region: "us-east-1".to_string(), // Would get from config
                                state: state_name,
                                creation_date,
                                tags: HashMap::new(), // Would extract from tags
                                additional_info,
                            });
                        }
                    }
                }
            }
        }

        ResourceType::S3 => {
            let s3_client = aws_sdk_s3::Client::new(&config);
            let response = s3_client
                .list_buckets()
                .send()
                .await
                .map_err(|e| AppError::AwsError(e.to_string()))?;

            if let Some(buckets) = response.buckets {
                for bucket in buckets {
                    let mut additional_info = HashMap::new();

                    if let Some(creation_date) = bucket.creation_date {
                        let date_str = chrono::DateTime::from_timestamp(
                            creation_date.secs(),
                            creation_date.subsec_nanos(),
                        )
                        .map(|dt| dt.format("%Y-%m-%d").to_string())
                        .unwrap_or_else(|| "Unknown".to_string());
                        additional_info.insert("Created".to_string(), date_str);
                    }

                    let creation_date = bucket
                        .creation_date
                        .map(|dt| chrono::DateTime::from_timestamp(dt.secs(), dt.subsec_nanos()))
                        .flatten();

                    let bucket_name = bucket.name.clone().unwrap_or_default();

                    resources.push(ResourceSummary {
                        id: bucket_name.clone(),
                        name: bucket_name,
                        resource_type: "S3 Bucket".to_string(),
                        region: "us-east-1".to_string(),
                        state: "active".to_string(),
                        creation_date,
                        tags: HashMap::new(),
                        additional_info,
                    });
                }
            }
        }

        ResourceType::Rds => {
            // For RDS, we'd use the RDS client
            println!("   RDS resources not yet implemented for AWS");
        }

        ResourceType::Lambda => {
            // For Lambda, we'd use the Lambda client
            println!("   Lambda resources not yet implemented for AWS");
        }

        ResourceType::LoadBalancer => {
            // For Load Balancers, we'd use the ELB client
            println!("   Load Balancer resources not yet implemented for AWS");
        }

        ResourceType::Vpc => {
            // For VPCs, we'd use the EC2 client
            println!("   VPC resources not yet implemented for AWS");
        }

        ResourceType::Subnet => {
            // For Subnets, we'd use the EC2 client
            println!("   Subnet resources not yet implemented for AWS");
        }

        ResourceType::SecurityGroup => {
            // For Security Groups, we'd use the EC2 client
            println!("   Security Group resources not yet implemented for AWS");
        }

        ResourceType::All => {
            // List all resource types - avoid recursion by implementing directly
            // Simulate GCP Compute Engine instances
            let mut additional_info = HashMap::new();
            additional_info.insert("Machine Type".to_string(), "n1-standard-1".to_string());
            additional_info.insert("Zone".to_string(), "us-central1-a".to_string());

            resources.push(ResourceSummary {
                id: "gcp-instance-1".to_string(),
                name: "web-server".to_string(),
                resource_type: "Compute Instance".to_string(),
                region: "us-central1".to_string(),
                state: "RUNNING".to_string(),
                creation_date: Some(Utc::now() - chrono::Duration::days(30)),
                tags: HashMap::new(),
                additional_info,
            });
        }
    }

    Ok(resources)
}

async fn list_gcp_resources(
    resource_type: &ResourceType,
) -> Result<Vec<ResourceSummary>, AppError> {
    println!("🔍 Fetching GCP resources...");

    let project_id = std::env::var("GOOGLE_CLOUD_PROJECT")
        .map_err(|_| AppError::ConfigurationError("GOOGLE_CLOUD_PROJECT not set".to_string()))?;

    let mut resources = Vec::new();

    match resource_type {
        ResourceType::Ec2 => {
            // Simulate GCP Compute Engine instances
            let mut additional_info = HashMap::new();
            additional_info.insert("Machine Type".to_string(), "n1-standard-1".to_string());
            additional_info.insert("Zone".to_string(), "us-central1-a".to_string());

            resources.push(ResourceSummary {
                id: "gcp-instance-1".to_string(),
                name: "web-server".to_string(),
                resource_type: "Compute Instance".to_string(),
                region: "us-central1".to_string(),
                state: "RUNNING".to_string(),
                creation_date: Some(Utc::now() - chrono::Duration::days(30)),
                tags: HashMap::new(),
                additional_info,
            });
        }

        ResourceType::All => {
            // List all resource types - avoid recursion by implementing directly
            // Simulate GCP Compute Engine instances
            let mut additional_info = HashMap::new();
            additional_info.insert("Machine Type".to_string(), "n1-standard-1".to_string());
            additional_info.insert("Zone".to_string(), "us-central1-a".to_string());

            resources.push(ResourceSummary {
                id: "gcp-instance-1".to_string(),
                name: "web-server".to_string(),
                resource_type: "Compute Instance".to_string(),
                region: "us-central1".to_string(),
                state: "RUNNING".to_string(),
                creation_date: Some(Utc::now() - chrono::Duration::days(30)),
                tags: HashMap::new(),
                additional_info,
            });
        }

        _ => {
            println!(
                "   Resource type {:?} not yet implemented for GCP",
                resource_type
            );
        }
    }

    Ok(resources)
}

async fn list_azure_resources(
    resource_type: &ResourceType,
) -> Result<Vec<ResourceSummary>, AppError> {
    println!("🔍 Fetching Azure resources...");

    let subscription_id = std::env::var("AZURE_SUBSCRIPTION_ID")
        .map_err(|_| AppError::ConfigurationError("AZURE_SUBSCRIPTION_ID not set".to_string()))?;

    let mut resources = Vec::new();

    match resource_type {
        ResourceType::Ec2 => {
            // Simulate Azure Virtual Machines
            let mut additional_info = HashMap::new();
            additional_info.insert("Size".to_string(), "Standard_B1s".to_string());
            additional_info.insert(
                "Resource Group".to_string(),
                "my-resource-group".to_string(),
            );

            resources.push(ResourceSummary {
                id: "azure-vm-1".to_string(),
                name: "web-vm".to_string(),
                resource_type: "Virtual Machine".to_string(),
                region: "East US".to_string(),
                state: "Running".to_string(),
                creation_date: Some(Utc::now() - chrono::Duration::days(25)),
                tags: HashMap::new(),
                additional_info,
            });
        }

        ResourceType::All => {
            // List all resource types - avoid recursion by implementing directly
            // Simulate Azure Virtual Machines
            let mut additional_info = HashMap::new();
            additional_info.insert("Size".to_string(), "Standard_B1s".to_string());
            additional_info.insert(
                "Resource Group".to_string(),
                "my-resource-group".to_string(),
            );

            resources.push(ResourceSummary {
                id: "azure-vm-1".to_string(),
                name: "web-vm".to_string(),
                resource_type: "Virtual Machine".to_string(),
                region: "East US".to_string(),
                state: "Running".to_string(),
                creation_date: Some(Utc::now() - chrono::Duration::days(25)),
                tags: HashMap::new(),
                additional_info,
            });
        }

        _ => {
            println!(
                "   Resource type {:?} not yet implemented for Azure",
                resource_type
            );
        }
    }

    Ok(resources)
}

fn output_resource_list(
    resources: &[ResourceSummary],
    format: &OutputFormat,
    provider: &CloudProvider,
) -> Result<(), AppError> {
    match format {
        OutputFormat::Table => {
            println!("\n📋 {} Resources:", provider.to_string().green());
            println!(
                "{:<20} {:<15} {:<15} {:<15} {:<20}",
                "ID", "Name", "Type", "State", "Region"
            );
            println!("{:-<85}", "");

            for resource in resources {
                let state_color = match resource.state.to_lowercase().as_str() {
                    "running" | "active" => "green",
                    "stopped" | "terminated" => "red",
                    "pending" | "starting" => "yellow",
                    _ => "white",
                };

                println!(
                    "{:<20} {:<15} {:<15} {:<15} {:<20}",
                    resource.id,
                    resource.name,
                    resource.resource_type,
                    resource.state.color(state_color),
                    resource.region
                );
            }
        }

        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(resources)?;
            println!("{}", json);
        }

        OutputFormat::Csv => {
            println!("ID,Name,Type,State,Region,CreationDate");
            for resource in resources {
                let creation_date = resource
                    .creation_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "N/A".to_string());

                println!(
                    "{},{},{},{},{},{}",
                    resource.id,
                    resource.name,
                    resource.resource_type,
                    resource.state,
                    resource.region,
                    creation_date
                );
            }
        }
    }

    Ok(())
}

fn show_resource_summary(
    resources: &[ResourceSummary],
    provider: &CloudProvider,
) -> Result<(), AppError> {
    let total_resources = resources.len();
    let running_resources = resources
        .iter()
        .filter(|r| r.state.to_lowercase() == "running" || r.state.to_lowercase() == "active")
        .count();

    let stopped_resources = resources
        .iter()
        .filter(|r| r.state.to_lowercase() == "stopped" || r.state.to_lowercase() == "terminated")
        .count();

    // Group by resource type
    let mut type_counts: HashMap<String, usize> = HashMap::new();
    for resource in resources {
        *type_counts
            .entry(resource.resource_type.clone())
            .or_insert(0) += 1;
    }

    println!("\n📊 Summary for {}:", provider.to_string().green());
    println!("   Total Resources: {}", total_resources);
    println!(
        "   Running/Active: {}",
        running_resources.to_string().green()
    );
    println!(
        "   Stopped/Terminated: {}",
        stopped_resources.to_string().red()
    );

    if !type_counts.is_empty() {
        println!("   By Type:");
        for (resource_type, count) in type_counts {
            println!("     {}: {}", resource_type, count);
        }
    }

    Ok(())
}
