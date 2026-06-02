use crate::errors::ScannerError;
use ignore::Walk;
use std::path::{Path, PathBuf};

pub fn collect_python_files(root: &Path, test_only: bool) -> Result<Vec<PathBuf>, ScannerError> {
    let mut paths = Vec::new();

    for entry in Walk::new(root) {
        let entry = entry.map_err(ScannerError::ReadDirError)?;
        let path = entry.path();

        if let Err(e) = std::fs::metadata(path) {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("warn: skipping {}, permission denied", path.display());
                continue;
            }
        }

        if test_only {
            let in_test_dir = path
                .components()
                .any(|c| matches!(c.as_os_str().to_str(), Some("test") | Some("tests")));
            if !in_test_dir {
                continue;
            }
        }

        if path.extension().and_then(|s| s.to_str()) == Some("py") {
            paths.push(path.to_owned());
        }
    }
    Ok(paths)
}
