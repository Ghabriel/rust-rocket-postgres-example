#![feature(proc_macro_hygiene, decl_macro)]

use postgres::{Client, NoTls};

use rocket::{get, routes};

use rocket_contrib::json::Json;

use serde::{Deserialize, Serialize};

const DATABASE_HOST: &str = "0.0.0.0";
const DATABASE_USER: &str = "postgres";
const DATABASE_PASSWORD: &str = "safepass";
const DATABASE_NAME: &str = "pudim";

#[derive(Deserialize, Serialize)]
struct Network {
    id: i32,
    name: String,
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/networks")]
fn networks() -> Option<Json<Vec<Network>>> {
    let mut client = Client::connect(
        &format!(
            "host={} dbname={} user={} password={}",
            DATABASE_HOST,
            DATABASE_NAME,
            DATABASE_USER,
            DATABASE_PASSWORD,
        ),
        NoTls,
    )
        .unwrap();
    let mut result = Vec::new();

    for row in client.query("SELECT id, name FROM networks", &[]).unwrap() {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);

        result.push(Network {
            id,
            name: name.into(),
        });
    }

    Some(Json(result))
}

fn main() {
    rocket::ignite().mount("/", routes![hello, networks]).launch();
}
