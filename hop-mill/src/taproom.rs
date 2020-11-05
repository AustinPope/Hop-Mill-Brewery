use rocket_contrib::templates::Template;
use serde::Serialize;

#[get("/taproom")]
pub fn taproom() -> Template {
    #[derive(Serialize)]
    struct Context {
        first_name: String,
        last_name: String,
    }

    let context = Context {
        first_name: String::from("Taproom"),
        last_name: String::from("Baby"),
    };
    Template::render("taproom", context)
}
