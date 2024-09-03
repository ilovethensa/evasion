use std::env;
use std::fs;
use std::io;

/// This function deletes the executable while it's running.
#[allow(clippy::single_call_fn)]
pub fn self_delete() -> io::Result<()> {
    let exe = match env::current_exe() {
        Ok(path) => path,
        Err(err) => return Err(err),
    };

    let exe_concat = match exe.canonicalize() {
        Ok(path) => path,
        Err(err) => return Err(err),
    };

    match fs::remove_file(exe_concat) {
        Ok(()) => Ok(()),
        Err(err) => Err(err),
    }
}
