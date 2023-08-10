## galoy-cli

[![GitHub license](https://img.shields.io/github/license/GaloyMoney/galoy-cli)](https://github.com/GaloyMoney/galoy-cli)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](#contributing)



**galoy-cli** is a Command Line Interface (CLI) client for interacting with Galoy Backend. It provides a variety of subcommands to perform different operations, making it convenient for developers and users to interact with the Galoy Backend.

## Installation

### Prerequisites
Before you begin, ensure you have the following installed:

- **Rust** (required for building from source)
- **Cargo** (Rust's package manager)

### Cargo
If you have Rust and Cargo installed, you can easily install galoy-cli using the following command:

```bash
cargo install galoy-cli
```
This will fetch the latest version of galoy-cli from the [crates.io]([crates.io](https://crates.io/)) repository and install it globally on your system.

<!-- #### Chocolatey -->



### Releases

You can download the pre-built binaries of galoy-cli from the [releases page](https://github.com/GaloyMoney/galoy-cli/releases/). Follow these steps:

1.  Visit the [releases page](https://github.com/GaloyMoney/galoy-cli/releases/) of the galoy-cli repository.
2. Locate the latest release and navigate to the "Assets" section.
3. Depending on your operating system, download the appropriate binary (e.g., *galoy-cli-x86_64-unknown-linux-mus* for Linux, *galoy-cli-x86_64-apple-darwin* for macOS, *galoy-cli-x86_64-pc-windows* for Windows).
4. Once downloaded, make the binary executable (if required) using the following command:
```bash
chmod +x galoy-cli
```
5. Optionally, move the binary to a directory listed in your system's `PATH` to make it accessible from anywhere.

### Verification
To verify the installation, open a new terminal window and run:
```sh
galoy-cli --version
```
You should see the version number of galoy-cli displayed.




## Usage

Galoy CLI provides a range of commands to interact with the Galoy Backend. Each command serves a specific purpose and enables you to perform various operations seamlessly. Here is a simple guide on how to use the galoy-cli:

```bash
galoy-cli [OPTIONS] <COMMAND>
```

Below are some of the main commands along with brief descriptions of their functionality:

#### request-phone-code
Request a verification code from a phone number to initiate user authentication
```bash
galoy-cli request-phone-code <phone-number>
```
#### login
Get an authentication token for a user account. This token is required for performing authorized actions.
```bash
galoy-cli login <phone-number> <phone-code>
```

#### pay
Execute a payment using the specified payment method and details. You can choose to make payments in Bitcoin (BTC) or USD.
```bash
galoy-cli pay [OPTIONS] --wallet <WALLET>
```
Options:
```sh
-u, --username <USERNAME>: The username associated with the recipient's account.
-o, --onchain-address <ONCHAIN_ADDRESS>: The recipient's on-chain Bitcoin address.
-l, --ln-payment-request <LN_PAYMENT_REQUEST>: The Lightning Network payment request for the recipient.
-w, --wallet <WALLET>: Specify the currency wallet to use for the payment. Possible values: btc, usd.
-c, --cents <CENTS>: The payment amount in cents (for USD payments).
-s, --sats <SATS>: The payment amount in satoshis (for BTC payments).
-m, --memo <MEMO>: An optional memo to attach to the payment.
```

To view detailed information about each command and its available options, use the help subcommand followed by the specific command name:
```bash
galoy-cli <command-name> --help
```
Remember that you can always refer to the galoy-cli --help command to view a summary of all available commands, options, and the default API endpoint.

```bash
galoy-cli --help
```

## Configuration

By default, `galoy-cli` is configured to interact with the Galoy Backend's mainnet production environment at [api.mainnet.galoy.io](https://api.mainnet.galoy.io/graphql). However, developers have the flexibility to switch between different environments based on their needs.

### Changing the API Endpoint
To change the API endpoint that galoy-cli interacts with, you can set the `GALOY_API` environment variable. This is particularly useful for testing against different environments or using a local development instance of the Galoy Backend.

For example, to switch to the staging environment, you can set the `GALOY_API` environment variable to [https://api.staging.galoy.io/graphql](https://api.staging.galoy.io/graphql):

```bash
export GALOY_API=https://api.staging.galoy.io/graphql
galoy-cli <command>
```

If you're working on a local development instance of the Galoy Backend, you can set the API endpoint to your local instance's URL. For instance, if your local Galoy Backend is running at port 4002, you can set the `GALOY_API` to http://localhost:4002/graphql

By setting the API endpoint to your local instance, you can test and develop with your own data in a controlled environment.



## Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are greatly appreciated. We welcome contributions from the community to improve galoy-cli and make it even more powerful and user-friendly. Whether you want to fix a bug, implement a new feature, or enhance the documentation, your contributions are valuable to us.

### Guidelines

- Follow the existing code style and conventions used in the project.
- Keep your pull request focused. If you're addressing multiple issues or features, consider creating separate PRs for clarity.
- Ensure your changes are well-documented and add or update any necessary documentation.
- If you're introducing new features or functionality, consider adding corresponding tests to maintain code quality.


## License

Distributed under the MIT License. See `LICENSE` for more information.

## Contact
[![Mattermost](https://img.shields.io/badge/chat-on%20mattermost-blue?style=social&logo=mattermost)](https://chat.galoy.io)
[![Twitter Follow](https://img.shields.io/twitter/follow/GaloyMoney?style=social)](https://twitter.com/GaloyMoney)
