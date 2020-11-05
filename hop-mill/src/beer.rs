use rocket_contrib::templates::Template;
use serde::Serialize;

#[get("/beer")]
pub fn beer() -> Template {
    #[derive(Serialize)]
    struct Context {
        first_name: String,
        last_name: String,
    }

    let context = Context {
        first_name: String::from("Beers"),
        last_name: String::from("Please"),
    };
    Template::render("beer", context)
}
