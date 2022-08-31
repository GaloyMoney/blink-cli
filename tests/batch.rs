use galoy_client::batch::Batch;
use galoy_client::GaloyClient;
use std::path::PathBuf;

use rust_decimal_macros::dec;

#[test]
fn batch_csv() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let filename = format!("{}/tests/example.csv", root.display());

    let api = "http://localhost:4002/graphql".to_string();
    let galoy_client = GaloyClient::new(api, None);

    let mut batch = Batch::new(galoy_client, dec!(10000));

    batch.add_csv(filename).unwrap();
    assert_eq!(batch.len(), 1);

    assert!(batch.populate_wallet_id().is_ok());
    assert!(batch.populate_sats().is_ok());

    batch.show();
}

#[test]
fn batch_is_ready() {
    let api = "http://localhost:4002/graphql".to_string();
    let galoy_client = GaloyClient::new(api.clone(), None);

    let phone = "+16505554321".to_string();
    let code = "321321".to_string();

    let jwt = galoy_client
        .user_login(phone, code)
        .expect("request should succeed");

    let galoy_client = GaloyClient::new(api, Some(jwt));

    let mut batch = Batch::new(galoy_client, dec!(10000));
    batch.add("userB".to_string(), dec!(10));

    assert!(batch.populate_wallet_id().is_ok());
    assert!(batch.populate_sats().is_ok());
    assert!(batch.check_self_payment().is_ok());
    assert!(batch.check_self_payment().is_ok());
}

#[test]
fn batch_cant_pay_self() {
    let api = "http://localhost:4002/graphql".to_string();
    let galoy_client = GaloyClient::new(api.clone(), None);

    let phone = "+16505554321".to_string();
    let code = "321321".to_string();

    let jwt = galoy_client
        .user_login(phone, code)
        .expect("request should succeed");

    let galoy_client = GaloyClient::new(api, Some(jwt));

    let mut batch = Batch::new(galoy_client, dec!(10000));
    batch.add("userA".to_string(), dec!(10));

    assert!(batch.populate_wallet_id().is_ok());
    assert!(batch.populate_sats().is_ok());
    assert!(batch.check_balance().is_ok());
    assert!(batch.check_self_payment().is_err());
}

#[test]
fn batch_balance_too_low() {
    let api = "http://localhost:4002/graphql".to_string();
    let galoy_client = GaloyClient::new(api.clone(), None);

    let phone = "+16505554321".to_string();
    let code = "321321".to_string();

    let jwt = galoy_client
        .user_login(phone, code)
        .expect("request should succeed");

    let galoy_client = GaloyClient::new(api, Some(jwt));

    let mut batch = Batch::new(galoy_client, dec!(10000));
    batch.add("userB".to_string(), dec!(1_000_000_000));

    assert!(batch.populate_wallet_id().is_ok());
    assert!(batch.populate_sats().is_ok());
    assert!(batch.check_balance().is_err());
    assert!(batch.check_self_payment().is_ok());
}
