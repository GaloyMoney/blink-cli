#!/bin/bash

set -e

pushd repo

cat <<EOF | cargo login
${CRATES_API_TOKEN}
EOF

cargo publish -p galoy-cli --all-features --no-verify
