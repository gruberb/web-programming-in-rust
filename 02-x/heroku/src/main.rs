#![feature(async_await)]

use std::env;
use tide::{configuration::Configuration};

fn get_server_port() -> u16 {
    env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8181)
}

fn main() {
    let mut app = tide::App::new(());
       let app_config = Configuration::build()
        .address(String::from("0.0.0.0"))
        .port(get_server_port())
        .finalize();

    app.config(app_config);

    app.at("/").get(async || "Hello, world!");

    app.serve();
}