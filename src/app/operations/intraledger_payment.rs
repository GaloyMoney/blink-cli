use anyhow::Result;

use rust_decimal::Decimal;

use crate::{app::App, client::types::Wallet, errors::payment_error::PaymentError};

impl App {
    pub async fn intraledger_payment(
        &self,
        username: String,
        wallet: Wallet,
        cents: Option<Decimal>,
        sats: Option<Decimal>,
        memo: Option<String>,
    ) -> Result<()> {
        if wallet == Wallet::Btc && sats.is_none() {
            return Err(PaymentError::AmountNotSpecifiedBTC.into());
        }

        if wallet == Wallet::Usd && cents.is_none() {
            return Err(PaymentError::AmountNotSpecifiedUSD.into());
        }

        match wallet {
            Wallet::Btc => {
                let sats = sats.expect("Can't unwrap sats");
                match self
                    .client
                    .intraleger_send_btc(username.clone(), sats, memo)
                    .await
                {
                    Ok(()) => {
                        println!("Successfully sent {} sats to username: {}", sats, username)
                    }
                    Err(err) => {
                        eprintln!("Error occurred while executing BTC intraledger payment ",);
                        return Err(err.into());
                    }
                }
            }
            Wallet::Usd => {
                let cents = cents.expect("Can't unwrap cents");
                match self
                    .client
                    .intraleger_send_usd(username.clone(), cents, memo)
                    .await
                {
                    Ok(()) => {
                        println!(
                            "Successfully sent {} cents to username: {}",
                            cents, username
                        )
                    }
                    Err(err) => {
                        eprintln!("Error occurred while sending USD intraledger payment ",);
                        return Err(err.into());
                    }
                }
            }
        }

        Ok(())
    }
}
