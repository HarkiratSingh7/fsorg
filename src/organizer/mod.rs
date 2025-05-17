pub mod configurations;
pub mod engine;
use log::error;
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

const FAIL_CONFIG_FILE: &str = "fsorg.json";

pub fn get_home_dir() -> Option<PathBuf> {
    env::var(if cfg!(target_os = "windows") {
        "USERPROFILE"
    } else {
        "HOME"
    })
    .ok()
    .map(PathBuf::from)
}

fn move_file_safely(from: &Path, to: &Path) -> std::io::Result<()> {
    if let Some(parent_dir) = to.parent() {
        fs::create_dir_all(parent_dir).inspect_err(|e| {
            error!(
                "Failed to create directory {}: {} !",
                parent_dir.display(),
                e
            );
        })?;
    }

    match fs::rename(from, to) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == ErrorKind::CrossesDevices => {
            fs::copy(from, to)?;
            fs::remove_file(from)?;
            Ok(())
        }
        Err(err) => Err(err),
    }
}
