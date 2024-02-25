
use deno_core::snapshot::create_snapshot;
use deno_core::snapshot::CreateSnapshotOptions;
use deno_core::snapshot::SnapshotFileSerializer;

use crate::extensions;

use std::env;
use std::path::PathBuf;
use std::fs::File;

const RUNTIME_SNAPSHOT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/RUNTIME_SNAPSHOT.bin");

pub fn create_runtime_snapshot() {
    println!("Building snapshot");

    // Build the file path to the snapshot.
    let snapshot_path = PathBuf::from(RUNTIME_SNAPSHOT_PATH);

    let serializer: SnapshotFileSerializer = SnapshotFileSerializer::new(File::create(snapshot_path).unwrap());

    let options: CreateSnapshotOptions<File> = CreateSnapshotOptions {
        cargo_manifest_dir: env!("CARGO_MANIFEST_DIR"),
        startup_snapshot: None,
        extensions: extensions(false),
        skip_op_registration: false,
        serializer: Box::new(serializer),
        with_runtime_cb: None,
    };

    // Create the snapshot.
    let res = create_snapshot(options, None);

    let output  = res.unwrap();

    let file = output.output;

    println!("Snapshot created: {:?}", file);
}
