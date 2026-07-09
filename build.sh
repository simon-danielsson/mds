#!/usr/bin/env bash

PRG="mds"

cd "$HOME/dev/rust/$PRG"

cargo build

./target/debug/$PRG test.md -o ../hello.html

