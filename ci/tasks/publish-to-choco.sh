#!/bin/bash

set -eu

export VERSION=$(cat version/version)
export CHECKSUM=$(curl -L https://github.com/GaloyMoney/galoy-cli/releases/download/${VERSION}/sha256sums.txt | grep galoy-cli-x86_64-pc-windows-gnu | cut -d' ' -f1)

pushd repo/dist/choco

sed -i "7s/<version>.*<\/version>/<version>${VERSION}<\/version>/" galoy-cli.nuspec
sed -i "3s/\$version=.*/\$version='${VERSION}'/" tools/chocolateyinstall.ps1
sed -i "4s/\$checksum=.*/\$checksum='${CHECKSUM}'/" tools/chocolateyinstall.ps1

choco apikey --key ${CHOCO_API_KEY} --source https://push.chocolatey.org/
choco push galoy-cli.${VERSION}.nupkg --source https://push.chocolatey.org/
