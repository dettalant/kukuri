use std::fs;
use std::path::Path;

pub fn read_file(path: &Path) -> std::io::Result<String> {
    // TODO: More better file NotFound error print.
    let s = fs::read_to_string(path)?;
    Ok(s)
}
