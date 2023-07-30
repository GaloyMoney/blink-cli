use anyhow::Result;
use csv;
use indicatif::{ProgressBar, ProgressStyle};
use prettytable::{format, row, Table};
use rust_decimal::Decimal;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

use crate::app::errors::payment_error::PaymentError;
use crate::app::App;
use crate::client::types::Wallet;

const CSV_HEADER_USERNAME: &str = "username";
const CSV_HEADER_CENTS: &str = "cents";
const CSV_HEADER_SATS: &str = "sats";
const CSV_HEADER_MEMO: &str = "memo";

pub struct ListedPayment {
    pub username: String,
    pub recipient_wallet_id: String,
    pub amount: Decimal,
    pub memo: Option<String>,
}

impl Wallet {
    pub fn to_unit(&self) -> &str {
        if *self == Wallet::Btc {
            "sats"
        } else {
            "cents"
        }
    }
}

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

    // TODO: add a check for the sending limits.

    Ok(())
}

pub fn verify_armed_records(
    armed_records: &Vec<ListedPayment>,
    wallet_type: &Wallet,
    skip_confirmation: bool,
) -> Result<()> {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row![
        "Username",
        format!("Amount (in {})", wallet_type.to_unit()),
        "Memo"
    ]);

    for record in armed_records {
        table.add_row(row![
            bl->record.username,
            br->record.amount,
            format!(
                "{}",
                if record.memo.is_some() {
                    record.memo.clone().unwrap()
                } else {
                    "".to_string()
                }
            )
        ]);
    }

    println!("These are the specified payouts:\n");
    table.printstd();
    println!("\nYou're sending funds from your {:?} wallet.", wallet_type);

    if !skip_confirmation {
        print!("Are you sure you want to submit this payout (type 'yes' to confirm): ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string();

        if input != "yes" {
            println!("Payout cancelled!");
            std::process::exit(0);
        }
    }

    println!();

    Ok(())
}

impl App {
    pub async fn batch_payment(&self, file: String, skip_confirmation: bool) -> Result<()> {
        // ----- Checks -----
        println!("Checking usernames and balances...");
        check_file_exists(&file)?;
        let (reader, wallet_type) = validate_csv(&file)?;

        for record in &reader {
            let username = record
                .get(0)
                .filter(|&username| !username.is_empty())
                .ok_or(PaymentError::NoUsernameFound(record.clone()))?;

            // check if username exists
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

        // ----- Arming the records in internal structure -----

        let mut armed_records: Vec<ListedPayment> = vec![];

        for record in &reader {
            let username = record
                .get(0)
                .ok_or(PaymentError::NoUsernameFound(record.clone()))?
                .to_string();
            let recipient_wallet_id = self.client.default_wallet(username.to_string()).await?;
            let amount: Decimal = Decimal::from_str(record.get(1).unwrap_or_default())?;
            let memo = record.get(2).map(|s| s.to_string());

            armed_records.push(ListedPayment {
                username: username.clone(),
                recipient_wallet_id,
                amount,
                memo: memo.clone(),
            });
        }

        verify_armed_records(&armed_records, &wallet_type, skip_confirmation)?;

        // ----- Run -----

        let total_size: u64 = reader.iter().len().try_into()?;
        let pb = ProgressBar::new(total_size);

        pb.enable_steady_tick(std::time::Duration::from_millis(10));
        pb.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>5}/{len:5} {msg}",
            )
            .unwrap()
            .progress_chars("=> "),
        );

        for record in armed_records.into_iter() {
            let ListedPayment {
                recipient_wallet_id,
                amount,
                memo,
                username,
            } = record;

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

            pb.inc(1);
            pb.println(format!(
                "Payment of {} {} sent successfully to {}!",
                amount,
                wallet_type.to_unit(),
                username
            ));
        }

        pb.finish_with_message("Batch payouts completed successfully!");

        Ok(())
    }
}
