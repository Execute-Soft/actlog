use crate::error::AppError;
use colored::*;
use sysinfo::System;

pub async fn list_processes() -> Result<(), AppError> {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Print title
    println!("{}", "=".repeat(80).blue());
    println!("{}", "  PROCESS LIST".bold().white().on_blue());
    println!("{}", "=".repeat(80).blue());

    // Print table header with separator
    let header = format!(
        "│ {:<8} │ {:<30} │ {:<12} │ {:<15} │",
        "PID".bold().cyan(),
        "Process Name".bold().cyan(),
        "CPU %".bold().cyan(),
        "Memory".bold().cyan()
    );
    println!("{}", header.blue());

    // Print header separator
    let separator = format!(
        "├{}┼{}┼{}┼{}┤",
        "─".repeat(8),
        "─".repeat(30),
        "─".repeat(12),
        "─".repeat(15)
    );
    println!("{}", separator.blue());

    // Print each process
    for (pid, process) in sys.processes() {
        let cpu_usage = process.cpu_usage();
        let memory_kb = process.memory();
        let memory_mb = memory_kb as f64 / 1024.0;

        // Color code CPU usage
        let cpu_color = if cpu_usage > 50.0 {
            cpu_usage.to_string().red()
        } else if cpu_usage > 20.0 {
            cpu_usage.to_string().yellow()
        } else {
            cpu_usage.to_string().green()
        };

        // Color code memory usage
        let memory_color = if memory_mb > 1000.0 {
            format!("{:.1} MB", memory_mb).red()
        } else if memory_mb > 100.0 {
            format!("{:.1} MB", memory_mb).yellow()
        } else {
            format!("{:.1} MB", memory_mb).green()
        };

        let row = format!(
            "│ {:<8} │ {:<30} │ {:<12} │ {:<15} │",
            pid,
            process.name(),
            cpu_color,
            memory_color
        );
        println!("{}", row.blue());
    }

    // Print bottom border
    let bottom_border = format!(
        "└{}┴{}┴{}┴{}┘",
        "─".repeat(8),
        "─".repeat(30),
        "─".repeat(12),
        "─".repeat(15)
    );
    println!("{}", bottom_border.blue());

    // Print summary
    let total_processes = sys.processes().len();
    println!("{}", "─".repeat(80).blue());
    println!(
        "{}",
        format!("Total Processes: {}", total_processes)
            .bold()
            .white()
            .on_green()
    );
    println!("{}", "─".repeat(80).blue());

    Ok(())
}
