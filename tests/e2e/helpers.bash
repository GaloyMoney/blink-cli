REPO_ROOT=$(git rev-parse --show-toplevel)

USER_A_PHONE="+16505554321"
USER_A_CODE="000000"
USER_A_USERNAME="alice"
USER_B_PHONE="+16505554322"
USER_B_CODE="000000"
USER_B_USERNAME="bob"
USER_C_PHONE="+16505554323"
USER_C_CODE="000000"
USER_C_USERNAME="charlie"

galoy_cli_cmd() {
  galoy_cli_location=${REPO_ROOT}/target/debug/galoy-cli
  if [[ ! -z ${CARGO_TARGET_DIR} ]] ; then
    galoy_cli_location=${CARGO_TARGET_DIR}/debug/galoy-cli
  fi

  ${galoy_cli_location} $@
}

tests_dir() {
  echo ${REPO_ROOT}/tests/e2e
}

bitcoin_cli_cmd() {
  docker compose exec bitcoind bitcoin-cli -regtest $@
}

galoy_cli_setup() {
  rm ~/.galoy-cli/GALOY_TOKEN || true
  local retries=0
  while [[ $retries -lt 30 ]]; do
    if galoy_cli_cmd globals; then break; fi
    sleep 1
    retries=$((retries+1))
  done
}

bitcoind_setup() {
  bitcoin_cli_cmd createwallet "galoycli" || true
  bitcoin_cli_cmd -generate 101
  sleep 10
}

login_user() {
  local user=$1

  if [[ "$user" == "A" ]]; then
    galoy_cli_cmd login ${USER_A_PHONE} ${USER_A_CODE}
  elif [[ "$user" == "B" ]]; then
    galoy_cli_cmd login ${USER_B_PHONE} ${USER_B_CODE}
  elif [[ "$user" == "C" ]]; then
    galoy_cli_cmd login ${USER_C_PHONE} ${USER_C_CODE}
  else
    echo "Invalid user: $user"
    exit 1
  fi
}

logout_user() {
  galoy_cli_cmd logout
}

get_balance() {
  local wallet_type=$1
  local response
  if [[ -z "$wallet_type" ]]; then
    response=$(galoy_cli_cmd balance)
    echo $response | jq -r 'map(select(.default == true)) | .[0] | .balance'
  else
    response=$(galoy_cli_cmd balance --$wallet_type)
    echo $response | jq -r '.[0] | .balance'
  fi
}

get_default_wallet_id() {
  echo $(galoy_cli_cmd me | jq -r '.defaultAccount.defaultWalletId')
}

fund_user() {
  local user=$1
  local wallet_type=$2
  local btc_amount=$3

  login_user $user
  
  start_balance=$(get_balance $wallet_type)
  
  galoy_cli_cmd receive --wallet $wallet_type --via onchain
  btc_address=$(galoy_cli_cmd receive --wallet $wallet_type --via onchain | jq -r '.address')

  bitcoin_cli_cmd sendtoaddress "$btc_address" $btc_amount
  bitcoin_cli_cmd -generate 10

  local retries=0
  while [[ $retries -lt 30 ]]; do
    final_balance=$(get_balance $wallet_type)
    if [ $final_balance -gt $start_balance ]; then break; fi
    sleep 5
    retries=$((retries+1))
  done

  logout_user
  [[ "$retries" != "30" ]] || exit 1
}
