REPO_ROOT=$(git rev-parse --show-toplevel)
GALOY_ENDPOINT="http://localhost:4455/graphql"


gql_file() {
  echo "${REPO_ROOT}/vendor/galoy-quickstart/galoy/test/bats/gql/$1.gql"
}

gql_query() {
  cat "$(gql_file $1)" | tr '\n' ' ' | sed 's/"/\\"/g'
}

random_uuid() {
  if [[ -e /proc/sys/kernel/random/uuid ]]; then
    cat /proc/sys/kernel/random/uuid
  else
    uuidgen
  fi
}

exec_graphql() {
  local token=$1
  local query_name=$2
  local variables=${3:-"{}"}
  local output=${4:-"."}

  if [[ ${token} == "anon" ]]; then
    AUTH_HEADER=""
  else
    AUTH_HEADER="Authorization: Bearer ${token}"
  fi

  curl -s \
    -X POST \
    ${AUTH_HEADER:+ -H "$AUTH_HEADER"} \
    -H "Content-Type: application/json" \
    -H "X-Idempotency-Key: $(random_uuid)" \
    -d "{\"query\": \"$(gql_query $query_name)\", \"variables\": $variables}" \
    "${GALOY_ENDPOINT}" | jq -r "${output}"
}
