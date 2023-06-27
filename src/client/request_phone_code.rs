use super::{
    queries::{
        user_request_auth_code, PhoneCodeChannelType, UserRequestAuthCode,
        UserRequestAuthCodeInput, UserRequestAuthCodeUserRequestAuthCode,
    },
    server, GaloyClient,
};
use graphql_client::reqwest::post_graphql;
use std::net::TcpListener;

impl GaloyClient {
    pub async fn request_phone_code(
        &self,
        phone: String,
        nocaptcha: bool,
        api: String,
    ) -> std::io::Result<()> {
        match nocaptcha {
            false => {
                let listener = TcpListener::bind("127.0.0.1:0")?;
                let port = listener.local_addr().unwrap().port();
                println!(
                    "Visit http://127.0.0.1:{}/login and solve the Captcha",
                    port
                );

                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?;
                rt.block_on(server::run(listener, phone, api)?)
            }

            true => {
                let input = UserRequestAuthCodeInput {
                    phone,
                    channel: Some(PhoneCodeChannelType::SMS),
                };

                let variables = user_request_auth_code::Variables { input };
                let response_body =
                    post_graphql::<UserRequestAuthCode, _>(&self.graphql_client, &api, variables)
                        .await
                        .expect("issue fetching response");
                let response_data = response_body.data.expect("Query failed or is empty");
                let UserRequestAuthCodeUserRequestAuthCode { success, errors } =
                    response_data.user_request_auth_code;

                match success {
                    Some(true) => {}
                    _ if !errors.is_empty() => {
                        println!("{:?}", errors);
                        log::error!("request failed (graphql errors)");
                    }
                    Some(false) => {
                        log::error!("request failed (success is false)");
                    }
                    _ => {
                        log::error!("request failed (unknown)");
                    }
                }

                Ok(())
            }
        }
    }
}
