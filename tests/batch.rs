use common::GaloyConfig;
use galoy_client::batch::Batch;
use galoy_client::GaloyClient;

use galoy_client::batch::PaymentInput;
use rust_decimal_macros::dec;

mod common;

fn load_client_config() -> GaloyConfig {
    let phone = std::env::var("PHONE_NUMBER").expect("Missing phone number");
    let code = std::env::var("AUTH_CODE").expect("Missing auth code");

    GaloyConfig { phone, code }
}

#[test]
fn batch_csv() -> anyhow::Result<()> {
    let filename = "./tests/fixtures/example.csv".to_string();

    let galoy_client = common::unauth_client();

    let mut batch = Batch::new(galoy_client, dec!(10_000))?;

    batch.add_csv(filename)?;
    assert_eq!(batch.len(), 2);

    assert!(batch.populate_wallet_id().is_ok());
    assert!(batch.populate_sats().is_ok());

    batch.show();

    Ok(())
}

#[test]
fn batch_cant_pay_self() -> anyhow::Result<()> {
    let config = load_client_config();
    let galoy_client = common::auth_client(config);

    let mut batch = Batch::new(galoy_client, dec!(10_000))?;

    batch.add(PaymentInput {
        username: "userA".to_string(),
        usd: dec!(10),
        memo: None,
    });

    assert!(batch.populate_wallet_id().is_ok());
    assert!(batch.populate_sats().is_ok());
    assert!(batch.check_balance().is_ok());
    assert!(batch.check_self_payment().is_err());

    Ok(())
}

#[test]
fn batch_balance_too_low() -> anyhow::Result<()> {
    let config = load_client_config();
    let galoy_client = common::auth_client(config);

    let mut batch = Batch::new(galoy_client, dec!(10_000))?;

    batch.add(PaymentInput {
        username: "userB".to_string(),
        usd: dec!(1_000_000_000),
        memo: None,
    });

    assert!(batch.populate_wallet_id().is_ok());
    assert!(batch.populate_sats().is_ok());
    assert!(batch.check_balance().is_err());
    assert!(batch.check_self_payment().is_ok());

    Ok(())
}

#[test]
fn execute_batch() -> anyhow::Result<()> {
    let config = load_client_config();
    let galoy_client = common::auth_client(config);

    let mut batch = Batch::new(galoy_client, dec!(10_000))?;

    batch.add(PaymentInput {
        username: "userB".to_string(),
        usd: dec!(2),
        memo: None,
    });
    batch.add(PaymentInput {
        username: "userB".to_string(),
        usd: dec!(5),
        memo: Some("memo for second batch tx".to_string()),
    });

    assert!(batch.populate_wallet_id().is_ok());
    assert!(batch.populate_sats().is_ok());
    assert!(batch.check_balance().is_ok());
    assert!(batch.check_self_payment().is_ok());

    let result = batch.execute().expect("didn't complete batch successfully");
    println!("{:?}", result);

    // TODO: check balance and transactions

    Ok(())
}
