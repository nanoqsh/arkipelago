use serde::Deserialize;
use std::net::SocketAddr;

const PATH: &str = "Client.toml";

#[derive(Deserialize)]
struct Tcp {
    host: String,
    port: u16,
}

#[derive(Deserialize)]
pub struct Config {
    tcp: Tcp,
}

impl Config {
    pub fn load() -> Self {
        let content = std::fs::read_to_string(PATH).expect("read config");
        toml::from_str(&content).expect("parse config")
    }

    pub fn socket_addr(&self) -> SocketAddr {
        format!("{}:{}", self.tcp.host, self.tcp.port)
            .parse()
            .unwrap()
    }
}
