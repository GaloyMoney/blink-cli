#!/usr/bin/env bats

load "helpers"

setup_file() {
 
  galoy_cli_setup
}

teardown_file() {
  stop_server
  stop_ws_server
}

@test "galoy-cli: login saves token to home directory" {
  login_user A
  if [[ ! -f ~/.galoy-cli/GALOY_TOKEN ]]; then echo "Token wasn't created"; exit 1; fi
}

@test "galoy-cli: can query me" {
  phone=$(galoy_cli_cmd me | jq -r ".phone")
  [ "$phone" = "${ALICE_PHONE}" ] || exit 1
}

@test "galoy-cli: logout deletes token from home directory" {
  logout_user
  if [[ -f ~/.galoy-cli/GALOY_TOKEN ]]; then echo "Token wasn't deleted"; exit 1; fi
}
