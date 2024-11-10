use lazy_static::lazy_static;
use std::env;


lazy_static! {
    pub static ref DISCORD_TOKEN: String = get_discord_token();
    pub static ref SHUTTLE_API_KEY: String = get_shuttle_key();
}

fn get_discord_token() -> String {
    dotenv::dotenv().ok();
    env::var("DISCORD_TOKEN").unwrap_or_else(|_| "foo".to_string())
}

fn get_shuttle_key() -> String {
    dotenv::dotenv().ok();
    env::var("SHUTTLE_API_KEY").unwrap_or_else(|_| "foo".to_string())
}