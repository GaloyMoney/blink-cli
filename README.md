# Galoy CLI
Galoy CLI is a Rust-based CLI client that can interact with the Galoy backend using GraphQL queries and mutations.

## Installation
To install Galoy CLI locally and set up a local environment:

1. Clone the repository using git clone `https://github.com/GaloyMoney/galoy-cli.git`.
2. Install Rust in your local machine and run cargo build to build all binary and library targets of the selected packages.
3. Run cargo run command to run all tests of the Galoy CLI repository and see the usage, commands, and options available to interact.
4. Interact with the CLI yourself to become familiar with it. After building, try the following command:

`GALOY_API=https://api.staging.galoy.io/graphql ./target/debug/galoy-client getinfo`
This command will retrieve the global values from the Galoy instance.

You can also test if the "GALOY CLI CAPTCHA SERVER" is running or not by running the following command:

`GALOY_API=https://api.staging.galoy.io/graphql ./target/debug/galoy-client request-phone-code +16505554321`

## Usage
To use the Galoy CLI, you need to run it with the desired command and options. 

### Commands:
getinfo: &nbsp; &nbsp; &nbsp;         Get global values from the instance. <br/>
default-wallet:     Get WalletId for an account. <br/>
me:                 Execute Me query. <br/>
send-intraledger:   Do an intraledger transaction. <br/>
request-phone-code: Request a code from a Phone number. <br/>
login:              Get JWT of an account. <br/>
batch:              Execute a batch payment.

To see the available options for each command, run galoy-client <COMMAND> --help.


### Options
The available options for the Galoy CLI are:

-a, --api <API>:   Set the API URL (default: http://localhost:4002/graphql) <br/>
-d, --debug:       Enable debug mode<br/>
-j, --jwt <JWT>:   Set the JWT for authorization<br/>
-h, --help:        Display help information <br/>
-V, --version:     Display version information


## Contributing
If you would like to contribute to Galoy CLI, please open a pull request on GitHub.