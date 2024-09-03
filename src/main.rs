#![warn(clippy::restriction, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::single_call_fn)]
#![allow(clippy::implicit_return)]
#![allow(clippy::missing_const_for_fn)]
#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

#[macro_use]
mod utils;
mod vm;

fn main() {
    if let Err(error) = run() {
        utils::warn!("Error: {}", error);
        #[allow(clippy::restriction)]
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    if vm::check() {
        #[cfg(windows)]
        {
            windows::self_delete()
        }
        #[cfg(unix)]
        {
            if let Err(e) = unix::self_delete() {
                return Err(format!("Failed to self-delete on Unix: {e}"));
            }
        }
    } else {
        utils::warn!("not running in a VM");
    }
    Ok(())
}
