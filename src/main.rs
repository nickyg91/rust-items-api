#![feature(plugin)]
#![feature(type_ascription)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate postgres;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use postgres::{Connection, TlsMode};
use std::collections::LinkedList;
use rocket_contrib::{Json, Value};
use rocket::response::content;

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub item_id: i64,
    pub item_name: String,
    pub sku: String
}

#[get("/items")]
fn all_items() -> Json<LinkedList<Item>> {
    let mut list: LinkedList<Item> = LinkedList::new();
    let sql = String::from("SELECT * FROM store_items");
    let conn = Connection::connect("postgres://rust:Rust@localhost/rust", TlsMode::None).unwrap();
    for row in &conn.query(&sql, &[]).unwrap(){
        let item = Item{
            item_id: row.get(0),
            item_name: row.get(1),
            sku: row.get(2)
        };
        list.push_back(item);
    }
    Json(list)
}

fn main(){
    rocket::ignite().mount("/", routes![all_items]).launch();
}
