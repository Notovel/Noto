use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());

    // Path to the WASM app directory
    let wasm_app_dir = Path::new(&manifest_dir).join("vel-web");

    // Tell Cargo to rerun this build script if WASM sources change
    println!("cargo:rerun-if-changed=vel-web/src");
    println!("cargo:rerun-if-changed=vel-web/Cargo.toml");

    // Check if wasm-pack is available
    let wasm_pack_check = Command::new("wasm-pack").arg("--version").output();

    if wasm_pack_check.is_err() {
        panic!("wasm-pack is not installed. Install it with: cargo install wasm-pack");
    }

    println!("Building WASM app in {} mode...", profile);

    // Determine build mode based on profile
    let build_mode = if profile != "release" {
        "--dev"
    } else {
        "--release"
    };

    println!("Building vel-web from folder: {}", wasm_app_dir.display());
    // Build the WASM app
    let output = Command::new("wasm-pack")
        .args(&[
            "build",
            "--target",
            "web",
            "--out-dir",
            "pkg",
            build_mode,
            "--no-typescript", // Skip TypeScript definitions for simpler setup
        ])
        .current_dir(&wasm_app_dir)
        .output()
        .expect("Failed to execute wasm-pack");

    if !output.status.success() {
        panic!(
            "wasm-pack build failed:\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!("WASM app built successfully!");

    // Set environment variable with the pkg directory path
    let pkg_dir = wasm_app_dir.join("pkg");
    if pkg_dir.exists() {
        println!("cargo:rustc-env=WASM_PKG_DIR={}", pkg_dir.display());
    }

    // Optional: Copy static files to target directory for easier deployment
    let target_dir = Path::new(&manifest_dir).join("target").join(profile);
    let static_dir = target_dir.join("static");

    if let Err(e) = fs::create_dir_all(&static_dir) {
        println!("cargo:warning=Failed to create static directory: {}", e);
    } else {
        // Copy WASM files to target/static for easier serving
        if let Err(e) = copy_dir_all(&pkg_dir, &static_dir) {
            println!("cargo:warning=Failed to copy WASM files: {}", e);
        }
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if path.is_dir() {
            copy_dir_all(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}
