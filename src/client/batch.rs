use std::fs::File;

use comfy_table::{Row, Table};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::Deserialize;

use super::*;
use crate::{BatchError, GaloyCliError};

#[derive(Debug, Deserialize)]
pub struct PaymentInput {
    pub username: String,
    pub usd: Decimal,
    pub memo: Option<String>,
}

impl From<PaymentInput> for Payment {
    fn from(input: PaymentInput) -> Payment {
        Payment {
            username: input.username,
            usd: input.usd,
            sats: None,
            wallet_id: None,
            memo: input.memo,
        }
    }
}

#[derive(Debug)]
struct Payment {
    username: String,
    usd: Decimal,
    sats: Option<Decimal>,
    wallet_id: Option<String>,
    memo: Option<String>,
}

pub struct Batch {
    payments: Vec<Payment>,
    client: GaloyClient,
    /// price in btc/usd
    price: Decimal,
}

impl Batch {
    pub fn new(client: GaloyClient, price: Decimal) -> Result<Self, GaloyCliError> {
        if price == Decimal::ZERO {
            return Err(GaloyCliError::Batching {
                message: "Price cannot be zero. Division by 0 downstream".to_string(),
                kind: crate::BatchError::DivisionByZero,
            });
        }

        let payments: Vec<Payment> = vec![];
        Ok(Self {
            payments,
            client,
            price,
        })
    }

    pub fn add(&mut self, input: PaymentInput) {
        self.payments.push(input.into());
    }

    pub fn add_csv(&mut self, filename: String) -> Result<(), GaloyCliError> {
        let file = File::open(filename)?;
        let mut rdr = csv::Reader::from_reader(file);
        for result in rdr.deserialize() {
            let record: PaymentInput = result?;
            self.add(record);
        }

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.payments.len()
    }

    pub fn is_empty(&self) -> bool {
        self.payments.is_empty()
    }

    pub fn populate_wallet_id(&mut self) -> Result<(), GaloyCliError> {
        for payment in self.payments.iter_mut() {
            let username = payment.username.clone();
            let query = &self.client.default_wallet(username)?;

            payment.wallet_id = Some(query.clone());
        }

        Ok(())
    }

    pub fn populate_sats(&mut self) -> Result<(), GaloyCliError> {
        for payment in self.payments.iter_mut() {
            let payment_btc: Decimal = payment.usd / self.price;
            payment.sats = Some(payment_btc * dec!(100_000_000));
        }

        Ok(())
    }

    pub fn check_self_payment(&self) -> Result<(), GaloyCliError> {
        let me = self.client.me()?;

        // TODO: username is deprecated, switch to handle when ready
        #[allow(warnings)]
        let me_username = me.username.ok_or_else(|| {
            GaloyCliError::GraphQl(message_only_error(
                "Empty `username`. Value not set".to_string(),
            ))
        })?;

        for payment in self.payments.iter() {
            if me_username == payment.username {
                println!("{:#?}", (me_username, &payment.username));
                return Err(GaloyCliError::Batching {
                    kind: BatchError::SelfPayment,
                    message: "Cannot pay to self".to_string(),
                });
            }
        }

        Ok(())
    }

    pub fn check_limit(&self) -> Result<(), GaloyCliError> {
        todo!("Check limit. need API on the backend for it");
    }

    pub fn check_balance(&self) -> Result<(), GaloyCliError> {
        let me = self.client.me()?;
        let me_wallet_id = me.default_account.default_wallet_id;

        let mut total_sats = dec!(0);

        for payment in self.payments.iter() {
            let sats = payment.sats.ok_or_else(|| GaloyCliError::Batching {
                message: "Sats needs to be populated first".to_string(),
                kind: BatchError::Empty,
            })?;
            total_sats += sats;
        }

        let me_default_wallet = me
            .default_account
            .wallets
            .iter()
            .find(|wallet| wallet.id == me_wallet_id);

        let balance_sats = me_default_wallet.ok_or_else(|| GaloyCliError::Batching {
            message: "No balance".to_string(),
            kind: BatchError::NoBalance,
        })?;

        if total_sats > balance_sats.balance {
            return Err(GaloyCliError::Batching {
                message: format!(
                    "Not enough balance, got {}, need {}",
                    balance_sats.balance, total_sats
                ),
                kind: BatchError::InsufficientBalance,
            });
        }

        Ok(())
    }

    pub fn show(&self) {
        let mut table = Table::new();
        let header = Row::from(vec![
            "Username",
            "Amount (USD)",
            "Amount (Sats)",
            "Wallet_Id",
            "Memo",
        ]);
        table.set_header(header);

        for Payment {
            username,
            usd,
            sats,
            wallet_id,
            memo,
        } in self.payments.iter()
        {
            let row = Row::from(vec![
                username.clone(),
                usd.to_string(),
                format!("{:?}", sats),
                format!("{:?}", wallet_id),
                format!("{:?}", memo),
            ]);
            table.add_row(row);
        }

        println!("{table}")
    }

    pub fn execute(&mut self) -> Result<(), GaloyCliError> {
        self.check_self_payment()?;
        self.check_balance()?;

        let mut table = Table::new();
        let header = Row::from(vec!["Username", "Amount (Sats)", "Amount (USD)", "Result"]);
        table.set_header(header);

        for Payment {
            username,
            memo,
            usd,
            sats,
            ..
        } in self.payments.drain(..)
        {
            let amount = sats.ok_or_else(|| GaloyCliError::Batching {
                message: "Need sats amount".to_string(),
                kind: BatchError::Empty,
            })?;

            let res = &self
                .client
                .intraleger_send(username.clone(), amount, memo)?;

            let row = Row::from(vec![
                username,
                amount.to_string(),
                usd.to_string(),
                format!("{:?}", res),
            ]);
            table.add_row(row);
        }

        println!("{table}");
        Ok(())
    }
}
