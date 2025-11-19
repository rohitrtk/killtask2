use std::process::{Command, Stdio};
use std::str;
use std::env;
use colored::*;
use std::collections::HashSet;

fn prettify_strings(strings: Vec<String>) -> String {
    if strings.len() == 0 {
        return "".to_string();
    } else if strings.len() == 1 {
        return format!("{}", strings[0]);
    } else if strings.len() == 2 {
        return format!("{} and {}", strings[0], strings[1]);
    } else {
        let before_last = &strings[0..strings.len() - 1].join(", ");
        let last = strings.last().unwrap();
        return format!("{}, and {}", before_last, last);
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("Usage: taskkill2 <port1> <port2> <port3> ...");
        return;
    }

    // Preserving the order ports were passed in
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
                return;
            }
        }
    };

    let formatted_ports: String = prettify_strings(unique_ports
            .clone()
            .into_iter()
            .collect::<Vec<u16>>()
            .into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
        );

    println!("{}", format!("Searching for processes listening on ports: {}", formatted_ports).yellow());

    let output = Command::new("netstat")
        .args(&["-ano"])
        .stdout(Stdio::piped())
        .output()
        .expect(&"Failed to run netstat".red());

    let stdout = str::from_utf8(&output.stdout)
        .expect(&"Unable to parse netstat output".red());

    let mut pids = HashSet::new();
    for port in unique_ports {
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

    if pids.is_empty() {
        println!("{}", format!("Found no processes running on the given ports.").yellow());
        return;
    }

    println!("Found PIDs: {:?}", pids);

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
}
