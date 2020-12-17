extern crate diesel;
extern crate serde_json;
use firestore_db_and_auth::{documents, Credentials, ServiceSession};
use rocket::request::Form;
use rocket::response::{Flash, Redirect};
use serde::Serialize;

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
    //let cred = Credentials::from_file("Hop-Mill-250e45068a78.json").expect("Read credentials file");
    //let session = ServiceSession::new(cred).expect("Create a service account session");
    //let _result = documents::write(&session, "messages", Some("mVIXrAqIC7a6pWNinzj8"), &message, documents::WriteOptions::default());
    let mut dummy_db: Vec<Message> = Vec::new();
    dummy_db.push(message);
    Flash::success(
        Redirect::to("/contact"),
        //format!("Thank you {} for your feedback!", message.name)
        format!("Message added successfully: {:?}", dummy_db),
    )
}
