#[cfg(windows)]
use std::process::Command;
#[cfg(windows)]
use std::process::Stdio;

use crate::utils;
pub fn check() -> bool {
    #[cfg(windows)]
    {
        let output = Command::new("wmic")
            .args(&["path", "win32_VideoController", "get", "name"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        return match output {
            Ok(output) => {
                let gpu_str = std::str::from_utf8(&output.stdout).unwrap().to_lowercase();
                return if gpu_str.contains("virtualbox") || gpu_str.contains("vmware") {
                    utils::warn!("GPU is weird, may be running in a VM");
                    true
                } else {
                    false
                };
            }
            Err(_) => false,
        };
    }
    #[cfg(unix)]
    return false;
}
