#!/bin/bash

set -eu

mkdir artifacts/binaries

mv x86_64-pc-windows-gnu/* artifacts/binaries
mv x86_64-apple-darwin/* artifacts/binaries
mv x86_64-unknown-linux-musl/* artifacts/binaries

cd artifacts/binaries
sha256sum * > sha256sums.txt
