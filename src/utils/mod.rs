use std::fs;
use std::io::Result;
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
    // TODO: More better file NotFound error print.
    fs::read_to_string(path)
}

pub fn write_file<P: AsRef<Path>>(path: P, s: &str) -> Result<()> {
    fs::write(path, s)
}

pub fn mkdir_recursive<P: AsRef<Path>>(path: P) -> Result<()> {
    fs::create_dir_all(path)
}

// pub fn cargo_manifest_dir() -> PathBuf {
//     match std::env::var("CARGO_MANIFEST_DIR") {
//         Ok(s) => PathBuf::from(s),
//         Err(e) => panic!("get_cargo_manifest_dir: Unable to get CARGO_MANIFEST_DIR: {:?}", e),
//     }
// }
