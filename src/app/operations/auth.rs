use anyhow::{Context, Result};

use crate::app::{file_manager, App};

impl App {
    pub async fn user_login(&self, phone: Option<String>, code: String, email: bool ,two_fa_code: Option<String>)  -> Result<()> {
        if let Some(phone) = phone {
            let result = self
                .client
                .user_login_phone(phone.clone(), code.clone())
                .await
                .context("Failed to log in")?;
            
            let auth_token = result.auth_token;
            let totp_required = result.totp_required;

            if totp_required {
                let mut is_valid_2fa = false;
                let mut final_two_fa_code = two_fa_code;

                while !is_valid_2fa {
                    if final_two_fa_code.is_none() {
                        println!("Your account requires two-factor authentication. Please enter your TFA code:");
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).expect("Failed to read line");
                        final_two_fa_code = Some(input.trim().to_string());
                    }

                    is_valid_2fa = self
                        .client
                        .validate_totp_code(auth_token.clone(), final_two_fa_code.clone().unwrap())
                        .await
                        .context("something went wrong")?;

                    if !is_valid_2fa {
                        println!("The entered 2FA code is incorrect. Please enter the correct TFA code:");
                        final_two_fa_code = None; 
                    }
                }
            }
        
            file_manager::save_data(file_manager::TOKEN_FILE_NAME, &auth_token)
                .context("Failed to save token")?;

            println!("User logged in successfully!");
        } else if email {
            let email_login_id =
                file_manager::get_data(file_manager::EMAIL_LOGIN_ID_FILE_NAME)?.unwrap();

            let result = self
                .client
                .user_login_email(email_login_id, code.clone())
                .await
                .context("Failed to log in")?;

            let auth_token = result.auth_token;
            let totp_required = result.totp_required;

           if totp_required {
                let mut is_valid_2fa = false;
                let mut final_two_fa_code = two_fa_code;

                while !is_valid_2fa {
                    if final_two_fa_code.is_none() {
                        println!("Your account requires two-factor authentication. Please enter your TFA code:");
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).expect("Failed to read line");
                        final_two_fa_code = Some(input.trim().to_string());
                    }

                    is_valid_2fa = self
                        .client
                        .validate_totp_code(auth_token.clone(), final_two_fa_code.clone().unwrap())
                        .await
                        .context("something went wrong")?;

                    if !is_valid_2fa {
                        println!("The entered 2FA code is incorrect. Please enter the correct TFA code:");
                        final_two_fa_code = None; 
                    }
                }
            }
            
            file_manager::save_data(file_manager::TOKEN_FILE_NAME, &auth_token)
                .context("Failed to save token")?;

            println!("User logged in successfully!");
        }
        Ok(())
    }

    pub async fn user_logout(&self) -> Result<()> {
        file_manager::remove_data(file_manager::TOKEN_FILE_NAME).context("Failed to log out")?;
        println!("User logged out successfully!");
        Ok(())
    }
}
