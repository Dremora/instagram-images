use std::path::{Path, PathBuf};

pub fn parse_directory_path(input: &str) -> Option<PathBuf> {
    let path = Path::new(input);
    if path.exists() && path.is_dir() {
        Some(path.to_path_buf())
    } else {
        None
    }
}
