mod env;

pub use env::*;

pub fn load_env() {
    let profile = if cfg!(test) {
        "test"
    } else if cfg!(debug_assertions) {
        "development"
    } else {
        "production"
    };

    dotenv::from_filename(format!(".env.{}.local", profile)).ok();
    dotenv::from_filename(format!(".env.local")).ok();
    dotenv::from_filename(format!(".env.{}", profile)).ok();
    dotenv::dotenv().ok();
}
