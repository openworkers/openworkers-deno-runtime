use std::env;
use std::fs::File;
use std::path::PathBuf;

const RUNTIME_SNAPSHOT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/target/RUNTIME_SNAPSHOT.bin");

fn main () {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/ext/*");
    println!("cargo:rerun-if-changed=src/runtime.rs");
    println!("cargo:rerun-if-changed=src/extensions.rs");
    
    let path = PathBuf::from(RUNTIME_SNAPSHOT_PATH);

    // Create the file if it doesn't exist
    if !path.exists() {
        File::create(&path).unwrap();
    }
}