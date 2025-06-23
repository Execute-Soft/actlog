use crate::error::AppError;
use colored::*;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct PortInfo {
    pub protocol: String,
    pub local_address: String,
    pub local_port: u16,
    pub remote_address: Option<String>,
    pub remote_port: Option<u16>,
    pub state: Option<String>,
    pub pid: Option<u32>,
    pub process_name: Option<String>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct PortListOptions {
    pub show_all: bool,
    pub filter_protocol: Option<String>,
    pub filter_port: Option<u16>,
    pub filter_pid: Option<u32>,
    pub sort_by: SortOption,
    pub limit: Option<usize>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum SortOption {
    Port,
    Protocol,
    Process,
    State,
}

impl Default for PortListOptions {
    fn default() -> Self {
        Self {
            show_all: false,
            filter_protocol: None,
            filter_port: None,
            filter_pid: None,
            sort_by: SortOption::Port,
            limit: None,
        }
    }
}

pub async fn list_ports() -> Result<(), AppError> {
    list_ports_with_options(PortListOptions::default()).await
}

pub async fn list_ports_with_options(options: PortListOptions) -> Result<(), AppError> {
    let ports = gather_ports_info().await?;
    let filtered_ports = filter_ports(ports, &options);
    let sorted_ports = sort_ports(filtered_ports, &options.sort_by);
    let final_ports = if let Some(limit) = options.limit {
        sorted_ports.into_iter().take(limit).collect()
    } else {
        sorted_ports
    };

    display_ports(&final_ports);
    Ok(())
}

async fn gather_ports_info() -> Result<Vec<PortInfo>, AppError> {
    let mut ports = Vec::new();

    // Get TCP ports using lsof
    let tcp_ports = get_tcp_ports().await?;
    ports.extend(tcp_ports);

    // Get UDP ports using lsof
    let udp_ports = get_udp_ports().await?;
    ports.extend(udp_ports);

    Ok(ports)
}

async fn get_tcp_ports() -> Result<Vec<PortInfo>, AppError> {
    let output = Command::new("lsof")
        .args(&["-i", "tcp", "-P", "-n"])
        .output()
        .map_err(|e| AppError::operation(format!("Failed to execute lsof: {}", e)))?;

    if !output.status.success() {
        return Err(AppError::operation("lsof command failed".to_string()));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut ports = Vec::new();

    for line in output_str.lines().skip(1) {
        // Skip header
        if let Some(port_info) = parse_lsof_line(line, "TCP") {
            ports.push(port_info);
        }
    }

    Ok(ports)
}

async fn get_udp_ports() -> Result<Vec<PortInfo>, AppError> {
    let output = Command::new("lsof")
        .args(&["-i", "udp", "-P", "-n"])
        .output()
        .map_err(|e| AppError::operation(format!("Failed to execute lsof: {}", e)))?;

    if !output.status.success() {
        return Err(AppError::operation("lsof command failed".to_string()));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut ports = Vec::new();

    for line in output_str.lines().skip(1) {
        // Skip header
        if let Some(port_info) = parse_lsof_line(line, "UDP") {
            ports.push(port_info);
        }
    }

    Ok(ports)
}

fn parse_lsof_line(line: &str, protocol: &str) -> Option<PortInfo> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 9 {
        return None;
    }

    // lsof output format: COMMAND PID USER FD TYPE DEVICE SIZE/OFF NODE NAME
    let command = parts[0];
    let pid_str = parts[1];
    let name_part = parts[8];

    let pid: u32 = pid_str.parse().ok()?;

    // Parse NAME field (e.g., "localhost:8080" or "*:8080")
    let (local_addr, local_port) = parse_address_port(name_part)?;

    Some(PortInfo {
        protocol: protocol.to_string(),
        local_address: local_addr,
        local_port,
        remote_address: None,
        remote_port: None,
        state: if protocol == "TCP" {
            Some("LISTEN".to_string())
        } else {
            None
        },
        pid: Some(pid),
        process_name: Some(command.to_string()),
    })
}

fn parse_address_port(addr_port: &str) -> Option<(String, u16)> {
    if addr_port == "*" {
        return Some(("*".to_string(), 0));
    }

    let parts: Vec<&str> = addr_port.split(':').collect();
    if parts.len() != 2 {
        return None;
    }

    let addr = parts[0];
    let port_str = parts[1];

    let port: u16 = port_str.parse().ok()?;

    let formatted_addr = if addr == "*" {
        "*".to_string()
    } else {
        addr.to_string()
    };

    Some((formatted_addr, port))
}

fn filter_ports(ports: Vec<PortInfo>, options: &PortListOptions) -> Vec<PortInfo> {
    ports
        .into_iter()
        .filter(|port| {
            // Filter by protocol
            if let Some(ref protocol) = options.filter_protocol {
                if !port.protocol.eq_ignore_ascii_case(protocol) {
                    return false;
                }
            }

            // Filter by port
            if let Some(filter_port) = options.filter_port {
                if port.local_port != filter_port {
                    return false;
                }
            }

            // Filter by PID
            if let Some(filter_pid) = options.filter_pid {
                if port.pid != Some(filter_pid) {
                    return false;
                }
            }

            true
        })
        .collect()
}

fn sort_ports(mut ports: Vec<PortInfo>, sort_by: &SortOption) -> Vec<PortInfo> {
    match sort_by {
        SortOption::Port => {
            ports.sort_by_key(|p| p.local_port);
        }
        SortOption::Protocol => {
            ports.sort_by(|a, b| a.protocol.cmp(&b.protocol));
        }
        SortOption::Process => {
            ports.sort_by(|a, b| {
                let a_name = a.process_name.as_deref().unwrap_or("");
                let b_name = b.process_name.as_deref().unwrap_or("");
                a_name.cmp(b_name)
            });
        }
        SortOption::State => {
            ports.sort_by(|a, b| {
                let a_state = a.state.as_deref().unwrap_or("");
                let b_state = b.state.as_deref().unwrap_or("");
                a_state.cmp(b_state)
            });
        }
    }
    ports
}

fn display_ports(ports: &[PortInfo]) {
    if ports.is_empty() {
        println!("{}", "No open ports found.".yellow());
        return;
    }

    // Header
    println!("\n{}", "🌐 OPEN PORTS".bold().cyan());
    println!("{}", "=".repeat(80).cyan());

    // Table header
    println!(
        "{:<8} {:<15} {:<8} {:<20} {:<8} {:<12} {:<15}",
        "PROTOCOL".bold(),
        "LOCAL ADDRESS".bold(),
        "PORT".bold(),
        "REMOTE ADDRESS".bold(),
        "REMOTE".bold(),
        "STATE".bold(),
        "PROCESS".bold()
    );
    println!("{}", "-".repeat(80).cyan());

    // Port entries
    for port in ports {
        let protocol = match port.protocol.as_str() {
            "TCP" => port.protocol.green(),
            "UDP" => port.protocol.blue(),
            _ => port.protocol.normal(),
        };

        let local_addr = if port.local_address == "*" {
            "0.0.0.0".dimmed()
        } else {
            port.local_address.normal()
        };

        let port_num = if port.local_port < 1024 {
            port.local_port.to_string().red()
        } else {
            port.local_port.to_string().normal()
        };

        let remote_addr = port.remote_address.as_deref().unwrap_or("-").dimmed();
        let remote_port = port
            .remote_port
            .map(|p| p.to_string())
            .unwrap_or("-".to_string())
            .dimmed();

        let state = port.state.as_deref().unwrap_or("-");
        let state_colored = match state {
            "LISTEN" => state.green(),
            "ESTABLISHED" => state.blue(),
            "TIME_WAIT" => state.yellow(),
            "CLOSE_WAIT" => state.red(),
            _ => state.normal(),
        };

        let process = port.process_name.as_deref().unwrap_or("-");
        let process_colored = if process != "-" {
            process.cyan()
        } else {
            process.normal()
        };

        let pid_info = if let Some(pid) = port.pid {
            format!("{} ({})", process_colored, pid.to_string().dimmed())
        } else {
            process_colored.to_string()
        };

        println!(
            "{:<8} {:<15} {:<8} {:<20} {:<8} {:<12} {:<15}",
            protocol, local_addr, port_num, remote_addr, remote_port, state_colored, pid_info
        );
    }

    // Summary
    println!("{}", "=".repeat(80).cyan());
    println!(
        "{} {} open ports",
        "📊".cyan(),
        ports.len().to_string().bold()
    );

    // Statistics
    let tcp_count = ports.iter().filter(|p| p.protocol == "TCP").count();
    let udp_count = ports.iter().filter(|p| p.protocol == "UDP").count();
    let listening_count = ports
        .iter()
        .filter(|p| p.state.as_deref() == Some("LISTEN"))
        .count();

    println!(
        "{} TCP: {}, UDP: {}, Listening: {}",
        "📈".cyan(),
        tcp_count.to_string().green(),
        udp_count.to_string().blue(),
        listening_count.to_string().yellow()
    );
}
