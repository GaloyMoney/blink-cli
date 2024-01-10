use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use prettytable::{format, row, Table};
use rust_decimal::{Decimal, RoundingStrategy};
use std::{io::Write, path::Path, str::FromStr};

use crate::{
    app::{errors::payment_error::PaymentError, App},
    client::{
        queries::{
            query_me::{QueryMeMeDefaultAccountWallets, WalletCurrency},
            RealtimePriceRealtimePriceBtcSatPrice,
        },
        types::{AmountCurrency, Wallet},
    },
};

const USERNAME_IDX: usize = 0;
const AMOUNT_IDX: usize = 1;
const CURRENCY_IDX: usize = 2;
const WALLET_IDX: usize = 3;
const MEMO_IDX: usize = 4;

const CSV_HEADERS: [&str; 5] = [
    "username", "amount", "currency", "wallet", "memo"
];


pub struct ListedPayment {
    pub username: String,
    pub amount: Decimal,
    pub currency: AmountCurrency,
    pub wallet_currency: WalletCurrency,
    pub memo: Option<String>,
    pub recipient_wallet_id: String,
}

pub struct TotalAmount {
    btc_wallet: BtcWalletAmount,
    usd_wallet: UsdWalletAmount,
}

pub struct BtcWalletAmount {
    sats: Decimal,
    usd: Decimal,
}

pub struct UsdWalletAmount {
    usd: Decimal,
}

impl WalletCurrency {
    pub fn to_str(&self) -> &str {
        if *self == WalletCurrency::BTC {
            "BTC"
        } else {
            "USD"
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


pub fn validate_csv(
    file: &str,
    default_wallet: &WalletCurrency,
) -> Result<Vec<csv::StringRecord>, PaymentError> {

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_path(file)
        .map_err(|_| PaymentError::FailedToReadCSV)?;

    let headers = reader.headers()
        .map_err(|_| PaymentError::FailedToGetHeaders)?;

    if headers.iter().zip(CSV_HEADERS.iter()).any(|(a, &b)| a != b) {
        return Err(PaymentError::IncorrectCSVFormat);
    }

    reader.records().map(|result| {
        let record = result.map_err(|_| PaymentError::FailedToGetRecords)?;
        let username_str = record.get(USERNAME_IDX)
            .ok_or(PaymentError::IncorrectCSVFormat)?
            .trim();
        let amount_str = record.get(AMOUNT_IDX)
            .ok_or(PaymentError::IncorrectCSVFormat)?
            .trim();
        let currency_str = record.get(CURRENCY_IDX)
            .ok_or(PaymentError::IncorrectCSVFormat)?
            .trim();

        // required fields
        if username_str.is_empty() || amount_str.is_empty() || currency_str.is_empty() {
            return Err(PaymentError::IncorrectCSVFormat);
        }

        // if wallet is not given use default wallet, optional fields ie wallet and memo
        let wallet_str = record.get(WALLET_IDX).map_or(default_wallet.to_str(), |wallet| if wallet.is_empty() { default_wallet.to_str() } else { wallet });
        let memo_str = record.get(MEMO_IDX).unwrap_or("");


        let amount = Decimal::from_str(amount_str).map_err(|_| PaymentError::IncorrectCSVFormat)?;
        //amount should be greater than 0        
        if amount <= Decimal::new(0, 0) {
            return Err(PaymentError::IncorrectCSVFormat);
        }

        let currency = match currency_str {
            "SATS" => AmountCurrency::SATS,
            "USD" => AmountCurrency::USD,
            _ => return Err(PaymentError::IncorrectCSVFormat),
        };

        //amount for SATS will be whole number and for USD max 2 decimals
        let formatted_amount = match currency {
            AmountCurrency::SATS => amount.round_dp(0).to_string(),
            AmountCurrency::USD => amount
                .round_dp_with_strategy(2, RoundingStrategy::MidpointAwayFromZero)
                .to_string(),
        };

        //if currency is SATS then wallet should be BTC
        if  currency_str == "SATS" && wallet_str != "BTC" {
            return Err(PaymentError::IncorrectCSVFormat);
        }

        //wallet can only be USD and BTC
        if wallet_str != "USD" && wallet_str != "BTC" {
            return Err(PaymentError::IncorrectCSVFormat);
        }

        Ok(csv::StringRecord::from(vec![
            username_str.to_string(),
            formatted_amount,
            currency_str.to_string(),
            wallet_str.to_string(),
            memo_str.to_string(),
        ]))
    }).collect()
}


pub fn check_sufficient_balance(
    amount_payable: &TotalAmount,
    btc_sat_price: &RealtimePriceRealtimePriceBtcSatPrice,
    btc_wallet_balance: Decimal,
    usd_wallet_balance: Decimal,
) -> Result<()> {
    let total_payable_amount_for_btc_wallet =
        convert_usd_to_btc_sats(amount_payable.usd_wallet.usd, btc_sat_price)
        + amount_payable.btc_wallet.sats;
    let total_payable_amount_for_usd_wallet = usd_to_cents(amount_payable.usd_wallet.usd);

    if  total_payable_amount_for_btc_wallet > btc_wallet_balance
        || total_payable_amount_for_usd_wallet > usd_wallet_balance
    {
        return Err(PaymentError::InsufficientBalance.into());
    }
    Ok(())
}

pub fn convert_usd_to_btc_sats(
    usd_amount: Decimal,
    realtime_price: &RealtimePriceRealtimePriceBtcSatPrice,
) -> Decimal {
    let base_decimal = Decimal::from(realtime_price.base);

    let mut ten_power_offset = Decimal::from(1);
    let ten = Decimal::from(10);
    for _ in 0..realtime_price.offset {
        ten_power_offset *= ten;
    }

    let current = base_decimal / ten_power_offset;
    (Decimal::from(100) * usd_amount / current).floor()
}

pub fn usd_to_cents(usd: Decimal) -> Decimal {
    let cents = usd * Decimal::new(100, 0);
    cents.round()
}

pub fn verify_armed_records(
    armed_records: &[ListedPayment],
    skip_confirmation: bool,
) -> Result<()> {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row![
        "Username",
        "Amount",
        "Wallet Using",
        "Memo",
        "Recipient Wallet ID"
    ]);

    for record in armed_records {
        let memo_display = if let Some(ref memo) = record.memo {
            memo.clone()
        } else {
            "".to_string()
        };

        table.add_row(row![
            record.username,
            format!("{} {:?}", record.amount.to_string(), record.currency),
            format!("{:?}", record.wallet_currency),
            memo_display,
            record.recipient_wallet_id,
        ]);
    }

    println!("These are the specified payouts:\n");
    table.printstd();

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
        println!("Validating CSV format...");
        check_file_exists(&file)?;

        let me = self.client.me().await?;
        let default_wallet_id = me.default_account.default_wallet_id;
        let default_wallet = me
            .default_account
            .wallets
            .iter()
            .find(|wallet| wallet.id == default_wallet_id)
            .unwrap();

        let usd_wallets: Vec<&QueryMeMeDefaultAccountWallets> = me
            .default_account
            .wallets
            .iter()
            .filter(|wallet| Wallet::Usd == Wallet::from(&wallet.wallet_currency))
            .collect();

        let btc_wallets: Vec<&QueryMeMeDefaultAccountWallets> = me
            .default_account
            .wallets
            .iter()
            .filter(|wallet| Wallet::Btc == Wallet::from(&wallet.wallet_currency))
            .collect();

        let mut total_amount_payable = TotalAmount {
            btc_wallet: BtcWalletAmount {
                sats: Decimal::new(0, 0),
                usd: Decimal::new(0, 0),
            },
            usd_wallet: UsdWalletAmount {
                usd: Decimal::new(0, 0),
            },
        };

        let wallet_type = &default_wallet.wallet_currency;
        let reader = validate_csv(&file, &wallet_type)?;

        for record in &reader {
            let username = record
                .get(0)
                .filter(|&username| !username.is_empty())
                .ok_or_else(|| PaymentError::NoUsernameFound(record.clone()))?;

            let amount = Decimal::from_str(&record[1]).unwrap();
            let currency = &record[2];

            if currency == "SATS" && wallet_type == &WalletCurrency::BTC {
                total_amount_payable.btc_wallet.sats += amount;
            } else if currency == "USD" && wallet_type == &WalletCurrency::BTC {
                total_amount_payable.btc_wallet.usd += amount;
            } else if currency == "USD" && wallet_type == &WalletCurrency::USD {
                total_amount_payable.usd_wallet.usd += amount;
            }

            self.client
                .default_wallet(username.to_string())
                .await
                .map_err(|_| PaymentError::UsernameDoesNotExist(username.to_string()))?;
        }

        let price_response = self.client.realtime_price_usd().await;
        let btc_sat_price = price_response.unwrap().btc_sat_price;
        let btc_wallet_balance = btc_wallets[0].balance;
        let usd_wallet_balance = usd_wallets[0].balance;

        check_sufficient_balance(
            &total_amount_payable,
            &btc_sat_price,
            btc_wallet_balance,
            usd_wallet_balance,
        )?;

        // ----- Arming the records in internal structure -----
        let mut armed_records: Vec<ListedPayment> = vec![];
        for record in &reader {
            let username = record
                .get(0)
                .ok_or(PaymentError::NoUsernameFound(record.clone()))?
                .to_string();
            let recipient_wallet_id = self.client.default_wallet(username.to_string()).await?;
            let amount: Decimal = Decimal::from_str(record.get(1).unwrap_or_default())?;
            let memo = record.get(4).map(|s| s.to_string());
            let currency_str = record.get(2).unwrap_or("");
            let wallet_str = record.get(3).unwrap_or("");
     
            let currency = match currency_str {
                "SATS" => AmountCurrency::SATS,
                "USD" => AmountCurrency::USD,
                _ => return Err(PaymentError::IncorrectCSVFormat.into()), 
            };

            let wallet = match wallet_str {
                "BTC" => WalletCurrency::BTC,
                "USD" => WalletCurrency::USD,
                _ => return Err(PaymentError::IncorrectCSVFormat.into()),
            };

            armed_records.push(ListedPayment {
                username: username.clone(),
                recipient_wallet_id,
                amount,
                memo: memo.clone(),
                currency,
                wallet_currency: wallet,
            });
        }

        verify_armed_records(&armed_records, skip_confirmation)?;
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
                username,
                amount,
                currency,
                wallet_currency,
                memo,
                recipient_wallet_id,
            } = record;

            match wallet_currency {
                WalletCurrency::USD => {
                    self.client
                        .intraleger_send_usd(
                            usd_wallets[0].id.clone(),
                            recipient_wallet_id,
                            usd_to_cents(amount),
                            memo,
                        )
                        .await?;
                }
                WalletCurrency::BTC => {
                    let mut final_amount = amount;
                    if currency == AmountCurrency::USD {
                        final_amount = convert_usd_to_btc_sats(amount, &btc_sat_price);
                    }
                    self.client
                        .intraleger_send_btc(
                            btc_wallets[0].id.clone(),
                            recipient_wallet_id,
                            final_amount,
                            memo,
                        )
                        .await?;
                }
                WalletCurrency::Other(_) => {
                   return Err(PaymentError::FailedToGetWallet("Invalid Wallet Currency".to_string()).into());
                }
            }
            pb.inc(1);
            pb.println(format!("Payment sent successfully to {}!", username));
        }
        pb.finish_with_message("Batch payouts completed successfully!");
        Ok(())
    }
}