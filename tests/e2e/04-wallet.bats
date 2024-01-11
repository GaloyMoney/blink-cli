#!/usr/bin/env bats

load "helpers"

setup_file() {
  galoy_cli_setup
}

@test "wallet: update default wallet" {
  redis_cli FLUSHALL > /dev/null 2>&1 || true
  login_user A

  default_wallet_id=$(get_default_wallet_id)

  balances=$(galoy_cli_cmd balance)
  non_default_wallet_id=$(echo "$balances" | jq -r 'map(select(.default == false)) | .[0].id')
  galoy_cli_cmd set-default-wallet --wallet-id "$non_default_wallet_id"

  updated_default_wallet_id=$(get_default_wallet_id)

  [ "$updated_default_wallet_id" = "$non_default_wallet_id" ] || exit 1

  logout_user
}


