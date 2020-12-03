#![feature(
    proc_macro_hygiene,
    decl_macro,
    register_attr,
    rustc_private,
    type_ascription
)]

#[macro_use]
extern crate rocket;

extern crate ws;

use rocket::Request;
use rocket_contrib::{serve::StaticFiles, templates::Template};
use std::thread;

mod about;
mod api;
mod beer;
mod chat;
mod contact;
mod index;
mod taproom;
mod tours;

use crate::chat::ws_rs;

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

fn main() {
    // use thread to separate the chat server from Rocket's server
    thread::Builder::new()
        .name("Thread for Rust Chat with ws-rs".into())
        .spawn(|| {
            ws_rs::websocket();
        })
        .unwrap();

    rocket::ignite()
        .register(catchers![not_found])
        .mount("/static", StaticFiles::from("static/"))
        .mount(
            "/",
            routes![
                about::about,
                api::new_message,
                beer::beer,
                contact::contact,
                index::index,
                taproom::taproom,
                tours::tours,
            ],
        )
        .attach(Template::fairing())
        .launch();
}
