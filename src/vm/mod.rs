use sysinfo::System;

mod anyrun;
mod files;
mod general;
mod gpu;
mod kvm;
mod parallels;
mod qemu;
mod usb;
mod users;
pub fn check() -> bool {
    let sys = System::new_all();
    general::check_cpu(&sys)
        || general::check_ram(&sys)
        || kvm::check()
        || parallels::check()
        || qemu::check()
        || files::check()
        || gpu::check()
        || files::check()
        || users::check()
        || anyrun::check()
        || usb::check()
}
