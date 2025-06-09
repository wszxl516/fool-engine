#!/bin/sh
export WINIT_UNIX_BACKEND=x11 
if [ ! -e "./target/debug/assets" ]; then
    ln -sf ../../assets ./target/debug
fi
cargo r -p fool-engine --features=debug -- $@
