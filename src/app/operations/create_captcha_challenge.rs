async fn create_captcha_challenge(&self) -> Result<CaptchaChallenge, CliError> {
    let variables = captcha_create_challenge::Variables;
    let response =
        post_graphql::<CaptchaCreateChallenge, _>(&self.graphql_client, &self.api, variables)?;
    if response.errors.is_some() {
        if let Some(error) = response.errors {
            return Err(CliError::CaptchaTopLevelError(error));
        }
    }
    let response = response
        .data
        .ok_or_else(|| CliError::CaptchaInnerError("Empty captcha response data".to_string()))?;
    let captcha_challenge_result = CaptchaChallenge::try_from(response)?;

    Ok(captcha_challenge_result)
}
