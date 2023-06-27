mod globals;
mod queries;
mod request_phone_code;
mod server;
mod user_login;

use reqwest::{header, Client as GraphQLClient};

pub struct GaloyClient {
    graphql_client: GraphQLClient,
    pub api: String,
}

impl GaloyClient {
    pub fn new(api: String, token: Option<String>) -> anyhow::Result<Self> {
        let mut client_builder = GraphQLClient::builder();

        if let Some(token) = token {
            let mut headers = header::HeaderMap::new();

            let token = format!("Bearer {}", token);

            let mut auth_value = header::HeaderValue::from_str(&token)?;
            auth_value.set_sensitive(true);
            headers.insert(header::AUTHORIZATION, auth_value);

            client_builder = client_builder.default_headers(headers)
        };

        let graphql_client = client_builder.build().expect("can't initialize client");

        Ok(Self {
            graphql_client,
            api,
        })
    }
}
