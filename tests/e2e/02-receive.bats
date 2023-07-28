#!/usr/bin/env bats

load "helpers"

setup_file() {
  galoy_cli_setup
  bitcoind_setup
}

@test "receive(onchain): A on BTC" {
  fund_user "A" "btc" 0.1
}

@test "receive(onchain): A on USD" {
  fund_user "A" "usd" 0.1
}
