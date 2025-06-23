use crate::cli::{Cli, Commands};
use crate::commands::{ports, process};
use crate::error::AppError;

pub async fn run(cli: Cli) -> Result<(), AppError> {
    match &cli.command {
        Some(Commands::List { processes, ports }) => {
            if *processes {
                process::list_processes().await?;
            }
            if *ports {
                ports::list_ports().await?;
            }
        }
        Some(Commands::Kill { process, port }) => {
            if let Some(process) = process {
                println!("Killing process: {}", process);
            }
            if let Some(port) = port {
                println!("Killing port: {}", port);
            }
        }

        None => {
            println!("No command provided");
        }
    }

    Ok(())
}
