use dotenv::dotenv;
pub struct Config {
    pub token: String,
}

impl Config {
    pub fn new() -> Config {
        dotenv().ok();
        let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
        Config { token: token }
    }
}