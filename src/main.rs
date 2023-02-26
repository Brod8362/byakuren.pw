extern crate rocket;
mod ascii;
mod post;
mod rss;

use rocket::{Build, get, Rocket, routes, launch, catchers, Request, catch};
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket_dyn_templates::{context, Template};
use rocket::response::content::RawXml;

#[get("/page/<doc_name>")]
fn render_doc(doc_name: String) -> Result<Template, Status> {
    let post = post::parse_full(&doc_name)?;
   
    Ok(
        Template::render("post", context! {
            title: &post.title,
            post: &post,
            posts: post::all_min(),
        })
    )
}

#[get("/about")]
fn about_page() -> Template {
    Template::render("about", context!{
        posts: post::all_min(),
        title: "About"
    })
}

#[get("/")]
fn home_page() -> Template {
    Template::render("home", context!{
        posts: post::all_min(),
        title: "byakuren.pw"
    })
}

#[get("/rss")]
fn rss_feed() -> RawXml<String> {
    let posts = post::all_min();
    let rss = rss::generate_rss(&posts, "https://hijiri.byakuren.pw");
    RawXml(rss)
}

#[catch(default)]
fn default_catcher(status: Status, _request: &Request) -> Template {
    Template::render("error", context!{
        posts: post::all_min(),
        code: ascii::num_as_ascii(status.code as i32),
        title: status.code,
    })
}

#[launch]
pub async fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![render_doc, about_page, home_page, rss_feed])
        .mount("/static", FileServer::from("static"))
        .register("/", catchers![default_catcher])
        .attach(Template::fairing())
}