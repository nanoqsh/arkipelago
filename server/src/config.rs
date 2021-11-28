use serde::Deserialize;
use std::{fs::File, io::Read};
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
        let mut file = match File::open(PATH) {
            Ok(file) => file,
            Err(err) => panic!("open config file {}: {}", PATH, err),
        };

        let mut buf = String::new();
        file.read_to_string(&mut buf).expect("read config");
        toml::from_str(&buf).expect("parse config")
    }

    pub fn socket_addr(&self) -> impl ToSocketAddrs + '_ {
        (self.tcp.host.as_str(), self.tcp.port)
    }
}
