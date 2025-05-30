#!/bin/sh
export WINIT_UNIX_BACKEND=x11 
cargo r -p fool-engine --features=debug -- $@
