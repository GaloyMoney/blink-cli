use galoy_client::batch::Batch;
use galoy_client::GaloyClient;

use galoy_client::batch::PaymentInput;
use rust_decimal_macros::dec;

mod common;

#[test]
fn batch_csv() {
    let filename = "./tests/fixtures/example.csv".to_string();

    let galoy_client = common::unauth_client();

    let mut batch = Batch::new(galoy_client, dec!(10_000));

    batch.add_csv(filename).unwrap();
    assert_eq!(batch.len(), 2);

    assert!(batch.populate_wallet_id().is_ok());
    assert!(batch.populate_sats().is_ok());

    batch.show();
}

#[test]
fn batch_cant_pay_self() {
    let galoy_client = common::auth_client();

    let mut batch = Batch::new(galoy_client, dec!(10_000));

    batch.add(PaymentInput {
        username: "userA".to_string(),
        usd: dec!(10),
        memo: None,
    });

    assert!(batch.populate_wallet_id().is_ok());
    assert!(batch.populate_sats().is_ok());
    assert!(batch.check_balance().is_ok());
    assert!(batch.check_self_payment().is_err());
}

#[test]
fn batch_balance_too_low() {
    let galoy_client = common::auth_client();

    let mut batch = Batch::new(galoy_client, dec!(10_000));

    batch.add(PaymentInput {
        username: "userB".to_string(),
        usd: dec!(1_000_000_000),
        memo: None,
    });

    assert!(batch.populate_wallet_id().is_ok());
    assert!(batch.populate_sats().is_ok());
    assert!(batch.check_balance().is_err());
    assert!(batch.check_self_payment().is_ok());
}

#[test]
fn execute_batch() {
    let galoy_client = common::auth_client();

    let mut batch = Batch::new(galoy_client, dec!(10_000));

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
}
