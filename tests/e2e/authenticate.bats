#!/usr/bin/env bats

load "helpers"

setup_file() {
  galoy_cli_setup
}

@test "galoy-cli: login saves token to home directory" {
  echo "galoy_cli_cmd login ${USER_A_PHONE} ${USER_A_CODE}"
  galoy_cli_cmd login ${USER_A_PHONE} ${USER_A_CODE}
  if [[ ! -f ~/.galoy-cli/GALOY_TOKEN ]]; then echo "Token wasn't created"; exit 1; fi
}

@test "galoy-cli: can query me" {
  phone=$(galoy_cli_cmd me | jq -r ".phone")
  [ "$phone" = "${USER_A_PHONE}" ] || exit 1
}

@test "galoy-cli: logout deletes token from home directory" {
  galoy_cli_cmd logout
  if [[ -f ~/.galoy-cli/GALOY_TOKEN ]]; then echo "Token wasn't deleted"; exit 1; fi
}
