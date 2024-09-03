#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)] {
            let message = format!($($arg)*);
            println!("[*] {}", message);
        }

    }
}
pub use crate::warn;
