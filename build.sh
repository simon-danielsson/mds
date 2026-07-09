#!/usr/bin/env bash

cd "$HOME/dev/rust/mdp"

cargo build

./target/debug/mdp test.md -o ../hello.html

