#[cfg(windows)]
use crate::utils;
#[cfg(windows)]
use windows::core::{s, PSTR};
#[cfg(windows)]
use windows::Win32::System::Registry::{
    RegCloseKey, RegOpenKeyExA, RegQueryInfoKeyA, HKEY, HKEY_LOCAL_MACHINE, KEY_READ,
};

#[allow(clippy::missing_const_for_fn)]
pub fn check() -> bool {
    #[cfg(windows)]
    {
        let mut h_key: HKEY = HKEY::default();
        let mut usb_number: u32 = 0;
        let mut class_name_buffer = [0u8; 256];
        let mut class_name_length = class_name_buffer.len() as u32;

        unsafe {
            let status = RegOpenKeyExA(
                HKEY_LOCAL_MACHINE,
                s!("SYSTEM\\ControlSet001\\Enum\\USBSTOR"),
                0,
                KEY_READ,
                &mut h_key,
            );

            if status.is_err() {
                utils::warn!("RegOpenKeyExA Failed");
                return false;
            }

            let status = RegQueryInfoKeyA(
                h_key,
                PSTR(class_name_buffer.as_mut_ptr()),
                Some(&mut class_name_length),
                None,
                Some(&mut usb_number),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            );

            if status.is_err() {
                utils::warn!("RegQueryInfoKeyA Failed");
                return false;
            }

            if usb_number < 2 {
                utils::warn!("[*] Possibly a virtualised environment");
                return true;
            }

            let _ = RegCloseKey(h_key);
            return false;
        }
    }
    #[cfg(unix)]
    false
}
