#[cfg(test)]
mod tests {

    #[test]
    fn test_globals() {
        use galoy_client::globals;

        let api_url = "http://localhost:4002/graphql".to_string();
        // TOOD: setup settings

        let query = globals::globals(&api_url);

        if let Err(err) = query {
            println!("{}", err);
            panic!("enum should not be None");
        }

        let r = query.unwrap();

        println!("{:?}", r);
        assert_eq!(r.nodes_ids.len(), 2)
    }

    #[test]
    fn test_default_wallet() {
        use galoy_client::default_wallet;

        let api_url = "http://localhost:4002/graphql".to_string();
        // TOOD: setup settings

        let username = "wrong username".to_string();
        let query = default_wallet::default_wallet(&api_url, &username);

        assert_eq!(query.is_err(), true);

        let username = "userA".to_string();
        let query = default_wallet::default_wallet(&api_url, &username);

        assert_eq!(query.is_err(), false)
    }
}
