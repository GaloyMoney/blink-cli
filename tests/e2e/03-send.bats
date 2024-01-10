#!/usr/bin/env bats

load "helpers"

setup_file() {
  galoy_cli_setup
  bitcoind_setup
}

@test "send(intraledger, btc): sats deducted from A's wallet and received by B" {
  redis_cli FLUSHALL > /dev/null 2>&1 || true
  fund_user "A" "btc" 0.1

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

@test "send(intraledger, usd): cents deducted from B's wallet and received by A" {
  redis_cli FLUSHALL > /dev/null 2>&1 || true
  fund_user "B" "usd" 0.1

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

@test "batch(A, B, C): submit batch from A to B and C" {
  redis_cli FLUSHALL > /dev/null 2>&1 || true
  fund_user "A" "usd" 0.01

  login_user B
  initial_balance_B=$(get_balance)
  logout_user

  login_user C
  initial_balance_C=$(get_balance)
  logout_user

  login_user A

  # payouts.csv sends from USD wallet
  initial_balance_A=$(get_balance "usd")
  
  galoy_cli_cmd batch --csv $(tests_dir)/payouts.csv --skip-confirmation

  final_balance_A=$(get_balance "usd")
  logout_user

  login_user B
  final_balance_B=$(get_balance)
  logout_user

  login_user C
  final_balance_C=$(get_balance)
  logout_user

  [ "$final_balance_B" -gt "$initial_balance_B" ] || exit 1
  [ "$final_balance_C" -gt "$initial_balance_C" ] || exit 1
  [ "$final_balance_A" -lt "$initial_balance_A" ] || exit 1
}
