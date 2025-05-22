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
    // Get output directory for built executable
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let profile = env::var("PROFILE").unwrap();

    // Get the source templates directory
    let templates_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("templates");

    // Calculate target templates directory (parallel to executable)
    let target_templates_dir = out_dir
        .ancestors()
        .find(|p| {
            p.file_name()
                .map_or(false, |name| name.to_string_lossy() == profile)
        })
        .expect("Could not find target directory");

    println!("cargo:rerun-if-changed=templates");

    // Create target templates directory
    fs::create_dir_all(&target_templates_dir).unwrap();

    // Copy templates recursively
    copy_dir_recursively(&templates_dir, &target_templates_dir).unwrap();
}
