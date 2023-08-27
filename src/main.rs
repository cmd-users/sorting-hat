#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use dotenv::dotenv;
use rocket::response::content::RawHtml;
use std::env;

mod schema; pub use schema::*;
mod models; use models::*;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Error connecting to the database")
}

fn add_data() {
    use schema::teste_table::dsl::*;
    let mut conn = establish_connection();
    diesel::insert_into(teste_table)
        .default_values()
        .execute(&mut conn)
        .expect("Error inserting into teste_table");
}

fn query_data() -> Vec<Teste> {
    use schema::teste_table::dsl::*;
    let mut conn = establish_connection();
    return teste_table
        .limit(10)
        .load::<Teste>(&mut conn)
        .expect("Error loading from teste_table")
}

#[get("/add")]
fn add() -> RawHtml<String> {
    add_data();
    index()
}

#[get("/")]
fn index() -> RawHtml<String> {
    let results = query_data();
    let mut list_html = String::new();
    for item in results {
        list_html.push_str(&format!("ID: {}\n", item.id));
    }

    RawHtml(format!(r#"
        <div>
            <p>Hello, world!</p>
        </div>
        <a href="/add">Add item</a>
        <div>Query items:
            <ol>{}</ol>
        </div>
    "#, list_html))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, add])
}
