#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
mod endpoints;
mod models;

//TODO: Change this to getting note by id
#[get("/greeting/<id>")]
fn get_name(id: i32) -> Json<models::note::Note> {
    //access database
    //get note attached to id
    //if note exists, send back
    //else send error dto
    todo!()
}

#[launch]
fn rocket() -> _ {
    let notes_endpoint = "/notes";

    let notes_routes = endpoints::notes::handlers::provide_routes_notes();

    rocket::build().mount(notes_endpoint, notes_routes)
}
