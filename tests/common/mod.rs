use super::*;

pub fn unauth_client() -> galoy_cli::GaloyClient {
    let api = "https://api.staging.galoy.io/graphql".to_string();
    GaloyClient::new(api, None)
}
