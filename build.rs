// build.rs
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let project_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let vcpkg_bin = project_root.join("vcpkg_installed/x64-windows/bin");
    let target_dir = project_root.join("target/debug");
    println!("cargo-debug:vcpkg_bin - {:?}",vcpkg_bin);

    if vcpkg_bin.exists() {
        for entry in fs::read_dir(&vcpkg_bin).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "dll" {
                    let filename = path.file_name().unwrap();
                    let dest = target_dir.join(filename);
                    fs::copy(&path, &dest).expect("Failed to copy DLL");
                    println!("Copied {:?}", filename);
                }
            }
        }
    } else {
        panic!("vcpkg bin directory not found: {:?}", vcpkg_bin);
    }
}