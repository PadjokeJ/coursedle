use rocket::fs::FileServer;
use rocket_dyn_templates::{Template, context};

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}
/* 
#[get("/id/<id>")]
fn api_course(id: String) -> &'static str {
  "hello"
}*/

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/static", FileServer::from("/static"))
}