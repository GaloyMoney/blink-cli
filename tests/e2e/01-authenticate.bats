#!/usr/bin/env bats

load "helpers"

setup_file() {
  galoy_cli_setup
}

@test "auth: login/logout A and set username" {
  login_user A
  galoy_cli_cmd set-username --username ${USER_A_USERNAME}
  username=$(galoy_cli_cmd me | jq -r '.username')
  [[ "$username" -eq "${USER_A_USERNAME}" ]] || exit 1
  logout_user
}

@test "auth: login/logout B and set username" {
  login_user B
  galoy_cli_cmd set-username --username ${USER_B_USERNAME}
  username=$(galoy_cli_cmd me | jq -r '.username')
  [[ "$username" == "${USER_B_USERNAME}" ]] || exit 1
  logout_user
}