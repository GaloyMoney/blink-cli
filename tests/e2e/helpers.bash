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
