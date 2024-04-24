pub fn prepared_post_key(post_id: &str) -> String {
    String::from(format!("post:create:{post_id}"))
}
