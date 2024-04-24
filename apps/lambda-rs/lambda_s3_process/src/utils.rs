pub fn parse_object_key(object_key: &String) -> String {
    let mut iter = object_key.rsplit(".").peekable();
    let end = iter.next();

    match iter.peek() {
        Some(_) => {
            let mut col = iter.collect::<Vec<_>>();

            col.reverse();
            col.join(".")
        }
        None => end.unwrap().to_owned(),
    }
}

#[cfg(test)]
mod unit_tests {
    use crate::utils::parse_object_key;

    #[test]
    fn should_parse_key_object_key() {
        let test_str = "550e8400-e29b-41d4-a716-446655440000.png".to_owned();
        let test_res = parse_object_key(&test_str);

        assert_eq!(test_res, "550e8400-e29b-41d4-a716-446655440000");

        let test_str = "550e8400-e29b-41d4-a716-446655440000".to_owned();
        let test_res = parse_object_key(&test_str);

        assert_eq!(test_res, "550e8400-e29b-41d4-a716-446655440000");

        let test_str = "550e8400-e29b-41d4-a716-446655440000.test.png".to_owned();
        let test_res = parse_object_key(&test_str);

        assert_eq!(test_res, "550e8400-e29b-41d4-a716-446655440000.test");
        assert_ne!(test_res, "test");
        assert_ne!(test_res, "png");

        let test_str = ".........test.png".to_owned();
        let test_res = parse_object_key(&test_str);

        assert_eq!(test_res, ".........test");
        assert_ne!(test_res, "test");

        let test_str = "550e8400-e29b-41d4-a716-446655440000.".to_owned();
        let test_res = parse_object_key(&test_str);

        assert_eq!(test_res, "550e8400-e29b-41d4-a716-446655440000");

        //TODO: This is an goofy edge case . probably shouldn't be taken into account.
        let test_str = ".........".to_owned();
        let test_res = parse_object_key(&test_str);

        assert_eq!(test_res, "........");
        assert_ne!(test_res, ".........");
    }
}
