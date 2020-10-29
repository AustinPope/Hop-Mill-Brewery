use rocket::request::FlashMessage;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[get("/contact")]
pub fn contact(msg: Option<FlashMessage>) -> Template {
    #[derive(Serialize)]
    struct Context<'a, 'b> {
        msg: Option<(&'a str, &'b str)>,
    }
    impl<'a, 'b> Context<'a, 'b> {
        pub fn raw(msg: Option<(&'a str, &'b str)>) -> Context<'a, 'b> {
            Context { msg: msg }
        }
    }
    Template::render(
        "contact",
        &match msg {
            Some(ref msg) => Context::raw(Some((msg.name(), msg.msg()))),
            None => Context::raw(None),
        },
    )
}
