use rocket::fs::{FileServer, NamedFile};

#[macro_use] extern crate rocket;

#[get("/")]
async fn index() -> NamedFile {
    NamedFile::open("templates/index.html").await.unwrap()
}

/* 
#[get("/id/<id>")]
fn api_course(id: String) -> &'static str {
  "hello"
}*/

#[launch]
fn rocket() -> _ {
    println!("Starting rocket");

    rocket::build()
    .mount("/", routes![index])
    .mount("/static", FileServer::from("static"))
}