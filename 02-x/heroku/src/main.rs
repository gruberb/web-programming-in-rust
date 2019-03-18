#![feature(async_await)]

// to be able to read environment variables
use std::env;
// to be able to pass a different base configuration to our app
use tide::{configuration::Configuration};

// we need to read the PORT from the env variable (Heroku sets it)
fn get_server_port() -> u16 {
    env::var("PORT")
    .ok()
    .and_then(|p| p.parse().ok())
    .unwrap_or(8181)
}
fn main() {
    let mut app = tide::App::new(());
    let app_config = Configuration::build()
        .address(String::from("0.0.0.0"))
        .port(get_server_port())
        .finalize();
    app.config(app_config);
    app.at("/").get(async || "Hello, World!");
    app.serve();
}