use thiserror::Error;

#[derive(Error, Debug)]
pub enum CaptchaError {
    #[error("Empty captcha create challenge")]
    EmptyCaptcha,
}
