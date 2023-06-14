use csv;
use rust_decimal::Decimal;
use std::path::Path;
use std::str::FromStr;

use crate::client::GaloyClient;
use crate::client::Wallet;

// Utility function to check if file exists
pub fn check_file_exists(file: &str) -> anyhow::Result<()> {
    let file_path = Path::new(&file);
    if !file_path.exists() {
        return Err(anyhow::anyhow!("File not found: {}", file));
    }
    Ok(())
}

// Utility function to read and validate the CSV file
pub fn validate_csv(
    galoy_cli: &GaloyClient,
    file: &str,
) -> anyhow::Result<(Vec<csv::StringRecord>, Wallet)> {
    let mut reader = csv::ReaderBuilder::new().delimiter(b',').from_path(file)?;
    let headers = reader.headers()?.clone();

    if &headers[0] != "username"
        || (&headers[1] != "cents" && &headers[1] != "sats")
        || (headers.len() == 3 && &headers[2] != "memo")
    {
        return Err(anyhow::anyhow!(
            "CSV format not correct, requires: username, (cents or sats), memo(optional)"
        ));
    }

    let wallet_type = if headers.get(1) == Some("cents") {
        Wallet::Usd
    } else {
        Wallet::Btc
    };

    let records: Vec<csv::StringRecord> = reader.records().collect::<Result<_, _>>()?;

    // Validate each record
    for record in &records {
        let username = record
            .get(0)
            .ok_or(anyhow::anyhow!("Username is missing"))?;

        let amount = record.get(1).ok_or(anyhow::anyhow!("Amount is missing"))?;
        amount
            .parse::<Decimal>()
            .map_err(|_| anyhow::anyhow!("Amount must be a number"))?;

        // Check if the username exists
        galoy_cli.default_wallet(username.to_string())?;
    }

    Ok((records, wallet_type))
}

pub fn check_sufficient_balance(
    records: &[csv::StringRecord],
    wallet_type: Wallet,
    galoy_cli: &GaloyClient,
) -> anyhow::Result<()> {
    let balance_info = galoy_cli.fetch_balance(Some(wallet_type), Vec::new())?;
    let current_balance: Decimal = balance_info.iter().map(|info| info.balance).sum();

    let mut total_payment_amount: Decimal = Decimal::new(0, 0);
    for record in records {
        let amount: Decimal = Decimal::from_str(record.get(1).unwrap_or_default())?;
        total_payment_amount += amount;
    }

    if total_payment_amount > current_balance {
        return Err(anyhow::anyhow!("Insufficient balance in the wallet"));
    }

    Ok(())
}

pub fn execute_batch_payment(
    records: &[csv::StringRecord],
    wallet_type: Wallet,
    galoy_cli: &GaloyClient,
) -> anyhow::Result<()> {
    for record in records {
        let username = record
            .get(0)
            .ok_or(anyhow::anyhow!("Username is missing"))?;

        let amount: Decimal = Decimal::from_str(record.get(1).unwrap_or_default())?;

        let memo = record.get(2).map(|s| s.to_string());

        match wallet_type {
            Wallet::Usd => {
                galoy_cli.intraleger_usd_send(username.to_string(), amount, memo)?;
            }
            Wallet::Btc => {
                galoy_cli.intraleger_send(username.to_string(), amount, memo)?;
            }
        }
    }
    Ok(())
}
