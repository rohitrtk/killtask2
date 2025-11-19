mod util;

use std::process::{Command, ExitCode, Stdio};
use std::collections::HashSet;
use util::prettify_strings;
use colored::*;

fn get_ports_from_args() -> Option<Vec<u16>> {
    use std::env;
    
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("Usage: taskkill2 <port1> <port2> <port3> ...");
        return None;
    }

    let mut ports = HashSet::new();
    let mut unique_ports = vec![];
    for arg in args {
        match arg.parse::<u16>() {
            Ok(p) => {
                if ports.insert(p) {
                    unique_ports.push(p);
                }
            },
            Err(_) => {
                eprintln!("{}", format!("Invalid port number supplied: {}", arg).red());
                return None;
            }
        }
    };

    Some(unique_ports)
}

fn find_pids(ports: Vec<u16>) -> HashSet<u32> {
    let output = Command::new("netstat")
        .args(&["-ano"])
        .stdout(Stdio::piped())
        .output()
        .expect(&"Failed to run netstat".red());

    let stdout = str::from_utf8(&output.stdout)
        .expect(&"Unable to parse netstat output".red());

    let mut pids = HashSet::new();
    for port in ports {
        for line in stdout.lines() {
            let line = line.trim();
    
            if line.contains(&format!(":{} ", port)) && line.contains("LISTENING") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(pid_str) = parts.last() {
                    if let Ok(pid) = pid_str.parse::<u32>() {
                        pids.insert(pid);
                    }
                }
            }
        }
    }

    pids
}

fn kill_pids(pids: HashSet<u32>) -> Result<(), String>{
    for pid in pids {
        println!("{}", format!("Attempting to kill PID {}", pid).yellow());

        let output = Command::new("taskkill")
            .args(&["/F", "/PID", &pid.to_string()])
            .stdout(Stdio::null())
            .output()
            .expect(&"Failed to run taskkill".red());

        let stderr_str = String::from_utf8_lossy(&output.stderr);

        if output.status.success() {
            println!("{}", format!("Successfully killed PID {}", pid).green());
        } else {
            let mut err = stderr_str.trim().to_string();
            if let Some(stripped) = err.strip_prefix("ERROR: ") {
                err = stripped.to_string();
            }

            println!("{}", format!("Failed to kill PID {}: {}", pid, err).red());
        }
    }

    Ok(())
}

fn main() -> std::process::ExitCode {
    let ports = match get_ports_from_args() {
        Some(ports) => ports,
        None => {
            return ExitCode::SUCCESS;
        }
    };

    let formatted_ports: String = prettify_strings(ports
            .clone()
            .into_iter()
            .collect::<Vec<u16>>()
            .into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
        );

    println!("{}", format!("Searching for processes listening on ports: {}", formatted_ports).yellow());

    let pids = find_pids(ports);
    if pids.is_empty() {
        println!("{}", format!("Found no processes running on the given ports.").yellow());
        return ExitCode::SUCCESS;
    }

    println!("Found PIDs: {:?}", pids);

    let _ = kill_pids(pids);

    ExitCode::SUCCESS
}