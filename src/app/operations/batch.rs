use anyhow::Result;
use csv;
use rust_decimal::Decimal;
use std::path::Path;
use std::str::FromStr;

use crate::app::errors::payment_error::PaymentError;
use crate::app::App;
use crate::client::types::Wallet;

const CSV_HEADER_USERNAME: &str = "username";
const CSV_HEADER_CENTS: &str = "cents";
const CSV_HEADER_SATS: &str = "sats";
const CSV_HEADER_MEMO: &str = "memo";

pub fn check_file_exists(file: &str) -> Result<(), PaymentError> {
    let file_path = Path::new(file);
    if !file_path.exists() {
        return Err(PaymentError::FileNotFound(file.to_string()));
    }
    Ok(())
}

pub fn validate_csv(file: &str) -> Result<(Vec<csv::StringRecord>, Wallet), PaymentError> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_path(file)
        .map_err(|_| PaymentError::FailedToReadCSV)?;
    let headers = reader
        .headers()
        .map_err(|_| PaymentError::FailedToGetHeaders)?;

    if &headers[0] != CSV_HEADER_USERNAME
        || (&headers[1] != CSV_HEADER_CENTS && &headers[1] != CSV_HEADER_SATS)
        || (headers.len() == 3 && &headers[2] != CSV_HEADER_MEMO)
    {
        return Err(PaymentError::IncorrectCSVFormat);
    }

    let wallet_type = if headers.get(1) == Some(CSV_HEADER_CENTS) {
        Wallet::Usd
    } else {
        Wallet::Btc
    };

    let records: Vec<csv::StringRecord> = reader
        .records()
        .collect::<Result<_, _>>()
        .map_err(|_| PaymentError::FailedToGetRecords)?;

    Ok((records, wallet_type))
}

pub fn check_sufficient_balance(
    records: &[csv::StringRecord],
    current_balance: Decimal,
) -> Result<()> {
    let mut total_payment_amount: Decimal = Decimal::new(0, 0);
    for record in records {
        let amount: Decimal = Decimal::from_str(record.get(1).unwrap_or_default())?;
        total_payment_amount += amount;
    }

    if total_payment_amount > current_balance {
        return Err(PaymentError::InsufficientBalance.into());
    }

    //TODO: add a check for the sending limits.

    Ok(())
}

impl App {
    pub async fn batch_payment(&self, file: String) -> Result<()> {
        check_file_exists(&file)?;
        let (reader, wallet_type) = validate_csv(&file)?;
        for record in &reader {
            let username = record
                .get(0)
                .filter(|&username| !username.is_empty())
                .ok_or(PaymentError::NoUsernameFound(record.clone()))?;

            //check if username exists
            self.client
                .default_wallet(username.to_string())
                .await
                .map_err(|_| PaymentError::UsernameDoesNotExist(username.to_string()))?;
        }

        let me = self.client.me().await?;

        let (sender_wallet_id, user_wallet_balance) = me
            .default_account
            .wallets
            .iter()
            .filter(|wallet| wallet_type == Wallet::from(&wallet.wallet_currency))
            .map(|wallet| (wallet.id.clone(), wallet.balance.clone()))
            .next()
            .unwrap_or_default();

        check_sufficient_balance(&reader, user_wallet_balance)?;

        for record in &reader {
            let username = record
                .get(0)
                .ok_or(PaymentError::NoUsernameFound(record.clone()))?;
            let recipient_wallet_id = self.client.default_wallet(username.to_string()).await?;

            let amount: Decimal = Decimal::from_str(record.get(1).unwrap_or_default())?;
            let memo = record.get(2).map(|s| s.to_string());

            match wallet_type {
                Wallet::Usd => {
                    self.client
                        .intraleger_send_usd(
                            sender_wallet_id.clone(),
                            recipient_wallet_id,
                            amount,
                            memo,
                        )
                        .await?;
                }
                Wallet::Btc => {
                    self.client
                        .intraleger_send_btc(
                            sender_wallet_id.clone(),
                            recipient_wallet_id,
                            amount,
                            memo,
                        )
                        .await?;
                }
            }
        }
        println!("Batch Payment successful!");
        Ok(())
    }
}
