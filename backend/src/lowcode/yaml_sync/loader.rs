use std::path::{Path, PathBuf};

use super::schema::OperationDef;

pub fn discover_yaml_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if !dir.exists() {
        tracing::warn!("Operations directory not found: {:?}", dir);
        return files;
    }
    for entry in walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if (ext == "yaml" || ext == "yml")
                    && !path
                        .file_name()
                        .map(|f| f.to_string_lossy().starts_with('_'))
                        .unwrap_or(false)
                {
                    files.push(path.to_path_buf());
                }
            }
        }
    }
    files.sort();
    files
}

pub fn parse_operation(path: &Path) -> Result<OperationDef, String> {
    let content =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read {:?}: {}", path, e))?;
    serde_yaml::from_str(&content).map_err(|e| format!("Failed to parse {:?}: {}", path, e))
}
