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
        let qemu_drivers = vec!["qemu-ga", "qemuwmi"];
        let sys32 = Path::new(&env::var("SystemRoot").unwrap()).join("System32");

        for entry in fs::read_dir(sys32).unwrap() {
            let entry = entry.unwrap();
            for driver in &qemu_drivers {
                if entry.file_name().to_str().unwrap().contains(driver) {
                    utils::warn!("Bad drivers detected, seem to be running in qemu!");
                    return true;
                }
            }
        }
    }
    utils::warn!("Not running in QEMU");
    false
}
