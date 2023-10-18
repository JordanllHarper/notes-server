use rocket::Route;

pub fn provide_routes_notes() -> Vec<Route> {
    routes![get_index, post_index]
}

//TODO: Change this to getting list of note names
#[get("/notes")]
pub fn get_index() -> String {
    //
    //access database
    //
    //get list notes
    //send list
    "Hello, World!".to_string()
}

#[post("/notes", format = "application/json", data = "<note>")]
pub fn post_index(note: rocket::serde::json::Json<crate::models::note::Note>) -> String {
    String::from("Sucessful post")
}
