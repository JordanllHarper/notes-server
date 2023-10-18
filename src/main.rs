#[macro_use]
extern crate rocket;
mod endpoints;
mod models;

#[launch]
fn rocket() -> _ {
    let notes_endpoint = "/notes";

    let notes_routes = endpoints::notes::handlers::provide_routes_notes();

    rocket::build().mount(notes_endpoint, notes_routes)
}
