use db::establish_connection;

#[macro_use]
extern crate diesel;

pub mod auth;
pub mod commands;
pub mod config;
pub mod db;
pub mod ui;

fn main() {
    let _conn = establish_connection();

    let mut _application_state = config::AppState {
        is_authenticated: false,
        user_id: None,
    };
}
