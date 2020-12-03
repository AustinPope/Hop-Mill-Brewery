use rocket_contrib::templates::Template;
use serde::Serialize;

#[get("/tours")]
pub fn tours() -> Template {
    #[derive(Serialize)]
    struct Context {
    }

    let context = Context {
    };
    Template::render("tours", context)
}
