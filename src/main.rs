#[macro_use]
extern crate rocket;

use askama::Template;
use rocket::fs::FileServer;
use rocket::response::content::RawHtml;

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate<'a> {
    name: &'a str,
}

#[derive(Template)]
#[template(path = "content.html")]
struct ContentTemplate<'a> {
    name: &'a str,
}

#[rocket::get("/")]
fn home() -> Result<RawHtml<String>, RawHtml<String>> {
    let response = RawHtml(HomeTemplate { name: "world" }.to_string());
    Ok(response)
}

#[rocket::get("/note")]
fn content() -> Result<RawHtml<String>, RawHtml<String>> {
    let response = RawHtml(ContentTemplate { name: "world" }.to_string());
    Ok(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![home])
        .mount("/", routes![content])
        .mount("/public", FileServer::from("public/"))
}
