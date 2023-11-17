// main.rs

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate diesel;


use diesel::*;
use rocket::{get, post, routes, Rocket};
use rocket_db_pools::{Database, Connection};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::env;
use dotenvy::dotenv;

mod schema;
use schema::posts;

#[derive(Queryable, Serialize)]
struct Post {
    id: i32,
    title: String,
    body: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = posts)]
struct NewPost {
    title: String,
    body: String,
}

#[derive(Database)]
#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);

fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[get("/")]
fn index(conn: DbConn) -> Json<Vec<Post>> {
    let results = posts::table
        .load::<Post>(*conn)
        .expect("Error loading posts");

    Json(results)
}

#[post("/", data = "<new_post>")]
fn create(new_post: Json<NewPost>, conn: DbConn) -> Json<Post> {
    let inserted_post: Post = diesel::insert_into(posts::table)
        .values(&*new_post)
        .get_result(*conn)
        .expect("Error saving new post");

    Json(inserted_post)
}


#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![index, create])
}

fn main() {
    rocket().launch();
}
