#[cfg(test)]
mod tests {
    use galoy_client::globals;

    #[test]
    fn test_globals() {
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
}
