#!/bin/sh
export RUST_LOG=debug
cargo r -p fool-script --bin fool-script --features=debug -- $@
