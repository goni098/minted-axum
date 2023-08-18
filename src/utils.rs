pub fn refresh_token_generate(user_id: i32) -> String {
  format!("refresh_token_{}", user_id)
}
