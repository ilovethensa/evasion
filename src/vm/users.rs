#[cfg(windows)]
use std::env;

use crate::utils;
pub fn check() -> bool {
    #[cfg(windows)]
    {
        let blacklisted_names = vec![
            "johnson",
            "miller",
            "malware",
            "maltest",
            "currentuser",
            "sandbox",
            "virus",
            "john doe",
            "test user",
            "sand box",
            "wdagutilityaccount",
            "bruno",
            "george",
            "harry johnson",
        ];

        let username = env::var("USERNAME").unwrap_or_default().to_lowercase();

        return if blacklisted_names.contains(&&username.as_str()) {
            utils::warn!("Cpu count is weird, may be running in a VM");
            true
        } else {
            false
        };
    };
    #[cfg(unix)]
    utils::warn!("No sussy users");
    #[cfg(unix)]
    false
}
