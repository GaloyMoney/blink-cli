#!/bin/bash

set -eu

mkdir artifacts/binaries

mv x86_64-pc-windows-gnu/* artifacts/binaries
mv x86_64-apple-darwin/* artifacts/binaries
mv x86_64-unknown-linux-musl/* artifacts/binaries

for file in artifacts/binaries/*; do
    if [ -f "$file" ]; then
        sha256sum "$file" > "$file".sha256
    fi
done

