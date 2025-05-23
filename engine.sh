#!/bin/sh
export WINIT_UNIX_BACKEND=x11 
cargo r -p engine --features=debug -- $@
