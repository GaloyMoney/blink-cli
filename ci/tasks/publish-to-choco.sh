#!/bin/bash

set -eu

choco apikey --key ${CHOCO_API_KEY} --source https://push.chocolatey.org/

pushd repo/dist/choco

# TODO
