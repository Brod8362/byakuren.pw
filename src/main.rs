extern crate rocket;
mod util;

use std::fs;
use comrak::{Arena, ComrakOptions, format_html, parse_document};
use rocket::{Build, get, Rocket, routes, launch};
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket_dyn_templates::{context, Template};

#[get("/page/<doc_name>")]
fn render_doc(doc_name: String) -> Result<Template, Status> {
    let arena = Arena::new();
    let content = match fs::read_to_string(format!("pages/{}.md", doc_name)) {
        Ok(c) => c,
        Err(_) => return Err(Status::NotFound)
    };
    let root = parse_document(&arena, &*content, &ComrakOptions::default());
    let mut html = vec![];
    format_html(root, &ComrakOptions::default(), &mut html).unwrap();

    Ok(
        Template::render("post", context! {
            rendered_page: String::from_utf8(html).unwrap(),
            posts: util::all_pages()
        })
    )
}

#[launch]
pub async fn rocket() -> Rocket<Build> {
    let pages = util::all_pages();
    println!("{:?}", pages);
    rocket::build()
        .mount("/", routes![render_doc])
        .mount("/static", FileServer::from("static"))
        .attach(Template::fairing())
}