use rocket_contrib::templates::Template;
use serde::Serialize;

#[get("/contact")]
pub fn contact() -> Template {
    #[derive(Serialize)]
    struct Context {
        // fields
    }
    let context = Context {
        // give fields values
    };
    Template::render("contact", context)
}
