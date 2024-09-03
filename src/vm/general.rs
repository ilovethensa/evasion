use crate::utils;
use sysinfo::System;

pub fn check_cpu(sys: &System) -> bool {
    let cpus = sys.cpus().len();
    if cpus % 2 == 0 || cpus < 2 {
        true
    } else {
        utils::warn!("Cpu count is weird, may be running in a VM");
        false
    }
}
pub fn check_ram(sys: &System) -> bool {
    let total_memory = sys.total_memory();

    if total_memory <= 2 * 0x4000_0000 {
        utils::warn!("[*] Possibly a virtualised environment");
        true
    } else {
        false
    }
}
