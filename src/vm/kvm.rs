#[cfg(unix)]
use crate::utils;
#[cfg(windows)]
use std::env;
#[cfg(unix)]
use std::fs::File;
#[cfg(unix)]
use std::io::{BufRead, BufReader};
#[cfg(windows)]
use std::path::PathBuf;
#[cfg(unix)]
use std::process::Command;

use crate::utils;

#[cfg(windows)]
pub fn check() -> bool {
    let bad_drivers_list = vec![
        "balloon.sys",
        "netkvm.sys",
        "vioinput",
        "viofs.sys",
        "vioser.sys",
    ];
    for driver in bad_drivers_list {
        let system_root = env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string());
        let path = PathBuf::from(system_root).join("System32").join(driver);
        if path.exists() {
            utils::warn!("Bad drivers detected");
            return true;
        }
    }
    false
}
#[cfg(unix)]
pub fn check() -> bool {
    check_cpuinfo() || check_systemd() || check_lscpu()
}
#[cfg(unix)]
fn check_cpuinfo() -> bool {
    let file = match File::open("/proc/cpuinfo") {
        Ok(file) => file,
        Err(_) => return false,
    };

    let reader = BufReader::new(file);
    for line in reader.lines().map_while(Result::ok) {
        if line.starts_with("model name") && line.contains("QEMU") {
            return true;
        }
    }

    false
}
#[cfg(unix)]
fn check_systemd() -> bool {
    let output = match Command::new("systemd-detect-virt").output() {
        Ok(output) => output,
        Err(error) => {
            utils::warn!("Failed to run command: {}", error);
            return false;
        }
    };

    if output.status.success() {
        match String::from_utf8(output.stdout) {
            Ok(output_str) => output_str.trim() != "none",
            Err(error) => {
                utils::warn!("Failed to parse output: {}", error);
                false
            }
        }
    } else {
        false
    }
}
#[cfg(unix)]
fn check_lscpu() -> bool {
    let command = Command::new("lscpu").output();

    match command {
        Ok(output) => {
            if output.status.success() {
                let output_str = String::from_utf8(output.stdout).unwrap_or_else(|_| String::new());
                return if output_str.contains("KVM") || output_str.contains("Virtualization type") {
                    utils::warn!("Lscpu indicates a VM");
                    true
                } else {
                    false
                };
            } else {
                false
            }
        }
        Err(_) => false,
    }
}
