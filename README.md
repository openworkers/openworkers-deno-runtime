# OpenWorkers runtime

OpenWorkers is a runtime for running javascript code in a serverless environment.
It is designed to be used with the [OpenWorkers CLI](https://github.com/openworkers/openworkers-cli).

This repository contains the runtime library.

## Usage

### Build all examples
```bash
cargo build --release --examples
```

### Snapshot the runtime
```bash
cargo run --bin snapshot
```

### Run the demo server 
#### With a new runtime instance for each request
```bash
cargo run --example serve-new -- examples/serve.js
```

#### With the same runtime instance for each request
```bash
cargo run --example serve-same -- examples/serve.js
```

### Execute a scheduled task
```bash
export RUST_LOG=openworkers_runtime=debug,serve=debug # Optional

cargo run --example scheduled -- examples/scheduled.js
```