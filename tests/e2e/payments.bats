#!/usr/bin/env bats

load "helpers"

setup_file() {
  galoy_cli_setup
}

@test "pay-btc: sats deducted from sender's wallet and received by recipient" {
  login_user B
  initial_balance_B=$(get_balance "BTC")
  logout_user

  login_user A
  initial_balance_A=$(get_balance "BTC")
  
  galoy_cli_cmd pay --username ${USER_B_USERNAME} --wallet btc --sats 100

  final_balance_A=$(get_balance "BTC")
  logout_user

  login_user B
  final_balance_B=$(get_balance "BTC")
  logout_user

  [ "$final_balance_B" -eq "$(($initial_balance_B + 100))" ] || exit 1
  [ "$final_balance_A" -eq "$(($initial_balance_A - 100))" ] || exit 1
}