#!/bin/sh
if [ ! -e "./target/debug/assets" ]; then
    ln -sf ../../assets ./target/debug
fi
unset WAYLAND_DISPLAY
cargo r -p fool-engine --features=debug -- $@
