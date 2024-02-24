# OpenWorkers runtime

OpenWorkers is a runtime for running javascript code in a serverless environment.
It is designed to be used with the [OpenWorkers CLI](https://github.com/openworkers/openworkers-cli).

This repository contains the runtime library.

## Usage

### Run demo server

```bash
export RUST_LOG=openworkers_runtime=debug,serve=debug # Optional

cargo run --example serve -- examples/hello.js
```
