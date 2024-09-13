#!/bin/bash

cargo skyline build --release
cp target/aarch64-skyline-switch/release/libDivineDragonDrifting.nro DivineDragonDrifting/
read