use serde::Deserialize;
use tokio::net::ToSocketAddrs;

const PATH: &str = "Server.toml";

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

    pub fn socket_addr(&self) -> impl ToSocketAddrs + '_ {
        (self.tcp.host.as_str(), self.tcp.port)
    }
}
