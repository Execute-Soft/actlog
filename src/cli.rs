use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "actlog")]
#[command(
    about = "⚙️ A Rust-based CLI tool to inspect running processes, open ports, and kill misbehaving PIDs or ports from the terminal."
)]
#[command(version)]
pub struct Cli {
    /// Show all available features
    #[arg(short, long)]
    pub features: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    List {
        /// List all running processes
        #[arg(short, long)]
        processes: bool,

        /// List all open ports
        #[arg(short, long)]
        ports: bool,

        /// Kill a process by PID
        #[arg(short, long)]
        kill: bool,

        /// Kill a port by port number
        #[arg(short, long)]
        kill_port: bool,

        /// Show help
        #[arg(short, long)]
        help: bool,

        /// Show version
        #[arg(short, long)]
        version: bool,

        /// Show usage
        #[arg(short, long)]
        usage: bool,
    },
}
