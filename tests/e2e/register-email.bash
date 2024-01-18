#!/bin/bash

GRAPHQL_ENDPOINT="http://localhost:4455/graphql"
AUTH_ENDPOINT="http://localhost:4455/auth/phone/login"
EMAIL_REG_ENDPOINT="http://localhost:4455/auth/email/code"
DB_CONTAINER="galoy-cli-kratos-pg-1"

get_email_code() {
    DB_USER="dbuser"
    DB_NAME="default"
    local EMAIL=$1
    local email_code_response=$(docker exec -i "$DB_CONTAINER" psql -U "$DB_USER" -d "$DB_NAME" -t -c "SELECT body FROM courier_messages WHERE recipient='$EMAIL' ORDER BY created_at DESC LIMIT 1;")
    local email_code=$(echo "$email_code_response" | grep -Eo '[0-9]{6}')
    echo $email_code
}

register_email() {
    local PHONE=$1
    local CODE="000000"
    local EMAIL=$2

    local login_response=$(curl -s -X POST "$AUTH_ENDPOINT" -H "Content-Type: application/json" -d '{"phone": "'"$PHONE"'", "code":"'"$CODE"'"}')
    echo "Login response: $login_response"
    local auth_token=$(echo "$login_response" | jq -r '.authToken')

    local mail_reg_init_response=$(curl -s -X POST "$GRAPHQL_ENDPOINT" -H "Content-Type: application/json" -H "Authorization: Bearer $auth_token" -d '
    {
        "query": "mutation UserEmailRegistrationInitiate { userEmailRegistrationInitiate(input: { email: \"'"$EMAIL"'\" }) { emailRegistrationId errors { code message path } me { createdAt id language phone totpEnabled username } } }"
    }')
    local email_registration_id=$(echo "$mail_reg_init_response" | jq -r '.data.userEmailRegistrationInitiate.emailRegistrationId')
    echo "Email registration ID: $email_registration_id"

    local email_code=$(get_email_code "$EMAIL")
    local email_reg_validate_response=$(curl -s -X POST "$GRAPHQL_ENDPOINT" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $auth_token" \
    -d '{"query": "mutation UserEmailRegistrationValidate { userEmailRegistrationValidate( input: { code: \"'"$email_code"'\", emailRegistrationId: \"'"$email_registration_id"'\" }) { errors { code message path } } }"}')
    echo "Email registration validate response: $email_reg_validate_response"
}
