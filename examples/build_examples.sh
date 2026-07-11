#!/usr/bin/env bash

set -xe

cargo build --release

for file in *.md; do
  ../target/release/mds "$file"
done
