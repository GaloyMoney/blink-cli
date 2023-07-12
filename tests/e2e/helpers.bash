REPO_ROOT=$(git rev-parse --show-toplevel)

galoy_cli_cmd() {
  galoy_cli_location=${REPO_ROOT}/target/debug/galoy-cli
  if [[ ! -z ${CARGO_TARGET_DIR} ]] ; then
    galoy_cli_location=${CARGO_TARGET_DIR}/debug/galoy-cli
  fi

  ${galoy_cli_location} $@
}

galoy_cli_setup() {
  rm ~/.galoy-cli/GALOY_TOKEN || true
}

galoy_cli_setup_usernames() {
  login_user A
  galoy_cli_cmd set-username --username ${USER_A_USERNAME} || true
  logout_user

  login_user B
  galoy_cli_cmd set-username --username ${USER_B_USERNAME} || true
  logout_user
}

login_user() {
  local user=$1

  if [[ "$user" == "A" ]]; then
    galoy_cli_cmd login ${USER_A_PHONE} ${USER_A_CODE}
  elif [[ "$user" == "B" ]]; then
    galoy_cli_cmd login ${USER_B_PHONE} ${USER_B_CODE}
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
