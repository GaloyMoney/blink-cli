#!/usr/bin/env bats

load "helpers"

setup_file() {
  galoy_cli_setup
}

@test "pay_btc: sats deducted from sender's wallet" {
  login_user A

  initial_balance=$(get_balance "BTC")
  galoy_cli_cmd pay --username ${USER_B_USERNAME} --wallet btc --sats 100
  final_balance=$(get_balance "BTC")

  [ "$final_balance" -eq "$(($initial_balance - 100))" ] || exit 1

  logout_user
}

