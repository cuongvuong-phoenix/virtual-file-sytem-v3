use once_cell::sync::Lazy;
use std::env;

pub static ADDRESS: Lazy<String> = Lazy::new(|| {
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "7878".to_string());

    format!("{}:{}", host, port)
});

pub static ALLOWED_ORIGIN: Lazy<Vec<String>> = Lazy::new(|| get_list_env("ALLOWED_ORIGIN"));

pub static DATABASE_URL: Lazy<String> =
    Lazy::new(|| env::var("DATABASE_URL").expect("DATABASE_URL is set"));

// ----------------------------------------------------------------
// Helpers
// ----------------------------------------------------------------
fn get_list_env(key: &str) -> Vec<String> {
    let env_val = env::var(key).expect(&format!("{} is set", key));

    env_val.split(",").map(|s| s.to_string()).collect()
}
