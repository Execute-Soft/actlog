use crate::cli::{CloudProvider, Commands};
use crate::error::AppError;
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudCredentials {
    pub provider: String,
    pub profile: String,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub region: Option<String>,
    pub project_id: Option<String>,
    pub subscription_id: Option<String>,
    pub token: Option<String>,
    pub expires_at: Option<String>,
}

pub async fn authenticate(cmd: &Commands) -> Result<(), AppError> {
    if let Commands::Authenticate {
        provider,
        profile,
        force,
    } = cmd
    {
        println!("🔐 Authenticating with {}...", provider.to_string().green());

        let config_dir = get_config_dir()?;
        let credentials_file = config_dir.join("credentials.json");

        // Load existing credentials
        let mut credentials = load_credentials(&credentials_file)?;

        // Check if credentials already exist and force is not set
        if !*force {
            if let Some(existing) =
                credentials.get(&format!("{}_{}", provider.to_string(), profile))
            {
                if existing.token.is_some() && !is_token_expired(existing) {
                    println!(
                        "✅ Already authenticated with {} (profile: {})",
                        provider.to_string().green(),
                        profile.green()
                    );
                    return Ok(());
                }
            }
        }

        // Perform authentication based on provider
        let new_credentials = match provider {
            CloudProvider::Aws => authenticate_aws(profile).await?,
            CloudProvider::Gcp => authenticate_gcp(profile).await?,
            CloudProvider::Azure => authenticate_azure(profile).await?,
        };

        // Store credentials
        credentials.insert(
            format!("{}_{}", provider.to_string(), profile),
            new_credentials,
        );
        save_credentials(&credentials_file, &credentials)?;

        println!(
            "✅ Successfully authenticated with {} (profile: {})",
            provider.to_string().green(),
            profile.green()
        );
    }

    Ok(())
}

async fn authenticate_aws(profile: &str) -> Result<CloudCredentials, AppError> {
    println!("🔑 Setting up AWS authentication...");

    // Check for AWS credentials in environment variables
    let access_key = std::env::var("AWS_ACCESS_KEY_ID").ok();
    let secret_key = std::env::var("AWS_SECRET_ACCESS_KEY").ok();
    let region = std::env::var("AWS_DEFAULT_REGION").ok();

    if access_key.is_none() || secret_key.is_none() {
        println!("⚠️  AWS credentials not found in environment variables.");
        println!("   Please set AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, and optionally AWS_DEFAULT_REGION");
        println!(
            "   Or run: actlog config --provider aws --api-key YOUR_KEY --secret-key YOUR_SECRET"
        );
        return Err(AppError::AuthenticationError(
            "AWS credentials not found".to_string(),
        ));
    }

    // Validate credentials by making a test API call
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .load()
        .await;
    let ec2_client = aws_sdk_ec2::Client::new(&config);

    match ec2_client.describe_regions().send().await {
        Ok(_) => {
            println!("✅ AWS credentials validated successfully");
            Ok(CloudCredentials {
                provider: "aws".to_string(),
                profile: profile.to_string(),
                access_key,
                secret_key,
                region,
                project_id: None,
                subscription_id: None,
                token: Some("aws_credentials_valid".to_string()),
                expires_at: None,
            })
        }
        Err(e) => {
            println!("❌ AWS credentials validation failed: {}", e);
            Err(AppError::AuthenticationError(format!(
                "AWS credentials validation failed: {}",
                e
            )))
        }
    }
}

async fn authenticate_gcp(profile: &str) -> Result<CloudCredentials, AppError> {
    println!("🔑 Setting up GCP authentication...");

    // Check for GCP credentials in environment variables
    let project_id = std::env::var("GOOGLE_CLOUD_PROJECT").ok();
    let credentials_path = std::env::var("GOOGLE_APPLICATION_CREDENTIALS").ok();

    if project_id.is_none() {
        println!("⚠️  GCP project ID not found in environment variables.");
        println!("   Please set GOOGLE_CLOUD_PROJECT");
        return Err(AppError::AuthenticationError(
            "GCP project ID not found".to_string(),
        ));
    }

    if credentials_path.is_none() {
        println!("⚠️  GCP credentials file path not found in environment variables.");
        println!("   Please set GOOGLE_APPLICATION_CREDENTIALS to point to your service account key file");
        return Err(AppError::AuthenticationError(
            "GCP credentials file not found".to_string(),
        ));
    }

    Ok(CloudCredentials {
        provider: "gcp".to_string(),
        profile: profile.to_string(),
        access_key: None,
        secret_key: None,
        region: None,
        project_id,
        subscription_id: None,
        token: Some("gcp_credentials_valid".to_string()),
        expires_at: None,
    })
}

async fn authenticate_azure(profile: &str) -> Result<CloudCredentials, AppError> {
    println!("🔑 Setting up Azure authentication...");

    // Check for Azure credentials in environment variables
    let subscription_id = std::env::var("AZURE_SUBSCRIPTION_ID").ok();
    let tenant_id = std::env::var("AZURE_TENANT_ID").ok();
    let client_id = std::env::var("AZURE_CLIENT_ID").ok();
    let client_secret = std::env::var("AZURE_CLIENT_SECRET").ok();

    if subscription_id.is_none() {
        println!("⚠️  Azure subscription ID not found in environment variables.");
        println!("   Please set AZURE_SUBSCRIPTION_ID");
        return Err(AppError::AuthenticationError(
            "Azure subscription ID not found".to_string(),
        ));
    }

    if tenant_id.is_none() || client_id.is_none() || client_secret.is_none() {
        println!("⚠️  Azure service principal credentials not found in environment variables.");
        println!("   Please set AZURE_TENANT_ID, AZURE_CLIENT_ID, and AZURE_CLIENT_SECRET");
        return Err(AppError::AuthenticationError(
            "Azure service principal credentials not found".to_string(),
        ));
    }

    Ok(CloudCredentials {
        provider: "azure".to_string(),
        profile: profile.to_string(),
        access_key: None,
        secret_key: None,
        region: None,
        project_id: None,
        subscription_id,
        token: Some("azure_credentials_valid".to_string()),
        expires_at: None,
    })
}

fn get_config_dir() -> Result<std::path::PathBuf, AppError> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| {
            AppError::ConfigurationError("Could not determine config directory".to_string())
        })?
        .join("actlog");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    Ok(config_dir)
}

fn load_credentials(
    credentials_file: &Path,
) -> Result<HashMap<String, CloudCredentials>, AppError> {
    if credentials_file.exists() {
        let content = fs::read_to_string(credentials_file)?;
        let credentials: HashMap<String, CloudCredentials> = serde_json::from_str(&content)?;
        Ok(credentials)
    } else {
        Ok(HashMap::new())
    }
}

fn save_credentials(
    credentials_file: &Path,
    credentials: &HashMap<String, CloudCredentials>,
) -> Result<(), AppError> {
    let content = serde_json::to_string_pretty(credentials)?;
    fs::write(credentials_file, content)?;
    Ok(())
}

fn is_token_expired(credentials: &CloudCredentials) -> bool {
    if let Some(expires_at) = &credentials.expires_at {
        if let Ok(expiry) = chrono::DateTime::parse_from_rfc3339(expires_at) {
            return chrono::Utc::now() > expiry;
        }
    }
    false
}
