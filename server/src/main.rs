#[allow(dead_code)]
mod cluster;
mod config;
#[allow(dead_code)]
mod layout;
mod slab;
#[allow(dead_code)]
mod tile;
mod tiles;

use self::{config::Config, tile::TileSet};
use core::{
    net::{Login, Unpacked},
    prelude::*,
};
use std::io;
use tokio::{
    io::{AsyncReadExt, BufReader},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
enum Error {
    IO(io::Error),
    Pack(core::net::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<core::net::Error> for Error {
    fn from(err: core::net::Error) -> Self {
        Self::Pack(err)
    }
}

async fn process(stream: &mut TcpStream) -> Result<Login, Error> {
    let mut reader = BufReader::with_capacity(1024, stream);
    let len = reader.read_u32().await?;
    let mut unpacked = Unpacked::new(len)?;
    reader.read_exact(unpacked.bytes()).await?;
    Ok(unpacked.to()?)
}

#[tokio::main]
async fn main() {
    let config = Config::load();
    let listener = TcpListener::bind(config.socket_addr())
        .await
        .expect("bind tcp listener");

    let addr = listener.local_addr().unwrap();
    println!("The server is listening on {}", addr);

    let tiles = TileList::new();
    let _ = TileSet::new(tiles.iter());

    loop {
        let mut stream = match listener.accept().await {
            Ok((stream, addr)) => {
                println!("Connection accepted: {}", addr);
                stream
            }
            Err(err) => {
                println!("Connection failed: {}", err);
                continue;
            }
        };

        tokio::spawn(async move {
            let login = match process(&mut stream).await {
                Ok(login) => login,
                Err(err) => panic!("Login failed: {:?}", err),
            };

            println!("Try to login: {} {}", login.name, login.pass);
        });
    }
}
