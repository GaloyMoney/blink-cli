#!/bin/bash

set -eu

VERSION=""
if [[ -f version/version ]];then
  VERSION="$(cat version/version)"
fi

REPO=${REPO:-repo}
BINARY=galoy-cli
OUT=${OUT:-none}
WORKSPACE="$(pwd)"

export CARGO_HOME="$(pwd)/cargo-home"
export CARGO_TARGET_DIR="$(pwd)/cargo-target-dir"

[ -f /workspace/.cargo/config ] && cp /workspace/.cargo/config ${CARGO_HOME}/config

pushd ${REPO}

set -x

make build-${TARGET}-release

cd ${CARGO_TARGET_DIR}/${TARGET}/release
OUT_DIR="${BINARY}-${TARGET}-${VERSION}"
rm -rf "${OUT_DIR}" || true
mkdir "${OUT_DIR}"

if [[ $TARGET == "x86_64-pc-windows-gnu" ]]; then
  mv ./${BINARY}.exe ${OUT_DIR}
else
  mv ./${BINARY} ${OUT_DIR}
fi

tar -czvf ${OUT_DIR}.tar.gz ${OUT_DIR}

mv ${OUT_DIR}.tar.gz ${WORKSPACE}/${OUT}/