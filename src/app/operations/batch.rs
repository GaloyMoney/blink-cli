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
            QueryMeMe, RealtimePriceRealtimePriceBtcSatPrice,
        },
        types::AmountCurrency,
    },
};

impl QueryMeMe {
    pub fn get_btc_wallet(&self) -> Option<&QueryMeMeDefaultAccountWallets> {
        self.default_account
            .wallets
            .iter()
            .find(|wallet| wallet.wallet_currency == WalletCurrency::BTC)
    }

    pub fn get_usd_wallet(&self) -> Option<&QueryMeMeDefaultAccountWallets> {
        self.default_account
            .wallets
            .iter()
            .find(|wallet| wallet.wallet_currency == WalletCurrency::USD)
    }

    pub fn get_btc_wallet_balance(&self) -> Option<Decimal> {
        self.default_account
            .wallets
            .iter()
            .find(|wallet| wallet.wallet_currency == WalletCurrency::BTC)
            .map(|wallet| wallet.balance)
    }

    pub fn get_usd_wallet_balance(&self) -> Option<Decimal> {
        self.default_account
            .wallets
            .iter()
            .find(|wallet| wallet.wallet_currency == WalletCurrency::USD)
            .map(|wallet| wallet.balance)
    }

    pub fn get_btc_wallet_id(&self) -> Option<String> {
        self.get_btc_wallet().map(|wallet| wallet.id.clone())
    }

    pub fn get_usd_wallet_id(&self) -> Option<String> {
        self.get_usd_wallet().map(|wallet| wallet.id.clone())
    }

    pub fn get_default_wallet_currency(&self) -> Option<&WalletCurrency> {
        let default_wallet_id = &self.default_account.default_wallet_id;
        self.default_account
            .wallets
            .iter()
            .find(|wallet| &wallet.id == default_wallet_id)
            .map(|wallet| &wallet.wallet_currency)
    }
}

const USERNAME_IDX: usize = 0;
const AMOUNT_IDX: usize = 1;
const CURRENCY_IDX: usize = 2;
const WALLET_IDX: usize = 3;
const MEMO_IDX: usize = 4;

const CSV_HEADERS: [&str; 5] = ["username", "amount", "currency", "wallet", "memo"];

pub struct ListedPayment {
    pub username: String,
    pub amount: Decimal,
    pub currency: AmountCurrency,
    pub wallet_currency: WalletCurrency,
    pub memo: Option<String>,
    pub recipient_wallet_id: Option<String>,
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
) -> Result<Vec<ListedPayment>, PaymentError> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_path(file)
        .map_err(|_| PaymentError::FailedToReadCSV)?;

    let headers = reader
        .headers()
        .map_err(|_| PaymentError::FailedToGetHeaders)?;

    if headers.iter().zip(CSV_HEADERS.iter()).any(|(a, &b)| a != b) {
        return Err(PaymentError::IncorrectCSVFormat);
    }

    reader
        .records()
        .map(|result| {
            let record = result.map_err(|_| PaymentError::FailedToGetRecords)?;
            let username = record
                .get(USERNAME_IDX)
                .ok_or(PaymentError::IncorrectCSVFormat)?
                .trim();
            let amount_str = record
                .get(AMOUNT_IDX)
                .ok_or(PaymentError::IncorrectCSVFormat)?
                .trim();
            let currency = record
                .get(CURRENCY_IDX)
                .ok_or(PaymentError::IncorrectCSVFormat)?
                .trim();

            // required fields
            if username.is_empty() || amount_str.is_empty() || currency.is_empty() {
                return Err(PaymentError::IncorrectCSVFormat);
            }

            // if wallet is not given use default wallet, optional fields ie wallet and memo
            let wallet_currency =
                record
                    .get(WALLET_IDX)
                    .map_or(default_wallet.to_str(), |wallet| {
                        if wallet.is_empty() {
                            default_wallet.to_str()
                        } else {
                            wallet
                        }
                    });
            let memo = record.get(MEMO_IDX).map(|s| s.to_string());

            let amount =
                Decimal::from_str(amount_str).map_err(|_| PaymentError::IncorrectCSVFormat)?;
            //amount should be greater than 0
            if amount <= Decimal::new(0, 0) {
                return Err(PaymentError::IncorrectCSVFormat);
            }

            let currency = match currency {
                "SATS" => AmountCurrency::SATS,
                "USD" => AmountCurrency::USD,
                _ => return Err(PaymentError::IncorrectCSVFormat),
            };

            let wallet_currency = match wallet_currency {
                "BTC" => WalletCurrency::BTC,
                "USD" => WalletCurrency::USD,
                _ => return Err(PaymentError::IncorrectCSVFormat),
            };

            //amount for SATS will be whole number and for USD max 2 decimals
            let formatted_amount = match currency {
                AmountCurrency::SATS => amount.round_dp(0),
                AmountCurrency::USD => {
                    amount.round_dp_with_strategy(2, RoundingStrategy::MidpointAwayFromZero)
                }
            };

            //if currency is SATS then wallet should be BTC
            if currency == AmountCurrency::SATS && wallet_currency != WalletCurrency::BTC {
                return Err(PaymentError::IncorrectCSVFormat);
            }

            Ok(ListedPayment {
                username: username.to_string(),
                amount: formatted_amount,
                currency,
                wallet_currency,
                memo,
                recipient_wallet_id: None,
            })
        })
        .collect()
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

    if total_payable_amount_for_btc_wallet > btc_wallet_balance
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
        let memo_display = record.memo.clone().unwrap_or_default();
        let recipient_wallet_id_display = if let Some(ref id) = record.recipient_wallet_id {
            id.clone()
        } else {
            "Not Set".to_string()
        };

        table.add_row(row![
            record.username,
            format!("{} {:?}", record.amount.to_string(), record.currency),
            format!("{:?}", record.wallet_currency),
            memo_display,
            recipient_wallet_id_display,
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
        let default_wallet_currency = me.get_default_wallet_currency().ok_or_else(|| {
            PaymentError::FailedToGetWallet("Default Wallet not found".to_string())
        })?;

        let usd_wallets_id = me
            .get_usd_wallet_id()
            .ok_or_else(|| PaymentError::FailedToGetWallet("USD Wallet not found".to_string()))?;

        let btc_wallets_id = me
            .get_btc_wallet_id()
            .ok_or_else(|| PaymentError::FailedToGetWallet("BTC Wallet not found".to_string()))?;

        let mut total_amount_payable = TotalAmount {
            btc_wallet: BtcWalletAmount {
                sats: Decimal::new(0, 0),
                usd: Decimal::new(0, 0),
            },
            usd_wallet: UsdWalletAmount {
                usd: Decimal::new(0, 0),
            },
        };

        let mut reader = validate_csv(&file, default_wallet_currency)?;

        for record in &reader {
            let amount = &record.amount;
            let currency = &record.currency;
            let wallet_type = &record.wallet_currency;

            if currency == &AmountCurrency::SATS && wallet_type == &WalletCurrency::BTC {
                total_amount_payable.btc_wallet.sats += amount;
            } else if currency == &AmountCurrency::USD && wallet_type == &WalletCurrency::BTC {
                total_amount_payable.btc_wallet.usd += amount;
            } else if currency == &AmountCurrency::USD && wallet_type == &WalletCurrency::USD {
                total_amount_payable.usd_wallet.usd += amount;
            }

            // Make sure username exists
            self.client
                .default_wallet(record.username.clone())
                .await
                .map_err(|_| PaymentError::UsernameDoesNotExist(record.username.clone()))?;
        }

        let price_response = self.client.realtime_price_usd().await;
        let btc_sat_price = price_response.unwrap().btc_sat_price;

        let btc_wallet_balance = me.get_btc_wallet_balance().ok_or_else(|| {
            PaymentError::FailedToGetWallet("BTC Wallet balance not found".to_string())
        })?;
        let usd_wallet_balance = me.get_usd_wallet_balance().ok_or_else(|| {
            PaymentError::FailedToGetWallet("USD Wallet balance not found".to_string())
        })?;

        check_sufficient_balance(
            &total_amount_payable,
            &btc_sat_price,
            btc_wallet_balance,
            usd_wallet_balance,
        )?;

        for record in reader.iter_mut() {
            let recipient_wallet_id = self.client.default_wallet(record.username.clone()).await?;
            record.recipient_wallet_id = Some(recipient_wallet_id);
        }

        verify_armed_records(&reader, skip_confirmation)?;

        let total_size: u64 = reader.len().try_into().unwrap();
        let pb = ProgressBar::new(total_size);

        pb.enable_steady_tick(std::time::Duration::from_millis(10));
        pb.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>5}/{len:5} {msg}",
            )
            .unwrap()
            .progress_chars("=> "),
        );

        for record in reader.into_iter() {
            let ListedPayment {
                username,
                amount,
                currency,
                wallet_currency,
                memo,
                recipient_wallet_id,
            } = record;

            let recipient_wallet_id_check = match recipient_wallet_id {
                Some(id) => id,
                None => {
                    return Err(PaymentError::FailedToGetWallet(
                        "Recipient Wallet ID not found".to_string(),
                    )
                    .into());
                }
            };

            match wallet_currency {
                WalletCurrency::USD => {
                    self.client
                        .intraleger_send_usd(
                            usd_wallets_id.clone(),
                            recipient_wallet_id_check,
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
                            btc_wallets_id.clone(),
                            recipient_wallet_id_check,
                            final_amount,
                            memo,
                        )
                        .await?;
                }
                WalletCurrency::Other(_) => {
                    return Err(PaymentError::FailedToGetWallet(
                        "Invalid Wallet Currency".to_string(),
                    )
                    .into());
                }
            }
            pb.inc(1);
            pb.println(format!("Payment sent successfully to {}!", username));
        }
        pb.finish_with_message("Batch payouts completed successfully!");
        Ok(())
    }
}
