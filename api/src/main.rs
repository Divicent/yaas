#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/abc")]
fn abc() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let b = rocket::build();
    b.mount("/", routes![index])
    .mount("/abc", routes![abc])
}