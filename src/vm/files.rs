use crate::utils;
#[cfg(windows)]
use std::env;
#[cfg(windows)]
use std::fs;
#[cfg(windows)]
use std::path::Path;
#[cfg(windows)]
use std::path::PathBuf;
#[cfg(windows)]
const BAD_FILE_NAMES: [&str; 6] = [
    "vboxmouse.sys",
    "vboxguest.sys",
    "vboxsf.sys",
    "vboxvideo.sys",
    "vmmouse.sys",
    "vboxogl.dll",
];
#[cfg(windows)]
const BAD_DIRS: [&str; 2] = [
    r"C:\Program Files\VMware",
    r"C:\Program Files\oracle\virtualbox guest additions",
];
pub fn check() -> bool {
    #[cfg(windows)]
    {
        let system32_folder = PathBuf::from(env::var("SystemRoot").unwrap()).join("System32");

        let files = match fs::read_dir(&system32_folder) {
            Ok(files) => files,
            Err(_) => {
                utils::warn!("Error accessing System32 folder");
                return false;
            }
        };

        for file in files {
            let file = match file {
                Ok(file) => file,
                Err(_) => continue,
            };

            let file_name = file.file_name().to_str().unwrap().to_lowercase();

            for bad_file_name in BAD_FILE_NAMES {
                if file_name == bad_file_name.to_lowercase() {
                    utils::warn!("May be running in a VM!");
                    return true;
                }
            }
        }

        for bad_dir in BAD_DIRS {
            if Path::new(bad_dir).exists() {
                utils::warn!("May be running in a VM: {}", bad_dir);
                return true;
            }
        }
    }
    false
}
