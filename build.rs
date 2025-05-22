use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn copy_dir_recursively(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_recursively(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

fn main() {
    println!("cargo:rerun-if-changed=templates");
    
    // Get output directory for built executable
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let profile = env::var("PROFILE").unwrap();

    // Get the source templates directory
    let templates_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("templates");
    
    // Check if templates directory exists
    if !templates_dir.exists() {
        println!("cargo:warning=Templates directory does not exist: {:?}", templates_dir);
        // In CI environment, we might not have all dependencies
        // Just exit gracefully if running with CI feature
        if cfg!(feature = "ci") {
            println!("cargo:warning=Running in CI mode, skipping template copying");
            return;
        }
    }

    // Calculate target templates directory (parallel to executable)
    let target_templates_dir = match out_dir
        .ancestors()
        .find(|p| {
            p.file_name()
                .is_some_and(|name| name.to_string_lossy() == profile)
        }) {
            Some(dir) => dir.to_path_buf(),
            None => {
                println!("cargo:warning=Could not find target directory, using OUT_DIR");
                out_dir
            }
        };

    // Create target templates directory
    match fs::create_dir_all(&target_templates_dir) {
        Ok(_) => println!("cargo:warning=Created templates directory: {:?}", target_templates_dir),
        Err(e) => println!("cargo:warning=Failed to create templates directory: {}", e),
    }

    // Only copy templates if the source directory exists
    if templates_dir.exists() {
        // Copy templates recursively
        if let Err(e) = copy_dir_recursively(&templates_dir, &target_templates_dir) {
            println!("cargo:warning=Failed to copy templates: {}", e);
        }
    }
}
