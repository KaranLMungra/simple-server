#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, routes, State};
use std::sync::Mutex;

struct Users {
    users: Mutex<Vec<String>>,
}

#[get("/greet/<name>")]
fn greet(name: String, config: State<Users>) -> String {
    let mut users = config.users.lock().unwrap();
    users.push(name);
    format!("Hello, {:?}!", users.clone())
}

fn main() {
    let users = Users {
        users: Mutex::new(Vec::new()),
    };
    rocket::ignite()
        .manage(users)
        .mount("/", routes![greet])
        .launch();
}
