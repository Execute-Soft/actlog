use crate::cli::{CloudProvider, Commands};
use crate::error::AppError;
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudConfig {
    pub provider: String,
    pub profile: String,
    pub api_key: Option<String>,
    pub secret_key: Option<String>,
    pub region: Option<String>,
    pub project_id: Option<String>,
    pub subscription_id: Option<String>,
}

pub async fn configure(cmd: &Commands) -> Result<(), AppError> {
    if let Commands::Config {
        provider,
        profile,
        api_key,
        secret_key,
        region,
        project_id,
        subscription_id,
    } = cmd
    {
        println!(
            "⚙️  Configuring {} settings for profile '{}'...",
            provider.to_string().green(),
            profile.green()
        );

        let config_dir = get_config_dir()?;
        let config_file = config_dir.join("config.json");

        // Load existing configuration
        let mut configs = load_configs(&config_file)?;

        // Create or update configuration
        let mut config = CloudConfig {
            provider: provider.to_string().to_lowercase(),
            profile: profile.to_string(),
            api_key: api_key.clone(),
            secret_key: secret_key.clone(),
            region: region.clone(),
            project_id: project_id.clone(),
            subscription_id: subscription_id.clone(),
        };

        // If no new values provided, prompt for them
        if api_key.is_none()
            && secret_key.is_none()
            && region.is_none()
            && project_id.is_none()
            && subscription_id.is_none()
        {
            config = prompt_for_config(provider, profile)?;
        }

        // Store configuration
        let key = format!("{}_{}", provider.to_string().to_lowercase(), profile);
        configs.insert(key, config);
        save_configs(&config_file, &configs)?;

        println!("✅ Configuration saved successfully!");

        // Show current configuration
        show_configuration(provider, profile, &configs)?;
    }

    Ok(())
}

fn prompt_for_config(provider: &CloudProvider, profile: &str) -> Result<CloudConfig, AppError> {
    use std::io::{self, Write};

    println!(
        "\n📝 Please provide configuration details for {} (profile: {}):",
        provider.to_string().green(),
        profile.green()
    );

    let mut config = CloudConfig {
        provider: provider.to_string().to_lowercase(),
        profile: profile.to_string(),
        api_key: None,
        secret_key: None,
        region: None,
        project_id: None,
        subscription_id: None,
    };

    match provider {
        CloudProvider::Aws => {
            print!("AWS Access Key ID: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            config.api_key = Some(input.trim().to_string());

            print!("AWS Secret Access Key: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            config.secret_key = Some(input.trim().to_string());

            print!("AWS Region (e.g., us-east-1): ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            config.region = Some(input.trim().to_string());
        }

        CloudProvider::Gcp => {
            print!("GCP Project ID: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            config.project_id = Some(input.trim().to_string());

            println!("Note: For GCP, you'll need to set GOOGLE_APPLICATION_CREDENTIALS environment variable");
            println!("to point to your service account key file.");
        }

        CloudProvider::Azure => {
            print!("Azure Subscription ID: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            config.subscription_id = Some(input.trim().to_string());

            print!("Azure Region (e.g., eastus): ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            config.region = Some(input.trim().to_string());

            println!("Note: For Azure, you'll need to set AZURE_TENANT_ID, AZURE_CLIENT_ID, and AZURE_CLIENT_SECRET");
            println!("environment variables for service principal authentication.");
        }
    }

    Ok(config)
}

fn show_configuration(
    provider: &CloudProvider,
    profile: &str,
    configs: &HashMap<String, CloudConfig>,
) -> Result<(), AppError> {
    let key = format!("{}_{}", provider.to_string().to_lowercase(), profile);

    if let Some(config) = configs.get(&key) {
        println!(
            "\n📋 Current configuration for {} (profile: {}):",
            provider.to_string().green(),
            profile.green()
        );
        println!("   Provider: {}", config.provider);

        if let Some(ref api_key) = config.api_key {
            println!("   API Key: {}...", &api_key[..api_key.len().min(8)]);
        }

        if let Some(ref secret_key) = config.secret_key {
            println!(
                "   Secret Key: {}...",
                &secret_key[..secret_key.len().min(8)]
            );
        }

        if let Some(ref region) = config.region {
            println!("   Region: {}", region);
        }

        if let Some(ref project_id) = config.project_id {
            println!("   Project ID: {}", project_id);
        }

        if let Some(ref subscription_id) = config.subscription_id {
            println!("   Subscription ID: {}", subscription_id);
        }
    }

    Ok(())
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

fn load_configs(config_file: &Path) -> Result<HashMap<String, CloudConfig>, AppError> {
    if config_file.exists() {
        let content = fs::read_to_string(config_file)?;
        let configs: HashMap<String, CloudConfig> = serde_json::from_str(&content)?;
        Ok(configs)
    } else {
        Ok(HashMap::new())
    }
}

fn save_configs(
    config_file: &Path,
    configs: &HashMap<String, CloudConfig>,
) -> Result<(), AppError> {
    let content = serde_json::to_string_pretty(configs)?;
    fs::write(config_file, content)?;
    Ok(())
}
