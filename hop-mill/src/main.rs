#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::Request;
use rocket_contrib::{serve::StaticFiles, templates::Template};

mod about;
mod api;
mod contact;
mod index;

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .mount("/static", StaticFiles::from("static/"))
        .mount(
            "/",
            routes![
                about::about,
                api::new_message,
                contact::contact,
                index::index
            ],
        )
        .attach(Template::fairing())
        .launch();
}
