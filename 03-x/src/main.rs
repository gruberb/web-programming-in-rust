#![feature(async_await, futures_api, await_macro)]

#[macro_use]
extern crate serde_derive;

use tide::{error::ResultExt, response, App, Context, EndpointResult};
use http::status::StatusCode;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
struct User {
    name: String,
    height: u32,
}

#[derive(Default)]
struct Database {
    contents: Mutex<Vec<User>>,
}

impl Database {
    fn insert(&self, user: User) -> usize {
        let mut table = self.contents.lock().unwrap();
        table.push(user);
        table.len() - 1
    }

    fn get_all(&self) -> Vec<User> {
        self.contents.lock().unwrap().clone()
    }

    fn get(&self, id: usize) -> Option<User> {
        self.contents.lock().unwrap().get(id).cloned()
    }

    fn set(&self, id: usize, user: User) -> bool {
        let mut table = self.contents.lock().unwrap();

        if let Some(old_user) = table.get_mut(id) {
            *old_user = user;
            true
        } else {
            false
        }
    }

    fn delete(&self, id: usize) -> bool {
        let mut table = self.contents.lock().unwrap();

        if let Some(_user) = table.get_mut(id) {
            self.contents.lock().unwrap().remove(id);
            true
        } else {
            false
        }
    }
}

async fn handle_get_users(cx: Context<Database>) -> EndpointResult {
    Ok(response::json(cx.app_data().get_all()))
}

async fn handle_get_user(cx: Context<Database>) -> EndpointResult {
    let id = cx.param("id").client_err()?;
    if let Some(user) = cx.app_data().get(id) {
        Ok(response::json(user))
    } else {
        Err(StatusCode::NOT_FOUND)?
    }
}

async fn handle_update_user(mut cx: Context<Database>) -> EndpointResult<()> {
    let user = await!(cx.body_json()).client_err()?;
    let id = cx.param("id").client_err()?;

    if cx.app_data().set(id, user) {
        Ok(())
    } else {
        Err(StatusCode::NOT_FOUND)?
    }
}

async fn handle_create_user(mut cx: Context<Database>) -> EndpointResult<String> {
    let user = await!(cx.body_json()).client_err()?;
    Ok(cx.app_data().insert(user).to_string())
}

async fn handle_delete_user(cx: Context<Database>) -> EndpointResult<String> {
    let id = cx.param("id").client_err()?;
    Ok(cx.app_data().delete(id).to_string())
}

fn main() {
    let mut app = App::new(Database::default());
    app.at("/users")
        .post(handle_create_user)
        .get(handle_get_users);
    app.at("/users/:id")
        .get(handle_get_user)
        .patch(handle_update_user)
        .delete(handle_delete_user);

    app.serve("127.0.0.1:8000").unwrap();
}