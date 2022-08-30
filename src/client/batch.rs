use serde::Deserialize;

use std::fs::File;

use super::*;

#[derive(Debug, Deserialize)]
struct PaymentInput {
    username: String,
    usd: u64,
}

#[derive(Debug)]
struct Payment {
    username: String,
    usd: u64,
    sats: Option<u64>,
    wallet_id: Option<String>,
}

pub struct Batch {
    payments: Vec<Payment>,
    client: GaloyClient,
    /// price in btc/usd
    price: f64,
}

impl Batch {
    pub fn new(client: GaloyClient, price: f64) -> Self {
        let payments: Vec<Payment> = vec![];
        Self {
            payments,
            client,
            price,
        }
    }

    pub fn add(&mut self, username: String, usd: u64) {
        self.payments.push(Payment {
            username,
            usd,
            wallet_id: None,
            sats: None,
        });
    }

    pub fn add_csv(&mut self, filename: String) -> anyhow::Result<()> {
        let file = File::open(filename)?;
        let mut rdr = csv::Reader::from_reader(file);
        for result in rdr.deserialize() {
            let record: PaymentInput = result?;
            self.add(record.username, record.usd);
        }

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.payments.len()
    }

    pub fn is_empty(&self) -> bool {
        self.payments.len() == 0
    }

    pub fn populate_wallet_id(&mut self) -> anyhow::Result<()> {
        for payment in &mut self.payments {
            let username = payment.username.to_string();
            let query = &self.client.default_wallet(username.clone());
            match query {
                Ok(value) => payment.wallet_id = Some(value.clone()),
                Err(error) => bail!("error query {:?}", error),
            }
        }

        Ok(())
    }

    pub fn populate_sats(&mut self) -> anyhow::Result<()> {
        for payment in &mut self.payments {
            let payment_btc: f64 = (payment.usd as f64) / self.price;
            payment.sats = Some((payment_btc * 100_000_000.) as u64);
        }

        Ok(())
    }

    pub fn check_self_payment(&self) -> anyhow::Result<()> {
        let me = self.client.me()?;
        let me_wallet_id = &me.default_account.default_wallet_id;

        for payment in &self.payments {
            match &payment.wallet_id {
                Some(wallet_id) => {
                    if me_wallet_id == wallet_id {
                        println!("{:#?}", (me_wallet_id, wallet_id));
                        bail!("can't pay to self wallet_id")
                    }
                }
                None => bail!("wallet_id is not present"),
            };
        }

        Ok(())
    }

    pub fn check_balance(&self) -> anyhow::Result<()> {
        let me = self.client.me()?;
        let me_wallet_id = me.default_account.default_wallet_id;

        let mut total_sats: u64 = 0;

        for payment in &self.payments {
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
            Some(value) => value.balance as u64,
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

    pub fn execute(&self) -> anyhow::Result<()> {
        self.check_self_payment()?;
        self.check_balance()?;

        for payment in &self.payments {
            let username = payment.username.clone();
            let amount = match &payment.sats {
                Some(value) => *value,
                None => bail!("need sats amount"),
            };
            let usd = &payment.usd;
            let res = &self
                .client
                .intraleger_send(username.clone(), amount)
                .context("issue sending intraledger")?;

            println!(
                "payment to {username} of sats {amount}, usd {usd}: {:?}",
                res
            );
        }

        Ok(())
    }
}
