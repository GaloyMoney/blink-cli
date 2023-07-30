# [galoy-cli release v0.1.2](https://github.com/GaloyMoney/galoy-cli/releases/tag/v0.1.2)


### Features

- Onchain receive (#182)

### Testing

- Bootstrap galoy for testing locally (#181)

# [galoy-cli release v0.1.1](https://github.com/GaloyMoney/galoy-cli/releases/tag/v0.1.1)


### Miscellaneous Tasks

- Added MIT license and basic readme

# [galoy-cli release v0.1.0](https://github.com/GaloyMoney/galoy-cli/releases/tag/v0.1.0)


### Bug Fixes

- Check code
- Add Comma in commands.rs (#170)
- Outdated code (#123)
- Openssl warning from cargo audit (#106)
- TOOD -> TODO typo

### Features

- Implement Batch Payments (#169)
- Add request-phone-code (#168)
- User, wallet and pay commands with efficient error handling (#166)
- Implement Login and Logout Commands (#163)
- Set Username (#142)
- Added cli command to send usd or sats explicitly (#131)
- [**breaking**] Request phone code via captcha (#52)
- Batch payment (still wip)
- Add intraledger send
- Adding query for default wallet

### Miscellaneous Tasks

- Cargo release description, release and licence and added blank changelog
- Bump serde from 1.0.160 to 1.0.163 (#133)
- Bump log from 0.4.17 to 0.4.18 (#134)
- Bump tera from 1.18.1 to 1.19.0 (#132)
- Bump reqwest from 0.11.16 to 0.11.18 (#135)
- Bump clap from 4.2.2 to 4.3.0 (#125)
- Bump tokio from 1.27.0 to 1.28.2 (#128)
- Bump graphql_client from 0.11.0 to 0.13.0 (#129)
- Bump csv from 1.2.1 to 1.2.2 (#130)
- Bump tokio from 1.22.0 to 1.25.0 (#84)
- Bump serde_json from 1.0.85 to 1.0.89 (#57)
- Bump rust_decimal from 1.26.1 to 1.27.0 (#58)
- Bump jsonwebtoken from 8.1.1 to 8.2.0 (#59)
- Bump url from 2.3.0 to 2.3.1 (#56)
- Bump clap from 4.0.26 to 4.0.29 (#55)
- Bump reqwest from 0.11.11 to 0.11.13 (#48)
- Bump clap from 3.2.19 to 4.0.26 (#49)
- Bump serde from 1.0.144 to 1.0.148 (#53)
- Bump anyhow from 1.0.63 to 1.0.66 (#43)
- Bump url from 2.2.2 to 2.3.0 (#24)
- Remove unnecessary specifier
- Using drain to avoid using clone
- Also test from csv
- Adding an optional memo field
- Test batch execution
- Use Into interface
- Bump anyhow from 1.0.62 to 1.0.63
- JWT should not be shown on the console
- Use interface from clap
- Misc style improvment
- Use Decimal instead of u64/i64
- Bump clap from 3.2.18 to 3.2.19
- Refactor check self payment
- Bump clap from 3.2.17 to 3.2.18
- Better use of match
- Some syntaxes improvment
- Wip
- Adding me query. needed for send payment
- Reorg file to map galoy-client
- Add missing (super)
- Restructure client module
- Add fail-on-warnings feature and fix all clippy warnings
- Syntax improvment
- Using proper is_ok() function
- To_string is not needed
- Using context instead of expect
- Hiding the implementation of the graphql client behind a struct
- Use anyhow::Context. also use String directly
- Use at top of file
- Reorg file to remove workspace

### Refactor

- [**breaking**] Start fresh with segregated layers (#162)

### Testing

- Add e2e Tests (#171)
- E2e authenticate (#139)
- Refactor code with common fn
- Bracket not needed
- Add first integration test
