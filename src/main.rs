extern crate rocket;
mod util;
mod ascii;

use std::fs;
use comrak::{Arena, ComrakOptions, format_html, parse_document};
use rocket::{Build, get, Rocket, routes, launch, catchers, Request, catch};
use rocket::fs::FileServer;
use rocket::http::Status;
use std::os::unix::fs::MetadataExt;
use rocket_dyn_templates::{context, Template};

#[get("/page/<doc_name>")]
fn render_doc(doc_name: String) -> Result<Template, Status> {
    let arena = Arena::new();
    let path = format!("pages/{}.md", doc_name);
    let metadata= match fs::metadata(&path) {
        Ok(m) => m,
        Err(_) => return Err(Status::NotFound)
    };
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return Err(Status::NotFound)
    };
    let root = parse_document(&arena, &*content, &ComrakOptions::default());
    let mut html = vec![];
    format_html(root, &ComrakOptions::default(), &mut html).unwrap();

    Ok(
        Template::render("post", context! {
            rendered_page: String::from_utf8(html).unwrap(),
            posts: util::all_pages(),
            title: doc_name.replace("_", " "),
            created: metadata.ctime(),
            edited: metadata.mtime(),
        })
    )
}

#[get("/about")]
fn about_page() -> Template {
    Template::render("about", context!{
        posts: util::all_pages(),
        title: "About"
    })
}

#[get("/")]
fn home_page() -> Template {
    Template::render("home", context!{
        posts: util::all_pages(),
        title: "byakuren.pw"
    })
}

#[catch(default)]
fn default_catcher(status: Status, _request: &Request) -> Template {
    Template::render("error", context!{
        posts: util::all_pages(),
        code: ascii::num_as_ascii(status.code as i32),
        title: status.code,
    })
}

#[launch]
pub async fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![render_doc, about_page, home_page])
        .mount("/static", FileServer::from("static"))
        .register("/", catchers![default_catcher])
        .attach(Template::fairing())
}