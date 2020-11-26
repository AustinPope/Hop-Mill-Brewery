extern crate diesel;
extern crate serde_json;
use serde::{Serialize};
use rocket::request::Form;
use rocket::response::{Flash, Redirect};
use firestore_db_and_auth::{Credentials, ServiceSession, documents, sessions};

#[derive(FromForm, Debug, Serialize)]
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
    let serialized = serde_json::to_string(&message).unwrap();
    let cred = Credentials::from_file("Hop-Mill-250e45068a78.json").expect("Read credentials file");
    let session = ServiceSession::new(cred).expect("Create a service account session");
    let _result = documents::write(&session, "messages", Some("mVIXrAqIC7a6pWNinzj8"),
     &serialized, documents::WriteOptions::default());
    Flash::success(
        Redirect::to("/contact"),
        format!("Message successfully serialized {}", serialized)
    )
}
