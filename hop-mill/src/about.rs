use rocket_contrib::templates::Template;
use serde::Serialize;

#[get("/about")]
pub fn about() -> Template {
    #[derive(Serialize)]
    struct Context {
        first_name: String,
        last_name: String,
    }

    let context = Context {
        first_name: String::from("Rust"),
        last_name: String::from("Rocket"),
    };
    Template::render("about", context)
}
