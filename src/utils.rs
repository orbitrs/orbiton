// Utility functions for orbiton CLI

pub mod fs {
    use anyhow::{Context, Result};
    use log::debug;
    use std::fs;
    use std::path::{Path, PathBuf};

    /// Copy a directory recursively
    #[allow(dead_code)]
    pub fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
        debug!("Copying directory from {src:?} to {dst:?}");

        // Create the destination directory if it doesn't exist
        if !dst.exists() {
            fs::create_dir_all(dst)
                .with_context(|| format!("Failed to create directory: {dst:?}"))?;
        }

        // Walk through the source directory
        for entry in
            fs::read_dir(src).with_context(|| format!("Failed to read directory: {src:?}"))?
        {
            let entry =
                entry.with_context(|| format!("Failed to read directory entry in: {src:?}"))?;

            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if src_path.is_dir() {
                // Recursive call for directories
                copy_dir_recursive(&src_path, &dst_path)?;
            } else {
                // Copy files
                fs::copy(&src_path, &dst_path)
                    .with_context(|| format!("Failed to copy {src_path:?} to {dst_path:?}"))?;
            }
        }

        Ok(())
    }

    /// Find all files with a specific extension
    #[allow(dead_code)]
    pub fn find_files_with_extension(dir: &Path, extension: &str) -> Result<Vec<PathBuf>> {
        let mut result = Vec::new();

        if !dir.exists() || !dir.is_dir() {
            return Ok(result);
        }

        for entry in walkdir::WalkDir::new(dir) {
            let entry =
                entry.with_context(|| format!("Failed to read directory entry in: {dir:?}"))?;

            let path = entry.path();

            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some(extension) {
                result.push(path.to_owned());
            }
        }

        Ok(result)
    }
}

pub mod crypto {
    /// Generate a random identifier
    #[allow(dead_code)]
    pub fn random_id() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos();

        format!("{now:x}")
    }
}
