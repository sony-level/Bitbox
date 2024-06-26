use dotenv::dotenv;
use std::env;

pub fn init_env() {
    dotenv().ok();
    env_logger::init();
}

pub fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}
