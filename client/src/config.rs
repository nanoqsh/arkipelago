use serde::Deserialize;
use std::{fs::File, io::Read, net::SocketAddr};

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
        let mut file = match File::open(PATH) {
            Ok(file) => file,
            Err(err) => panic!("open config file {}: {}", PATH, err),
        };

        let mut buf = String::new();
        file.read_to_string(&mut buf).expect("read config");
        toml::from_str(&buf).expect("parse config")
    }

    pub fn socket_addr(&self) -> SocketAddr {
        format!("{}:{}", self.tcp.host, self.tcp.port)
            .parse()
            .unwrap()
    }
}
