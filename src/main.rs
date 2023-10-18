#[macro_use]
extern crate rocket;

#[get("/")]
fn get_index() -> String {
    "Hello, World!".to_string()
}

#[get("/greeting/<name>")]
fn get_name(name: &str) -> String {
    format!("Hello, {}", name)
}

#[post("/")]
fn post_index() -> String {
    String::from("Sucessful post")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_index, post_index])
        .mount("/greeting", routes![get_name])
}
