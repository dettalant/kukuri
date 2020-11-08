use std::fs;
use std::path::Path;

pub fn read_file(path: &Path) -> std::io::Result<String> {
    // TODO: More better file NotFound error print.
    let s = fs::read_to_string(path)?;
    Ok(s)
}

pub fn write_file(path: &Path, s: &str) -> std::io::Result<()> {
    fs::write(path, s)?;
    Ok(())
}
