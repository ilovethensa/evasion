use crate::utils;
#[cfg(windows)]
use winreg::enums::*;
#[cfg(windows)]
use winreg::RegKey;
pub fn check() -> bool {
    #[cfg(windows)]
    {
        let uuids = vec![
            "bb926e54-e3ca-40fd-ae90-2764341e7792", // win10 free
            "90059c37-1320-41a4-b58d-2b75a9850d2f", // win7 free
        ];

        let hk_local_machine = RegKey::predef(HKEY_LOCAL_MACHINE);
        let key = hk_local_machine
            .open_subkey("SOFTWARE\\Microsoft\\Cryptography")
            .unwrap();

        if let Ok(uuid) = key.get_value::<String, &str>("MachineGuid") {
            if uuids.contains(&uuid.as_str()) {
                utils::warn!("May be running in anyrun");
                return true;
            }
        }
        utils::warn!("Passed anyrun checks!");
        false
    }

    #[cfg(unix)]
    {
        utils::warn!("Passed anyrun checks!");
        false
    }
}
