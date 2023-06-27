use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    version,
    author = "Galoy",
    about = "Galoy CLI",
    long_about = "CLI client to interact with Galoy's APIs"
)]
pub struct Cli {
    #[clap(
        long,
        env = "GALOY_API",
        default_value = "http://localhost:4002/graphql"
    )]
    pub api: String,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Get global values from the instance
    Globals,
    /// Get auth token of an account
    Login { phone: String, code: String },
    /// Logout the current user by removing the auth token
    Logout,
    /// Request a code from a Phone number
    RequestPhoneCode {
        #[clap(value_parser)]
        phone: String,

        #[clap(long, action)]
        nocaptcha: bool,
    },
}
