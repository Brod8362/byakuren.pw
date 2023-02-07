extern crate rocket;

use rocket::{Build, get, Rocket, routes, launch};
use rocket::http::Status;
use rocket_dyn_templates::Template;

#[get("/page/<doc_name>")]
fn render_doc(doc_name: String) -> Result<Template, Status> {
    Err(Status::NotFound)
}

#[launch]
pub async fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![render_doc])
        .attach(Template::fairing())
}