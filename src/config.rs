use std::env;

lazy_static! {
    pub static ref LISTEN_ADDR: String =
        env::var("DNSL_LISTEN").unwrap_or("127.0.0.1:53".to_owned());
    pub static ref LOOKUP_ADDR: String =
        env::var("DNSL_LOOKUP").unwrap_or("0.0.0.0:10053".to_owned());
    pub static ref UPSTREAM: String = env::var("DNSL_UPSTREAM").unwrap_or("8.8.8.8:53".to_owned());
    pub static ref ROUTE_FILE: String = env::var("DNSL_ROUTE").unwrap_or("route.yml".to_owned());
}

pub const UDP_MSG_MAX_SIZE: usize = 512;
