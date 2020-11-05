use rocket_contrib::templates::Template;
use serde::Serialize;

#[get("/tours")]
pub fn tours() -> Template {
    #[derive(Serialize)]
    struct Context {
        first_name: String,
        last_name: String,
    }

    let context = Context {
        first_name: String::from("Gimme"),
        last_name: String::from("Tours"),
    };
    Template::render("tours", context)
}
