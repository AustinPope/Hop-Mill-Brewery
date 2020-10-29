use rocket::request::Form;

#[derive(FromForm, Debug)]
pub struct Message {
    // name: String,
    // email_address: String,
    // phone_number: String,
    #[form(field = "subject")]
    subject: String,
    #[form(field = "message")]
    message: String,
}

#[post("/", data = "<message_form>")]
pub fn new_message(message_form: Form<Message>) -> String {
    // a forward or a failure can be caught by using the Option and Result
    let message: Message = message_form.into_inner();
    let mut dummy_db: Vec<Message> = Vec::new();
    dummy_db.push(message);
    format!("Message added successfully: {:?}", dummy_db)
}
