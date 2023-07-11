#!/usr/bin/env bats

load "helpers"

setup_file() {
  galoy_cli_setup
  galoy_cli_setup_usernames
}

@test "pay(intraledger, btc): sats deducted from sender's wallet and received by recipient" {
  login_user B
  initial_balance_B=$(get_balance)
  logout_user

  login_user A
  initial_balance_A=$(get_balance "btc")
  
  galoy_cli_cmd pay --username ${USER_B_USERNAME} --wallet btc --sats 100

  final_balance_A=$(get_balance "btc")
  logout_user

  login_user B
  final_balance_B=$(get_balance)
  logout_user

  [ "$final_balance_B" -gt "$initial_balance_B" ] || exit 1
  [ "$final_balance_A" -lt "$initial_balance_A" ] || exit 1
}

@test "pay(intraledger, usd): cents deducted from sender's wallet and received by recipient" {
  login_user A
  initial_balance_A=$(get_balance)
  logout_user

  login_user B
  initial_balance_B=$(get_balance "usd")
  
  galoy_cli_cmd pay --username ${USER_A_USERNAME} --wallet usd --cents 1

  final_balance_B=$(get_balance "usd")
  logout_user

  login_user A
  final_balance_A=$(get_balance)
  logout_user

  [ "$final_balance_A" -gt "$initial_balance_A" ] || exit 1
  [ "$final_balance_B" -lt "$initial_balance_B" ] || exit 1
}
