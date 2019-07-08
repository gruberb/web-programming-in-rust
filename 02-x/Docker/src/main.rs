#![feature(async_await)]

extern crate tide;

use tide::App;
use std::{env, net::SocketAddr};

fn main() {
    let mut app = App::new(());
    let address = SocketAddr::from(([127, 0, 0, 1], get_server_port()));

    app.at("/").get(async move |_| "hello world");
    app.serve(address).expect("Start server");
}

fn get_server_port() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|port| port.parse().ok())
        .unwrap_or_else(|| 8186)
}

