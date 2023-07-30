#!/bin/bash

VERSION="$(cat version/version)-dev"

pushd repo

sed -i'' "s/^version.*/version = \"${VERSION}\"/" Cargo.toml
cargo check # Generates the Cargo.lock file version update

if [[ -z $(git config --global user.email) ]]; then
  git config --global user.email "bot@galoy.io"
fi
if [[ -z $(git config --global user.name) ]]; then
  git config --global user.name "CI Bot"
fi

git status
git add -A

if [[ "$(git status -s -uno)" != ""  ]]; then
  git commit -m "ci(dev): set version to ${VERSION}"
fi
