use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "actlog",
    about = "A CLI tool to inspect running processes, open ports, and kill misbehaving PIDs or ports from the terminal.",
    version,
    long_about = "A CLI tool to inspect running processes, open ports, and kill misbehaving PIDs or ports from the terminal."
)]
pub struct Cli {
    /// The command to run
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List running processes or open ports
    List {
        /// List processes
        #[arg(long)]
        processes: bool,
        /// List open ports
        #[arg(long)]
        ports: bool,
    },
    /// Kill a process or port
    Kill {
        /// Kill a process by PID
        #[arg(long)]
        process: Option<u32>,
        /// Kill a port by number
        #[arg(short, long)]
        port: Option<u16>,
    },
}
