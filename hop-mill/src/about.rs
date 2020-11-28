use rocket_contrib::templates::Template;
use serde::Serialize;

#[get("/about")]
pub fn about() -> Template {
    #[derive(Serialize)]
    struct Context {
    }

    let context = Context {
    };
    Template::render("about", context)
}
