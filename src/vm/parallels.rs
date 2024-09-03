#[cfg(windows)]
use std::env;
#[cfg(windows)]
use std::fs;
#[cfg(windows)]
use std::path::Path;

use crate::utils;
pub fn check() -> bool {
    #[cfg(windows)]
    {
        let parallels_drivers = ["prl_sf", "prl_tg", "prl_eth"];
        let sys32fold = Path::new(&env::var("SystemRoot").unwrap()).join("System32");

        match fs::read_dir(sys32fold) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            for driver in &parallels_drivers {
                                if entry.file_name().to_str().unwrap().contains(driver) {
                                    utils::warn!(
                                        "Bad drivers detected, seem to be running in parallels"
                                    );
                                    return true;
                                }
                            }
                        }
                        Err(_) => return false,
                    }
                }
            }
            Err(_) => return false,
        }
    }
    utils::warn!("Not running in parallels");
    false
}
