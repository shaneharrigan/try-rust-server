#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test")]
fn testing_path() -> &'static str{
    "Wow, it worked!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, testing_path])
}
