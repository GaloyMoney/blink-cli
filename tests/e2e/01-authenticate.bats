#!/usr/bin/env bats

load "helpers"

setup_file() {
  galoy_cli_setup
}

@test "auth: login A using email" {
  login_user_with_email A
  logout_user
}

@test "auth: login/logout A and set username" {
  redis_cli FLUSHALL > /dev/null 2>&1 || true
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

@test "auth: login/logout C and set username" {
  login_user C
  galoy_cli_cmd set-username --username ${USER_C_USERNAME}
  username=$(galoy_cli_cmd me | jq -r '.username')
  [[ "$username" == "${USER_C_USERNAME}" ]] || exit 1
  logout_user
}

