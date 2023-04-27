#[macro_use] extern crate rocket;
use api::{ Node, get_node_list};
use rocket::serde::json::Json;

#[get("/ping")]
fn ping() -> &'static str {
    "Pong"
}

#[get("/nodes")]
fn node_list() -> Json<Vec<Node>> {
    Json(get_node_list())
}


#[launch]
fn rocket() -> _ {
    let b = rocket::build();
    b.mount("/v1", routes![ping, node_list])

}