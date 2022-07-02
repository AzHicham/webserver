use rocket::form::Form;
use rocket::serde::{Deserialize, Serialize};

#[get("/status")]
pub(crate) async fn world() -> &'static str {
    "ok"
}

#[post("/hello/<name>", data = "<task>")]
pub(crate) async fn hello(name: String, task: Form<Task>) -> String {
    format!("Hello, {}!", name.as_str());
    format!("Hello, {:?}!", task)
}

#[derive(FromForm, Debug, Serialize, Deserialize)]
pub struct Task {
    // #[field(validate = len(1..))]
    description: String,
    completed: bool,
}
