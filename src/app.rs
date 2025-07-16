use crate::cli::{Cli, Commands};
use crate::commands::{
    authenticate, cleanup_resources, configure, list_resources, report_costs, scale_instances,
};
use crate::error::AppError;

pub async fn run(cli: Cli) -> Result<(), AppError> {
    match &cli.command {
        Commands::Authenticate { .. } => {
            authenticate(&cli.command).await?;
        }

        Commands::Config { .. } => {
            configure(&cli.command).await?;
        }

        Commands::ReportCosts { .. } => {
            report_costs(&cli.command).await?;
        }

        Commands::ScaleInstances { .. } => {
            scale_instances(&cli.command).await?;
        }

        Commands::Cleanup { .. } => {
            cleanup_resources(&cli.command).await?;
        }

        Commands::List { .. } => {
            list_resources(&cli.command).await?;
        }
    }

    Ok(())
}
