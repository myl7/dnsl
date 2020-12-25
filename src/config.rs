use std::env;

pub struct Config {
    pub listen_addr: String,
    pub lookup_addr: String,
    pub upstream: String,
    pub route_file: String,
    pub udp_msg_max_size: usize,
}

impl Config {
    fn load() -> Self {
        dotenv::dotenv().ok();

        Self {
            listen_addr: env::var("DNSL_LISTEN").unwrap_or("127.0.0.1:53".to_owned()),
            lookup_addr: env::var("DNSL_LOOKUP").unwrap_or("0.0.0.0:10053".to_owned()),
            upstream: env::var("DNSL_UPSTREAM").unwrap_or("8.8.8.8:53".to_owned()),
            route_file: env::var("DNSL_ROUTE").unwrap_or("route".to_owned()),
            udp_msg_max_size: 512,
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::load();
}
