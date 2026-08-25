use rocket::{Build, Rocket, fs::{FileServer, NamedFile}};

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

//#[launch]
fn rocket() -> Rocket<Build> {
    println!("Starting rocket");

    rocket::build()
    .mount("/", routes![index])
    .mount("/static", FileServer::from("static"))
}

fn main() {
    println!("test")
}