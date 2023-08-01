#!/bin/bash

WINDOWS_TARGET="x86_64-pc-windows-gnu"

echo "Building target for platform ${WINDOWS_TARGET}"
echo

SQLX_OFFLINE=true cargo build --release --target "${WINDOWS_TARGET}" --all-features

echo
echo Done
