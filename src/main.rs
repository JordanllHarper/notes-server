#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;

#[derive(rocket::serde::Deserialize, rocket::serde::Serialize)]
#[serde(crate = "rocket::serde")]
struct Note {
    id: i32,
    contents: String,
}

impl Note {
    fn new(id: i32, contents: String) -> Note {
        Note { id, contents }
    }
}

//TODO: Change this to getting list of note names
#[get("/")]
fn get_index() -> String {
    //access database
    //get list notes
    //serialize to json body
    //send
    "Hello, World!".to_string()
}

//TODO: Change this to getting note by id
#[get("/greeting/<id>")]
fn get_name(id: i32) -> Json<Note> {
    todo!()
}

#[post("/notes", format = "application/json", data = "<note>")]
fn post_index(note: Json<Note>) -> String {
    String::from("Sucessful post")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_index, post_index])
        .mount("/greeting", routes![get_name])
}
