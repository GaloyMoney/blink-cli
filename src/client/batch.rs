use serde::Deserialize;

use std::fs::File;

use super::*;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

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
    pub fn new(client: GaloyClient, price: Decimal) -> Self {
        let payments: Vec<Payment> = vec![];
        Self {
            payments,
            client,
            price,
        }
    }

    pub fn add(&mut self, input: PaymentInput) {
        self.payments.push(input.into());
    }

    pub fn add_csv(&mut self, filename: String) -> anyhow::Result<()> {
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

    pub fn populate_wallet_id(&mut self) -> anyhow::Result<()> {
        for payment in self.payments.iter_mut() {
            let username = payment.username.clone();
            let query = &self.client.default_wallet(username);
            match query {
                Ok(value) => payment.wallet_id = Some(value.clone()),
                Err(error) => bail!("error query {:?}", error),
            }
        }

        Ok(())
    }

    pub fn populate_sats(&mut self) -> anyhow::Result<()> {
        for payment in self.payments.iter_mut() {
            let payment_btc: Decimal = payment.usd / self.price;
            payment.sats = Some(payment_btc * dec!(100_000_000));
        }

        Ok(())
    }

    pub fn check_self_payment(&self) -> anyhow::Result<()> {
        let me = self.client.me()?;

        #[allow(deprecated)]
        let me_username = match me.username {
            Some(value) => value,
            None => bail!("no username has been set"),
        };

        for payment in self.payments.iter() {
            if me_username == payment.username {
                println!("{:#?}", (me_username, &payment.username));
                bail!("can't pay to self")
            }
        }

        Ok(())
    }

    pub fn check_limit(&self) -> anyhow::Result<()> {
        todo!("Check limit. need API on the backend for it");
    }

    pub fn check_balance(&self) -> anyhow::Result<()> {
        let me = self.client.me()?;
        let me_wallet_id = me.default_account.default_wallet_id;

        let mut total_sats = dec!(0);

        for payment in self.payments.iter() {
            let sats = match payment.sats {
                Some(value) => value,
                None => bail!("sats needs to be populated first"),
            };
            total_sats += sats;
        }

        let me_default_wallet = me
            .default_account
            .wallets
            .iter()
            .find(|wallet| wallet.id == me_wallet_id);

        let balance_sats = match me_default_wallet {
            Some(value) => value.balance,
            None => bail!("no balance"),
        };
        if total_sats > balance_sats {
            bail!(
                "not enough balance, got {}, need {}",
                balance_sats,
                total_sats
            )
        }

        Ok(())
    }

    pub fn show(&self) {
        println!("{:#?}", &self.payments)
    }

    pub fn execute(&mut self) -> anyhow::Result<()> {
        self.check_self_payment()?;
        self.check_balance()?;

        for Payment {
            username,
            memo,
            usd,
            sats,
            ..
        } in self.payments.drain(..)
        {
            let amount = match sats {
                Some(value) => value,
                None => bail!("need sats amount"),
            };
            let res = &self
                .client
                .intraleger_send(username.clone(), amount, memo)
                .context("issue sending intraledger")?;

            println!(
                "payment to {username} of sats {amount}, usd {usd}: {:?}",
                res
            );
        }

        Ok(())
    }
}
