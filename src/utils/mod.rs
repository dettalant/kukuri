use std::fs;
use std::path::{Path, PathBuf};

pub fn read_file(path: &Path) -> std::io::Result<String> {
    // TODO: More better file NotFound error print.
    let s = fs::read_to_string(path)?;
    Ok(s)
}

pub fn write_file(path: &Path, s: &str) -> std::io::Result<()> {
    fs::write(path, s)?;
    Ok(())
}

pub fn cargo_manifest_dir() -> PathBuf {
    match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(s) => PathBuf::from(s),
        Err(e) => panic!("get_cargo_manifest_dir: Unable to get CARGO_MANIFEST_DIR: {:?}", e),
    }
}
