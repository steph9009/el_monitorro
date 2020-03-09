#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate actix;
extern crate background_jobs;
extern crate dotenv;
extern crate failure;
extern crate futures;
extern crate mockall;
extern crate rake;
extern crate rss;
extern crate serde_derive;
extern crate sled;

use rocket_contrib::json::Json;

mod config;
mod db;
pub mod keyword_tagger;
mod models;
mod schema;
mod sync;

use keyword_tagger::*;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/keywords", data = "<text>")]
fn keywords(text: String) -> Json<Vec<Keyword>> {
    let keyword_tagger = KeywordTagger {
        text,
        stop_words: None,
    };
    let keywords = keyword_tagger.process();

    Json(keywords)
}

pub fn rocket() -> rocket::Rocket {
    rocket::custom(config::from_env())
        .mount("/api", routes![index, keywords])
        .attach(db::Conn::fairing())
}