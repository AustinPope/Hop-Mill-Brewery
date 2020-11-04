extern crate diesel;
use rocket::request::Form;
use rocket::response::{Flash, Redirect};

#[derive(FromForm, Debug)]
pub struct Message {
    #[form(field = "name")]
    name: String,
    #[form(field = "email")]
    email_address: String,
    #[form(field = "phone")]
    phone_number: String,
    #[form(field = "subject")]
    subject: String,
    #[form(field = "message")]
    message: String,
}

#[post("/contact/form", data = "<message_form>")]
pub fn new_message(message_form: Form<Message>) -> Flash<Redirect> {
    // a forward or a failure can be caught by using the Option and Result
    let message: Message = message_form.into_inner();
    if message.name.is_empty() {
        return Flash::error(Redirect::to("/contact"), "Name cannot be empty.");
    } else if message.email_address.is_empty() {
        return Flash::error(Redirect::to("/contact"), "Email Address cannot be empty.");
    } else if message.phone_number.is_empty() {
        return Flash::error(Redirect::to("/contact"), "Phone Number cannot be empty.");
    } else if message.subject.is_empty() {
        return Flash::error(Redirect::to("/contact"), "Subject cannot be empty.");
    } else if message.message.is_empty() {
        return Flash::error(Redirect::to("/contact"), "Message cannot be empty.");
    }
    let mut dummy_db: Vec<Message> = Vec::new();
    dummy_db.push(message);
    Flash::success(
        Redirect::to("/contact"),
        format!("Message added successfully: {:?}", dummy_db),
    )
}
