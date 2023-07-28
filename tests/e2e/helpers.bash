REPO_ROOT=$(git rev-parse --show-toplevel)
BASH_SOURCE=${BASH_SOURCE:-tests/e2e/helpers/.}
source $(dirname "$BASH_SOURCE")/_common.bash

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


login_user() {
  local user=$1

  if [[ "$user" == "A" ]]; then
    galoy_cli_cmd login ${ALICE_PHONE} ${ALICE_CODE}
  elif [[ "$user" == "B" ]]; then
    galoy_cli_cmd login ${BOB_PHONE} ${BOB_CODE}
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
